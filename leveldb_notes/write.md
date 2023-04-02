# LevelDB Compaction

Compaction分为MinorCompaction和MajorCompaction。我们就直接叫Flush Memtable和Compaction SST。

LevelDB的Immutable memtable只能由一个。Flush Memtable优先级比较高，因为会影响前台写链路。

Flush memtable的调用点为`CompactMemTable()`。有两个调用点，不过都是后台线程做Compaction，所以不会出现并发Compaction的情况。

调用链路如下：

1. 首先ref住current version。其实用处不大，因为没有并发compaction。
2. WriteLevel0Table。这里会遍历memtable，生成新的SST。
   1. 在构建SST的过程中会将file number加入到`pending_outputs_`中。`pending_outputs_`的作用是记录当前正在进行Compaction的文件，防止被删除。然而实际上`RemoveObsoleteFiles`也只有在后台线程调用。所以这里应该没啥作用。
   2. `PickLevelForMemTableOutput`。会选择本次生成的SST应该放置的位置。因为如果Range没有重合的话，就可以直接把文件推到深层中。这里判断的条件有，当前文件的范围和下一层是否有重合，如果有则会放弃下推。还有一个条件是判断当前文件和下两层的文件重叠的Bytes不能太大，目的是为了防止MajorCompaction的时候写放大太大
   3. 生成本次Compaction的VersionEdit
3. 设置VersionEdit的`logfile_number`，表示为小于该`logfile_number`的log file都可以被删除掉了。
4. `versions_->LogAndApply`。将VersionEdit写入到Manifest中，然后生成新的Version。
   1. 这里会调用`VersionSet::Finalize`，用来找出下一次要Compaction的level，以及CompactionScore。计算逻辑为，对于L0，希望L0的file小于4个，否则就会触发Compaction。对于其他Level，则希望total size小于`(10^max(level, 1)) * 10mb`。
   2. 将新生成的Version加入到VersionSet中，并修改Current指向最新Version。
5. 然后会unref immutable memtable，并尝试删除不必要的文件。因为在Flush Memtable之后，该Memtable对应的log就可以被删除了。
   1. 具体来说，这里会将所有活跃version的所有文件加入到`live file`这个集合中。然后将上面提到的`pending_outputs_`也加入到活跃文件集合中。
   2. 接着遍历db目录下所有的文件，对于log file。如果他的number小于`versions_->LogNumber()`，就会被删除。这里`versions_->LogNumber()`就是在Flush Memtable的时候被推高的值。对于SST，当他不在活跃文件中的时候就会被移除。

至此Flush Memtable就结束了。在调用点的外部会Notify等在`background_work_finished_signal`上的线程。也就是前台等待Flush Memtable的写入线程。

然后看一下MajorCompaction

1. PickCompaction
   1. 这里会优先选择SizeCompaction。表示优先进行由于level size过大而触发的Compaction。会根据之前记录的`compaction_level_`来遍历该层下所有的文件，并挑选出`f->largest`大于`compaction_pointer_[level]`的文件。这里compaction pointer代表的是每次compaction结束的下一个文件，即轮转Compaction。
   2. 如果没有size compaction，那么就会选择seek compaction。会直接取到需要compaction的file。
   3. Ref住current version。还是和之前一样，compaction的时候是单线程后台执行，所以current应该不会改变。
   4. 对于L0来说，会选择在L0中所有和当前文件Range重合的文件。在GetOverlappingInputs也会对L0有特殊处理，即当挑选到了重合的文件的时候，如果导致需要查找的Range变大了，则会更新range，并重新搜索。
   5. SetupOtherInputs：
      1. AddBoundaryInputs。这里是处理一个corner case，就是compaction file和其后面的SST的user key相同的时候，会把后面的SST也带上做Compaction。
         1. 因为我们需要保证的是，对于同一个UserKey，其Sequence会随着Level递增而递减的。
      2. 然后找下一层中，和本次CompactionFile重合的所有文件。同样要处理Boundary。
         1. 这里处理Boundary的原因应该和GC Record有关。因为在Compaction的时候，把相同UserKey的数据放在一起做Compaction感觉逻辑简单一些。不过很细节的例子我还没有想到。
      3. 然后会在此尝试拓宽一次。这里注释有写到，如果可以在不增加Level + 1的range情况下，拓宽Level的Range。则会在此进行pick。这里还有个限制就是Compaction Size不能超出限制。
      4. 然后会计算level + 2的文件中的重合文件。
      5. 更新compaction pointer为本次level中的最大值。下次Compaction就会找比这个值大的file来进行compaction
2. 如果是trivial move。即level层文件只有一个，并且level + 1层没有overlapping。同时与level + 2的overlapping size小于一个阈值。就会直接把该SST下移一层。
3. 执行Compaction。DoCompactionWork
   1. DBImpl里维护了所有的Snapshot。这里会取最老的snapshot的sequence number。
   2. 在Compaction的过程中，还会不断check是否需要CompactMemTable。来防止前台停写。
   3. 对于每个Key，都会尝试计算当前生成的SST与level + 2的重叠大小。如果过大则会直接生成SST，来减少写放大。
   4. 然后会判断当前Key是否需要保留。
      1. 如果上一个Key的sequence小于snapshot，则drop。因为smallest snapshot会看到上一个key。
      2. 如果当前key的类型为删除，并且当前key的sequence小于smallest snapshot，且后面没有文件可能含有该key。则drop。
   5. 不drop的话，就把当前的key加入到builder中。当file size超过该level的max size的话，则生成SST。
   6. 生成Compaction Results。这里会把被Compaction掉的文件删除，记录新生成的SST。然后Apply到Version中。

感觉一些边界条件想的不是很清楚。还有就是Compaction的时候会考虑level + 2的事情，感觉比较新鲜。

有关Compaction的时候，为什么要根据Snapshot来判断。是因为用户在拿到Snapshot后，可能距离使用Snapshot有一定时间。这样导致可能一些SST并没有被Ref住，为了保证该Snapshot有效，我们就不能删除这些老的Key。

假如Snapshot中Ref住了SST。这样就不需要判断了，因为用户可以直接去老的SST中读。不过这样用户还需要ref住memtable，可能会导致内存无法被及时释放。