# RocksDB WritePath概览

简单过一遍RocksDB前台的写链路。

忽略这些，从下面的第一张写链路图开始看。。

1. DB::Put
   1. 没给CF的话会用DefaultCF
   2. 构造一个WriteBatch，把key，value，以及column family塞进去
   3. 这里有个Comparator的timestamp size，还不知道是干啥的
2. DBImpl::Write
   1. 先走到没有Callback的版本
   2. 然后有一堆InvalidArg的检查。disable memtable先不看，pipelined write先不看
   3. 构造一个Writer
   4. `write_thread_.JoinBatchGroup`
      1. 这里用来进行GroupCommit。有一个atomic指针代表最新的writer，如果是`write_stall_dummy_`的话，说明发生WriteStall，会等待在`stall_cv_`中。
      2. 然后会尝试把自己CAS链表头中。并且如果在自己是最后一个元素的时候，返回true。
      3. 返回true代表自己是第一个Writer，也代表了是Leader。这里会调`SetState(STATE_GROUP_LEADER)`，将状态设置为Leader。
      4. 如果本次没有成为Leader的话，这里会等待直到：
         1. 当前线程成为新的Leader
         2. 当前线程成为Follower，然后Leader帮助我们写完了MemTable，或者需要我们自己去写MemTable
         3. pipeline write相关，暂时不看
         4. 这里AwaitState写的比较细。会划分fast path和slow path，之后再细研究下。
   5. GroupCommit之后。先看Leader逻辑。
      1. PreprocessWrite
         1. 看到了个有意思的`InstrumentedMutexLock`，是一个锁的Wrapper，可以用来统计持锁时间。
         2. 这里会先判断如果total log size超过了`GetMaxTotalWalSize()`，并且如果CF的数量大于1的时候，会调用`WaitForPendingWrites()` + `SwitchWAL()`。简单看了看大概意思是等其他的写者写完，然后切一个WAL来写。下来再细看。
         3. 然后如果需要刷MemTable的时候，这里也会先通过`WaitForPendingWrites()`排空写者，然后切一个新的MemTable出来。
         4. `TrimMemtableHistory()`，下来再看
         5. 如果MemTable满了，调`ScheduleFlushes()`，还不太清楚和上面的`HandleWriteBufferManagerFlush()`有什么关联。
         6. `WriteController`用来控制写入速度
         7. 然后有一坨log sync的逻辑，暂时没看懂，等下来看。。

看到一个非常好的图，贴在这里

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230405151152.png)

按照这个图捋一遍：

写流程的主链路在`db_impl_write.cc::WriteImpl`中

每一个写入请求都会对应一个Writer。里面记录了本次写入需要的信息。包括：

* 本次WriteBatch
* 一些option，比如是否关闭WAL，是否同步落盘，callback等。

`DBImpl`中有一个`WriteThread`，用来协调写入。里面一个比较关键的结构就是一个`std::atomic<Writer*>`，用来指向最新的Writer。其实是指向了由Writer组成的链表表头，每个线程可以并发的向链表中插入Writer，但是只有Leader才能从链表中pop元素。

构造好Writer后，调用`WriteThread::JoinBatchGroup`将自己加入到一个WriteGroup中。

核心逻辑是把当前Writer加入到上面说的链表中，并且当自己作为链表中的第一个元素出现的时候，会认为自己是Leader。

如果不是Leader的话，会调`AwaitState`等待Leader唤醒。

Leader会调`DBImpl::PreprocessWrite`，会做一些DelayWrite，处理WriteStall，切MemTable的逻辑。

`WriteThread::EnterAsBatchGroupLeader`。记录本次WriteGroup的Leader为当前的Writer。

`CreateMissingNewerLinks`把刚才由Writer组成的单链表连成双向链表。

然后会从Leader开始遍历，不断将链表中的Writer加入到本次WriteGroup中。当遇到配置出现冲突的时候，会停止添加新的Writer，或者当本次WriteBatch过大的时候也会停止添加新的Writer。

组成WriteGroup后，这里会遍历一遍本次WriteBatch，看是否允许并行写MemTable。条件判断有：MemTable是否允许并行写，Put不能是Inplace写入，不能有Merge操作。

然后计算一下本次应该增加多少的sequence，按照默认配置来说，就是本次WriteGroup会有多少个操作，就会增加多少的Sequence。（与之对应的是Sequence代表的是WAL的ByteSize）

`DBImpl::WriteToWAL`。首先调`MergeBatch`把本次WriteGroup的WriteBatch写成一个。然后写WAL并落盘。

然后记录，CurrentSequence为last sequence + 1，然后last sequence要加上刚才说的WriteGroup的操作数量。

接着是写MemTable的地方，如果本次不允许并发写入，则调用`WriteBatchInternal::InsertInto`。否则的话，调用`write_thread_.LaunchParallelMemTableWriters`。然后Leader调用自己的写入逻辑，同样是`WriteBatchInternal::InsertInto`。

这里唤醒就是将WriteGroup中的所有Writer的状态都设置为`STATE_PARALLEL_MEMTABLE_WRITER`。这样其他等待的线程会在刚才的JoinBatchGroup恢复出来，并发现自己当前的状态是作为并行写MemTable的Writer出现。则会走和Leader相同的逻辑，即调用`WriteBatchInternal::InsertInto`。

然后对于Leader来说，这里有一些LogSync的逻辑，目前还没太看懂是什么逻辑，大概感觉就是标记一些日志已经被Sync过了。

接着Leader和Follower都会调用`write_thread_.CompleteParallelMemTableWriter`，表示完成本次MemTable的写入。这里会原子递减一个running count，如果自己不是最后一个人完成写入的话，就要AwaitState直到所有人写完。否则的话则退出，并说明当前Writer负责将整个WriteGroup结束掉。

负责退出的线程负责更新LastSequence，因为这时候才算是完成了写入，其他的读者才可以读。

对于Leader来说，调用的是`write_thread_.ExistAsBatchGroupLeader`。核心逻辑是，如果当前WriteGroup之后还有其他人，那么当前的Leader就负责指定一个新的Leader，这里老Leader会在此调用CreateMissingNewerLinks，帮助构建双向链表，然后把本次WriteGroup从链表中断开。最后将链表的尾部节点设置为新的Leader并唤醒他。然后再次唤醒本次WriteGroup的其他Writer，告诉他们写入已经完成。

对于Follower，如果是负责退出WriteGroup的人，则它会调用`write_thread_.ExitAsBatchGroupFollower`。这里会复用Leader的逻辑，同样是指定一个新的Leader，然后唤醒其他人，只不过Follower会额外唤醒一下Leader而已。

最后返回给用户的是`writer.FinalStatus()`。感觉这个链路上对于Status的处理很复杂，之后可以仔细研究一下错误的传递，因为涉及到各种错误，比如每个Writer的Callback的错误，写MemTable的错误等。要先汇聚到WriteGroup的Status中，再设置回Writer的Status，同时还有一些Swallow Error的逻辑。

然后回过头来看一下写MemTable的逻辑，由于这个函数有几个重载的版本，这里关注一下并行写MemTable是怎么处理的。这里会创建MemTableInserter，然后调`writer->batch->Iterate(&inserter)`。WriteBatch的Iterate会遍历本次WriteBatch中的所有操作，然后在Handler中调用对应的函数，比如一次写入就会调Handler的Put。

这里MemTableInserter就是作为Handler传入的。比如一次PutCF，就会传入column family id，以及本次写入的KV。获取到memtable，然后调`MemTable::Add()`

在MemTable这里会做一些编码的工作，从table中allocate出来一个KeyHandle，把KV写入到Handle中，然后调`table->InsertKeyConcurrently()`，插入完成后，如果有BloomFilter的话还会更新一下BF。

并行插入MemTable的链路和不并行的区别就是会显式的调用支持并发写入的Table的接口。并且由于Memtable会记录`first_seqno_以及earliest_seqno_`。对于并发写的情况下，需要通过CAS来更新最小值。