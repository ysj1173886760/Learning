# RocksDB-1 Cache

## 接口

目前有两类Cache。分别是LRUCache，以及HyperClockCache。（还有一个CompressedSecondaryCache，注释中写是experimental的，就不看了。）

两种CacheOptions都会继承一个ShardedCacheOptions，说明都是分shard的cache

比较有意思的是RocksDB支持一个叫SecondaryCache的东西，是在In-memory Cache和磁盘层中间的又一层Cache，有点CacheLib的感觉，或者通过NVM来做SecondaryCache。这个目前只有LRU Cache支持，也可以读一读。

Cache的接口处，Handle就是所谓的CacheEntry，即存储在Cache内部的数据，简单看了下用法貌似是通过reinterpret_cast来转化为内置类型用的，所以接口处的handle就类似是一个占位符，用户只要把他看作是一种可以向Cache内部写入，读取，并且可以存储数据的黑盒就行。

这还有若干个Callback，最常见的就是DeleterFn，用来在Cache换出的时候，执行的一些类似析构函数的东西，因为Cache的接口偏向C风格，传入的Value是一个void*。还有几个Callback看起来是针对SecondaryCache用的，现在先不关注。

Cache还有Priority，高优先级的Cache更不容易被换出去。在插入的时候可以指定Priority

看看读写的接口： 

```cpp
  virtual Status Insert(const Slice& key, void* value, size_t charge,
                        DeleterFn deleter, Handle** handle = nullptr,
                        Priority priority = Priority::LOW) = 0;
  virtual Handle* Lookup(const Slice& key, Statistics* stats = nullptr) = 0;
  virtual bool Ref(Handle* handle) = 0;
  virtual bool Release(Handle* handle, bool erase_if_last_ref = false) = 0;
  virtual void* Value(Handle* handle) = 0;
```

这里忽略了为SecondaryCache所准备的接口，以及一些统计数据相关的接口。

Insert就是根据Key插入Value。这里的charge指的是该Entry需要占用的空间大小。

Lookup则是根据Key找到Handle。然后用户可以通过Value获取Handle中的数据。

Ref和Release则是增加和减少ref cnt。

## HyperClockCache

 RocksDB的ClockCache是无锁的，对于高并发情况下表现的会比较好。注释中提到大多数的LookUp和Release都是一个简单的原子操作。

我比较关注的缺点是：

* HashTable是不可变的。（为了做LockFree）
* 当Pin住的Entry过多的时候，会导致Eviction会耗费大量CPU去遍历可以换出的Entry。这时候性能会有回退。
* 只支持16byte的key

然后看看数据部分

```cpp
struct ClockHandleBasicData {
  void* value = nullptr;
  Cache::DeleterFn deleter = nullptr;
  // A lossless, reversible hash of the fixed-size (16 byte) cache key. This
  // eliminates the need to store a hash separately.
  UniqueId64x2 hashed_key = kNullUniqueId64x2;
  size_t total_charge = 0;
}
```

BasicData中存了Value，Deleter，hash key，以及total charge。即数据的主要部分。

```cpp
struct ClockHandle : public ClockHandleBasicData {
  // Constants for handling the atomic `meta` word, which tracks most of the
  // state of the handle. The meta word looks like this:
  // low bits                                                     high bits
  // -----------------------------------------------------------------------
  // | acquire counter          | release counter           | state marker |
  // -----------------------------------------------------------------------
  std::atomic<uint64_t> meta{};
};  // struct ClockHandle
```

ClockHandle中的meta存储了当前Entry的状态，以及他的counter。这里acquire counter代表ref的数量，release counter代表unref的数量。当acquire counter == release counter的时候，说明没有人引用该Entry。

### HyperClockTable

这个结构看起来是一个最内部的实现的结构，有点单个Shard的Cache的感觉。不过Table这个名字感觉更像是一个哈希表，还不太清楚他有没有Swap的功能。

这里的HandleImpl就是在外面使用的时候被Handle cast过来的结构，即Handle的真身。

```cpp
  struct ALIGN_AS(64U) HandleImpl : public ClockHandle {
    // The number of elements that hash to this slot or a lower one, but wind
    // up in this slot or a higher one.
    std::atomic<uint32_t> displacements{};

    // Whether this is a "deteched" handle that is independently allocated
    // with `new` (so must be deleted with `delete`).
    // TODO: ideally this would be packed into some other data field, such
    // as upper bits of total_charge, but that incurs a measurable performance
    // regression.
    bool detached = false;
  };  // struct HandleImpl
```

这里的displacements看起来和哈希表相关，可以后面再看。

detached说的是当前Handle的分配方式。等下结合实现看就可以。

后面还有一堆数据结构，先怼到代码中看看怎么用的吧

#### Constructor

构造函数中会给出capacity。这里他会根据capacity，metadata_charge_policy，以及opts来计算哈希表的大小。

这里的length_bits_就是哈希表下标所使用的位数。

metadata_charge_policy说的是是否要计算Metadata的开销。比如上面的HandleImpl的大小是否要计算在total size中。

这里有个比较细节的地方是，当Metadata的大小也会算入Cache的total size中的时候，如果用户给出的estimated_value_size过小，就会导致空间占用大头在Metadata上。由于我们会超发一些Metadata（数据有Load factor的限制，而Metadata的大小则是与哈希表大小正相关），所以可能导致这里计算出来的哈希表大小的空间占用都在Metadata上，这里就会缩减一下哈希表的大小来减少Metadata的开销。（很细节的因果逻辑我也没理太清楚）

#### Insert



