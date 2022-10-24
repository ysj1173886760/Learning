# BufferPool

内核月报这里有一篇[文章](http://mysql.taobao.org/monthly/2017/05/01/)可以参考一下

有一些需要知道的点：

* BufferPool有若干个instance，每个instance都有自己的锁，信号量，物理块(Buffer chunks)以及逻辑链表
* 如果对表进行了压缩，则对应的数据页称为压缩页，如果需要从压缩页中读取数据，则压缩页需要先解压，形成解压页，解压页为16KB。压缩页的大小是在建表的时候指定，目前支持16K，8K，4K，2K，1K。
* 正常情况下，Buffer Pool中会把压缩和解压页都缓存起来，当Free List不够时，按照系统当前的实际负载来决定淘汰策略。如果系统瓶颈在IO上，则只驱逐解压页，压缩页依然在Buffer Pool中，否则解压页和压缩页都被驱逐。（系统瓶颈在IO上说明FreeList不够用可能是由于IO能力不够，刷不下脏页导致无法换出，这时候如果将解压页也换出则会加剧磁盘压力）
  * 压缩页的效果是CPU换IO。缓存压缩页的效果是内存换IO。FreeList不够用是内存不够了，但是如果根因是IO的话，就应该还缓存压缩页
* Free List：代表未被使用的页，新页的申请要从FreeList中获取
* LRU List：根据LRU排序的当前使用的数据页。当FreeList中没有结点的时候，则需要从LRU List中淘汰末尾的结点。
  * LRU List中包含未被解压的数据页
  * LRU List分为两个部分，前5/8为yong list，后面为old list。新的page默认被加在old list头
    * 这里主要是为了防止预读的数据页和全局扫描污染buffer pool。所以全局扫描最多占用3/8的buffer pool
* FLU List: 这里就是Flush List。表示的是脏页链表。所有的页都被修改过。FLU List上记录了第一次修改的LSN。FLU List中的结点按照oldest modification排序
  * 8.0之前的版本中，假如FLU List需要通过flush list mutex保护，保证是按序插入的
  * 8.0之后通过LinkBuf来允许一定程度的乱序，即不再要求获取Lsn和插入Flush List是原子的。
* Unzip LRU List：链表中存储的都是解压页
* Zip Clean List：只在Debug模式下有，存储没有被解压的压缩页。这些压缩页刚刚从磁盘读取出来，还没来的及被解压，一旦被解压后，就从此链表中删除，然后加入到Unzip LRU List中。
* Zip Free：压缩页有不同的大小，比如8K，4K，InnoDB使用了类似内存管理的伙伴系统来管理压缩页。这个链表是用来保存压缩页的空闲链表。
  * 比如8k的链表里就是8k的FreeList。如果新的页面是一个8k的，但是8k的FreeList没有空间了，则从16k的链表中取出一个结点分裂成两个8k的块。

## 数据结构

主要在`include/buf0buf.h`中

核心的数据结构有3个，可以从outline中看到：

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023114339.png)

### buf_pool_t

对应了一个buffer pool instance。即一个buffer pool。这里列几个比较关键的结构

```cpp
/** Array index of this buffer pool instance */
ulint instance_no;
// 从这里可以看出来是若干个instance

/** Current pool size in bytes */
ulint curr_pool_size;
/** buffer pool chunks */
buf_chunk_t *chunks;
/** old buffer pool chunks to be freed after resizing buffer pool */
buf_chunk_t *chunks_old;
/** Current pool size in pages */
ulint curr_size;
/** Previous pool size in pages */
ulint old_size;
// 从这里可以看出来buffer pool可以做resize

/** Size in pages of the area which the read-ahead algorithms read if invoked */
page_no_t read_ahead_area;
// buffer pool控制预读

/** Hash table of buf_page_t or buf_block_t file pages, 
buf_page_in_file() == true, indexed by (space_id, offset). 
page_hash is protected by an array of mutexes. */
hash_table_t *page_hash;
// 用来定位block的哈希表。通过(space_id, offset)定位到buf_block_t或者buf_page_t。这里还提到了page hash是通过array of mutexes来保护的

/** Base node of the modified block list */
UT_LIST_BASE_NODE_T(buf_page_t, list) flush_list;
// dirty list

/** Base node of the free block list */
UT_LIST_BASE_NODE_T(buf_page_t, list) free;
// free list

/** Base node of the LRU list */
UT_LIST_BASE_NODE_T(buf_page_t, LRU) LRU;
/** Pointer to the about LRU_old_ratio/BUF_LRU_OLD_RATIO_DIV oldest blocks in the LRU list; NULL if LRU length less than BUF_LRU_OLD_MIN_LEN; NOTE: when LRU_old != NULL, its length should always equal LRU_old_len */
buf_page_t *LRU_old;
// 这里是LRU list，old只是整个链表中的一个指针

```

### buf_page_t

主要存储一些控制信息

```cpp
page_id_t id;
/** Block state. @see buf_page_in_file */
buf_page_state state;

/** The flush LSN, LSN when this page was written to the redo log. For non redo logged pages this is set using: buf_flush_borrow_lsn() */
lsn_t newest_modification;
/** log sequence number of the youngest modification to this block, zero if not modified. Protected by block mutex */
lsn_t oldest_modification;
// newest modification的记录意义是要保证WAL protocol。
// oldest modification的意义是Flu List中要按照这一项排序

/** node of the LRU list */
UT_LIST_NODE_T(buf_page_t) LRU;
// intrusive linked-list

/** Node used in chaining to buf_pool->page_hash 
or buf_pool->zip_hash */
buf_page_t *hash;
// intrusive linked-list
```

### buf_block_t

这里buf_block_t的第一项就是buf_page_t。

```cpp
/** page information; this must be the first field, so that buf_pool->page_hash can point to buf_page_t or buf_block_t */
buf_page_t page;
// 放到第一个位置用来保证buf_page_t和buf_block_t可以互相转化

/** read-write lock of the buffer frame */
BPageLock lock;
// page-level latch

/** pointer to buffer frame which is of size UNIV_PAGE_SIZE, and aligned to an address divisible by UNIV_PAGE_SIZE */
byte *frame;
// 具体的数据

/** node of the decompressed LRU list; a block is in the unzip_LRU list if page.state == BUF_BLOCK_FILE_PAGE and page.zip.data != NULL. Protected by both LRU_list_mutex and the block mutex. */
UT_LIST_NODE_T(buf_block_t) unzip_LRU;
// 如果是解压页的话，则是unzip_LRU的链表结点。

/** Counter which controls building of a new hash index for the page */
std::atomic<uint32_t> n_hash_helps;
// 应该是给adaptive hash index用的东西

/** mutex protecting this block: state (also protected by the buffer pool mutex), io_fix, buf_fix_count, and accessed; we introduce this new mutex in InnoDB-5.1 to relieve contention on the buffer pool mutex */
BPageMutex mutex;
// 注释里也写到了，控制元信息的锁。用来减少争用
```

## 问题

虽然我略过了很多的控制项信息，这里目前展现出来的基本的控制项也足够引出很多疑问了。

这里先提出问题，然后后文一步一步回答：

* `buf_page_state`都包含哪些项，他们的作用是什么
* buffer pool的起始分配点在哪里，怎么进行内存分配的
* buffer pool什么时候需要resize，怎么进行resize的
* zip_LRU和LRU的关系是什么
* 刷脏的流程是什么
* Page读盘的流程是什么
* LRU和LRU_old是怎么配合的
* watch是什么
* page hash的定位方式为什么是(space_id, offset)而非page_id。是怎么分shard的
* 不同的buffer pool instance有什么关系
* hazard pointer是干什么的
* double write的作用以及实现

我们先通过尝试回答这些问题来争取对buffer pool模块有个基本的了解

## BufferPool初始化

初始化代码在`buf_pool_init`中

这里`innobase_should_madvise_buf_pool`貌似是和core文件相关的，就不仔细看了

然后是一个并行的调用`buf_pool_create`

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023160603.png)

初始化为4个核，然后4个4个的初始化instance

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023160959.png)

初始化好了以后再去初始化一些全局的信息。从这里可以看到LRU old的比例为3/8

`buf_stat_per_index`这里也有注释，记录了每个索引中有多少个page在buffer pool中

然后看看`buf_pool_create`，用来初始化一个instance

主要的工作就是初始化一些mutex以及链表

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023162512.png)

初始化chunk，个数就是total size / chunk unit

然后通过`zalloc_withkey`分配若干个chunk。`zalloc_withkey`的作用在注释中也写到了：Dynamically allocates zero-initialized storage of given size. Instruments the memory with given PSI memory key in case PFS memory support is enabled.

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023163218.png)

这里会对刚才分配的chunk都初始化一遍。`buf_chunk_init`失败则会释放掉刚才分配的内存，并直接返回。

`buf_chunk_t`中只有一些控制项信息

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023163932.png)

所以刚才通过zalloc分配的也只有一些控制项。真正的内存分配则在`buf_chunk_init`中

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023164914.png)

mem size的占用是总共page的数据占用，加上buf block的占用。

分配好内存后，第一部分是block descriptor，也就是`buf_block_t`，后一部分是frame。有点索引数据分离的感觉，这样对于经常访问的控制项信息有更好的局部性。

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023165613.png)

第一块是先计算block descriptor的空间占用。其中size是page的数量。

这里`chunk->blocks + size`就是block descriptor的空间占用。这里如果出现了交集，就减少一个block descriptor，同时增进frame为一个page size的距离。

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023171610.png)

大概就是这个样子。end of block descriptor区域和start of frame应该差距小于一个frame大小

然后下面的代码是初始化block descriptor，并指向对应的frame。然后将这个block加入到buffer pool的free list中

最后将这个chunk加入到buffer pool中

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023172553.png)

初始化完chunk之后，还会设置一些别的信息：

* read ahead area是page num / 32，也就是1/32的缓冲区可以用来做read ahead buffer
* n_page_hash_locks代表了page hash的lock数量。必须为2的倍数，并且不超过1024个。
* zip hash的大小就是2 * page num

最后则是会初始化一下Flush相关的项。比如watch。watch就是`buf_page_t`，存储一些控制信息，之后我们会去研究他的作用。

然后我们看一下`ib_create`是在干什么

其实这里的hash table的同步大概可以猜到就是锁住bucket。因为他要求锁的数量是2的整数次幂。

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023175014.png)

创建一个哈希表

如果没有sync obj的话，创建heap，并返回table

否则的话，则创建若干个sync obj，对应了外面的锁的数量

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20221023175326.png)

初始化锁，然后传给rw_locks字段

这里的level可以到latch_level_tz中看到。说的主要是获取latch的顺序。因为要保证latch是无死锁的。

通过这里的注释大概也可以猜出来innodb在干什么

>  Latching order levels. If you modify these, you have to also update
> LatchDebug internals in sync0debug.cc

这里的level应该是debug模式使用的，在mutex enter的地方hook一些函数，我们就可以跟踪到不按照顺序获取latch的位置，进而定位到死锁的代码。(虽然不能完全预防，但是可以在遇到死锁的时候快速定位问题)

至此buffer pool的初始化就结束了。

## GetPage流程

然后看一下buffer pool对外暴露的最主要的接口，即get page，是怎么执行的。

核心的函数为`buf_page_get_gen`

