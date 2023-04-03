# LevelDB Read

读链路主要分为两个，一个是点读，一个是RangeScan。

点读的接口是Get

1. DBImpl::Get
   1. ref住当前的mem table，immmutable mem table，以及version
   2. 先读mem table，再读immutable memtable，最后调`version->Get()`
   3. 读完后更新统计信息，可能触发SeekCompaction
   4. Unref
2. 读MemTable的时候会构建LookupKey。这是因为写入MemTable的时候会用一个varint作为length存到开头。所以在比较的时候也需要拼一个一样格式的Key，然后靠`MemTable::KeyComparator::operator()`来比较。
3. Version::Get()
   1. `Version::ForEachOverlapping`。先遍历L0的所有有重合的SST，然后对于下面的每一层，挑出重合的SST来调callback。
   2. 这里读到相交的L0后，会根据file number做排序。在单线程后台Compaction的情况下，L0的FileNumber应该是递增的，不太清楚这里排序的目的是什么。
   3. 调用的callback中会记录首次读取的SST，用来更新统计信息。
   4. 然后尝试读SST。这里会从TableCache中先把table的元数据读出来。然后调用`Table::InternalGet`，先读IndexBlock，定位到具体的DataBlock。如果有filter block的话，还会做一次filter的过滤。最后生成Block Iterator，尝试定位该Key。

RangeScan的接口是NewIterator

1. DBImpl::NewIterator
2. DBImpl::NewInternalIterator
   1. ref住当前的memtable，immutable以及version。存到IteState中，并注册CleanupIteratorState到Iterator的Cleanup中。
   2. 构建MergeIterator，由mem，imm，以及Version的Iterator组成。其中Version的Iterator得到的是，所有L0 Table的TwoLevelIterator，以及非L0的ConcatenatingIterator，也就是由FileNumIterator和Table的TwoLevelIterator组成的TwoLevelIterator。
3. 生成DBIter。DBIter的作用是过滤相同的User Key，以及delete mark。
   1. 在DBIter读取的过程中，还会随机进行Sample。大概意思是期望每读1MB，会作为一次Seek记录到当前读到的文件中。具体理由我不是很清楚。。
   2. 这里DBIter还需要处理双向的Scan。对于前向Scan来说比较容易，不断递增直到找到第一个小于Sequence的Key就行。对于反向Scan来说，只有当前Key大于了Sequence之后，我们应该返回的是上一个Key的值，所以有一个saved value。
