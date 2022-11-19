# Photondb

简单看看

## Env

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119105848.png)

env里封装了随机读，顺序写，以及后台任务

现在的实现有Photon和Std。可能对应的就是异步和同步接口这样

## Table

最外面是Table，里面会包一个`raw::Table<Env>`，这里的Env就是上面的std或者photon

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119110236.png)

raw Table是异步的接口。std的Table外面套了个polling封装成同步接口

主要就是Table的open,close

对于kv的get，put，delete。但是要传进来一个lsn，目前不太清楚期望的lsn是什么样的

还有一个特殊的pin，目前不太清楚是干什么的。

## RawTable

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119110438.png)

这里结构和sled有一定差异，tree和store是解耦开的。比较有意思

```rust
pub(super) struct AtomicTxnStats {
    pub(super) read: Counter,
    pub(super) write: Counter,
    pub(super) split_page: Counter,
    pub(super) reconcile_page: Counter,
    pub(super) consolidate_page: Counter,
}

pub(super) struct AtomicStats {
    pub(super) success: AtomicTxnStats,
    pub(super) conflict: AtomicTxnStats,
}

pub(crate) struct Tree {
    options: Options,
    stats: AtomicStats,
    safe_lsn: AtomicU64,
}
```

这个Tree的结构有点迷惑

```rust
pub(crate) struct PageStore<E: Env> {
    options: Options,
    env: E,
    table: PageTable,

    version_owner: Arc<VersionOwner>,
    page_files: Arc<PageFiles<E>>,
    manifest: Arc<Mutex<Manifest<E>>>,

    jobs: Vec<E::JoinHandle<()>>,
    shutdown: ShutdownNotifier,
}
```

还不清楚page files是怎么和PageTable联动的，看一下写链路吧。

读写都是会走txn

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119111501.png)

从这里看就感觉和LevelDB很类似了。除了lsn是用户传的以外

key就是raw key + lsn。value则是Delete或者Put

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119111941.png)

这里store的guard实际上就是当前的Version。Version里面含有了文件列表等信息，相当于是对当前的version加一个引用，防止被gc掉。和leveldb是类似的。

从这个角度看，txn实际上是拿到file的快照

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119112311.png)

从这里看，刚才tree的stats貌似是一些统计信息。

try_write中，会先find leaf。定位到对应的btree node

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119112619.png)

```rust
pub(super) struct PageView<'a> {
    pub(super) id: u64,
    pub(super) addr: u64,
    pub(super) page: PageRef<'a>,
    pub(super) range: Option<Range<'a>>,
}
```

这里的Range竟然是自己传进去的，我还以为是记录在Page上的

通过PageView根据id读出page。判断Epoch是否有变化，如果有，则说明出现了SMO，我们会做执行reconcile_page，然后重试

看代码貌似没有merge的样子。。。

到了leaf就返回了，否则就从当前的view中找child的page id，以及他的range。

这里的Range和Split有关，下来再仔细看看

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119114002.png)

构建Delta。从guard中开启一个PageTxn。

这里猜测可能就是从LSS中分配一个buffer，然后把Delta写进去。

大概可以猜到，当PageTable CAS失败的时候，这个PageTxn就会GG掉，然后释放掉这个PageTxn，以及对应的buffer

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221119115304.png)

把新的Page Prepend上去

然后去page table中做CAS。如果失败了，就返回一个Addr。

成功的话，就会执行落盘，notify flush job

看起来确实没有Abort LSS buffer的地方。并发的读写貌似是根据lsn来的。通过在key中加入lsn来保证不会乱掉，因为每个key都是unique的。

会在consolidate的时候，根据lsn合并版本。他有一个safe lsn。超过safe lsn的version可能在宕机后就读不到了。所以合并的时候会保留小于safe lsn的最新一个版本用来做持久化，以及所有后续的版本。

做完consolidate之后会删除掉那些delta中的page。删除的信息也是写在了和Page一样的Log中，然后做CAS。CAS如果失败就会把这段删除信息丢掉。

GC的时候遍历Version。如果是空文件，直接Rewrite。然后计算空间利用率，如果不高，就开始GC

GC会遍历文件，并根据空间利用率计算分数，然后排序，取出最高的一个做rewrite

cleaned file就表示已经被rewrite的文件，用来避免重复rewrite。目前还不太清楚为什么这样做，因为GC线程只有一个，不太清楚为什么需要用一个变量标记。也可能是为了简单，因为每次扫描都是重新扫描所有文件



