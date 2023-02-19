# RocksDB-1 HyperClockCache

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
  virtual void Erase(const Slice& key) = 0;
```

这里忽略了为SecondaryCache所准备的接口，以及一些统计数据相关的接口。

Insert就是根据Key插入Value。这里的charge指的是该Entry需要占用的空间大小。如果handle不为nullptr，代表本次写入要将这个entry pin住

Lookup则是根据Key找到Handle。然后用户可以通过Value获取Handle中的数据。

Ref和Release则是增加和减少ref cnt。

Erase是删除掉该Key对应的Cache。还不太清楚并发的Erase/Insert的行为是什么。

## HyperClockCache

这里简单提一下clock eviction algorithm，指的是每个cache entry有一个countdown（score），rocksdb中值最多为3。eviction算法会遍历cache中的cache entry，减少cache entry上的countdown，当遇到一个cache entry的countdown为0的时候，就把该entry换出。

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

这里的state有几种状态：

* Empty: 代表当前slot没人使用，并且其他数据都是未初始化的状态
* Construction：代表当前slot被一个线程所独占
* Shareable：代表当前slot持有一个reference counted entry。包含两种子状态：
  * Visible：代表当前entry可以被lookup返回
  * Invisible：当前entry不可以被lookup返回（被用户所删除）

State transitions:

* Empty -> Construction：插入的时候，获取slot的所有权
* Construction -> Visible：初始化Entry成功后，修改成visible
* Visible -> Invisible：Erase entry的时候。不会出现Invisible -> Visible
* Shareable -> Construction：清理掉一个没有引用的entry，然后开始构建新的entry
* Construction -> Empty：删除掉一个Entry的时候，会重置为Empty

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

#### Constructor/Destructor

构造函数中会给出capacity。这里他会根据capacity，metadata_charge_policy，以及opts来计算哈希表的大小。

这里的length_bits_就是哈希表下标所使用的位数。

metadata_charge_policy说的是是否要计算Metadata的开销。比如上面的HandleImpl的大小是否要计算在total size中。

这里有个比较细节的地方是，当Metadata的大小也会算入Cache的total size中的时候，如果用户给出的estimated_value_size过小，就会导致空间占用大头在Metadata上。由于我们会超发一些Metadata（数据有Load factor的限制，而Metadata的大小则是与哈希表大小正相关），所以可能导致这里计算出来的哈希表大小的空间占用都在Metadata上，这里就会缩减一下哈希表的大小来减少Metadata的开销。（很细节的因果逻辑我也没理太清楚）

哈希表定好了长度就不会改变了，这块相关的变量都是被标记为const的

析构的时候就是遍历一下哈希表，对于还有数据的slot，释放掉他们。

#### Lookup

Lookup里面用到的一个Helper是FindSlot。

FindSlot接受一个hash key，以及一堆callback。

```cpp
inline HyperClockTable::HandleImpl* HyperClockTable::FindSlot(
    const UniqueId64x2& hashed_key, std::function<bool(HandleImpl*)> match_fn,
    std::function<bool(HandleImpl*)> abort_fn,
    std::function<void(HandleImpl*)> update_fn, size_t& probe) {
  size_t base = static_cast<size_t>(hashed_key[1]);
  size_t increment = static_cast<size_t>(hashed_key[0]) | 1U;
  size_t current = ModTableSize(base + probe * increment);
  while (probe <= length_bits_mask_) {
    HandleImpl* h = &array_[current];
    if (match_fn(h)) {
      probe++;
      return h;
    }
    if (abort_fn(h)) {
      return nullptr;
    }
    probe++;
    update_fn(h);
    current = ModTableSize(current + increment);
  }
  // We looped back.
  return nullptr;
}
```

这里increment会or一个1，作用是让increment变成奇数。通常情况下会和TableSize是互质的，这样在遍历的时候就不会固定在某些特定的slot上，而是在遍历所有的slot之后才会返回第一个遍历的slot。(具体的原理有点忘了，死去的记忆告诉我貌似和费马小定理的使用相关，有兴趣同学看看初等数论应该可以找到类似的结论)

看起来会至多probe log次。通过match_fn来确定是否定位到正确的slot。然后判断是否需要abort。最后调用update fn。

这里至多查询log次可能是有什么理论保证，这块我也有点迷惑。。。

这里结合一下LookUp传入进来的函数来结合理解。

match_fn，会先尝试递增acquire counter。如果是visible并且hash key相同，则直接返回。否则的话减少acquire counter。如果不是visible，也要减少counter。对于其他的状态，则放弃undo。乐观读取的好处是可以减少一次load，但是在需要undo的时候，开销则多了一次fetch sub。

对于abort fn来说，会判断当前的displacements是否为0。证明没有人哈希到这个位置，说明entry不存在。

update fn则啥都不做。

#### Insert

insert会先reserve一个occupancy，用来确保slot的数量不超过load factor

然后会有两条分支，对应了是否设置`strict_capacity_limit`

* ChargeUsageMaybeEvictStrict
  * 会先计算出需要evict掉的内存大小。然后通过CAS将新的值写进去。这里通过CAS就是为了防止超发。而对于NonStrict的情况，则可以使用简单的fetch_add
  * 调用Evict。具体的逻辑在后面说。这里会返回本次evict掉的entry数量以及大小。然后根据返回值判断是否符合本次插入的条件。比如是否成功evict掉了足够的空间，或者是否evict掉了一个entry来保证load factor。
* ChargeUsageMaybeEvictNonStrict
  * 只有在超过了capacity的时候才会尝试evict。其数量最小为capacity / 1024，所以evict的数量也是粗粒度的。
  * 调用Evict。但是只有在entry数量不满足条件的时候才会返回false。即就算本次evict没有找到足够的空间，也会返回ok

这里要注意的是，对于`strict_capacity_limit`为false的情况，我们需要保证Insert不能返回失败，但是又必须保证load factor的限制被满足。所以这里就引入了之前说的detached handle。

* 如果尝试Evict失败了，如果用户没有给出handle，那么rocksdb会返回ok，但是不会插入数据，其表现和刚插入立刻就被evict相同。
* 如果用户给出了handle，那么rocksdb就会使用detached_insert，会从堆中分配空间。

对于DetachedInsert来说，逻辑比较简单，直接new一个新的handle出来，设置为detached，然后将其设置为用户的handle，并不会插入到hash table中。

对于非detached insert来说，则会调用之前说的FindSlot。

match_fn：会先尝试or一下OccupiedBit，用来将empty转为construction。（对于其他状态来说，这个or没有作用）

* 如果成功了，我们会将其设置为visible。acquire counter为initial countdown。然后将新的state写入。
* 如果设置Construction失败了，且当前的state状态不为visible，则认为匹配失败。
* 而如果当前的state为visible，则会做类似Lookup的操作，增加ref cnt，比对hash key。

abort：不会abort

update：会更新当前handle的displacements，代表当前slot冲突了多少次。

后面的处理逻辑会假设大概率会成功，不成功的时候可能对应了probe超过log次（也可能是并发的Eviction导致的），这时候会fallback到detached insertion。

#### Evict

Evict会先增加clock pointer。每次Evict会遍历4个slot。当释放的数量超过请求的charge时，则会退出。或者遍历的总数量超过了MaxCountdown * 哈希表大小，说明每个slot都遍历了若干次，这时候如果还不能Evict足够的话，就退出了（来避免一直吃CPU）。

选好slot后，就会更新上面的countdown，这里会跳过不是visible的handle，以及acquire不等于release的handle。然后递减上面的countdown。这里的算法其实也对应了rocksdb注释中说的ClockCache的优先级工作的不是很好，因为即便是对于一个低优先级的entry，多次acquire release以后，他的countdown也会变成最大值3次。这里优先级影响的只是最开始插入进来的时候不容易被换走而已。

如果countdown为0的话，就会将其CAS到Construction的状态。执行Free。

这里的Free流程先减去probe路径上的displacements。调用数据的deleter，然后将meta设置为空。（这里他的注释比较有意思，他说理论上可以先将meta设为Empty，然后执行FreeData，这样其他人就可以更早的看到meta的state。但是代价是需要将delete必要的数据显拷贝出来，然后他说benchmark中发现这样拷贝会有一定的性能回退，所以就没这么做。可以看到这块还是非常细节的。）

### 代码结构

最后看一下代码的结构。

```cpp
class HyperClockCache : public ShardedCache<ClockCacheShard<HyperClockTable>>;
```

HyperClockCache中套了继承+模版，这里就看一下RocksDB这样组织代码的理由。

第一层`ShardedCache<T>`比较合理，因为CacheShard可能有很多种。比如还有`LRUCacheShard`

第二层这里ClockCacheShard中还套了个Table有点奇怪，因为这里其实并没有其他Table的实现，并且在FreeData中也假设了Table的类型就是HyperClockTable

CacheShardBase的作用是希望提供Concept的约束，这样ShardedCache使用起来更加友好。里面还封装了一些比较通用的函数，比如计算哈希值。

ShardedCache也会继承一个ShardedCacheBase，会将一些不依赖模版的实现抽象出来，比如ShardNum，以及capacity

ShardedCache本身则负责分发Shard。

而最后HyperClockCache则负责了转化Value，理解HyperClockTable的作用。这是因为Value/GetDeleter/GetCharge这些函数其实和CacheTable无关，而是一些操作Handle的活。所以就再封装一层来处理这些逻辑，当然还有返回个自己的名字。