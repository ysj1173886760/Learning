# Innodb Undo

在前文中已经介绍了Innodb中的Undo Log的作用，主要有三个点，分别是保存旧版本，回滚事务，以及作为持久化的活跃事务状态表。这篇文章中来细节介绍一下Innodb中的Undo Log，对于Undo Log来说，需要关注的有：Undo Log是如何组织的，Undo Log的类型，Undo Log何时做GC，Innodb如何通过Undo做事务回滚，保证事务原子性，在Crash Recovery的时候，Undo起到了什么作用。

## Undo Log 组织

先回忆一下，Innodb中的一个表空间下有若干的Segment，Segment中包含了由Extent为分配单位的连续的Page。

对于Innodb来说，Undo Log Record被记录在Undo Page中，为了避免事务过大，当一个Undo Page不足以存储一个事务的Undo Log Record的时候，Innodb就会再去申请新的Undo Page，并通过链表的方式与之前的Undo Page链接在一起。

<img src="https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230922090556.png" style="zoom:50%;" />

分别介绍一下这几块的作用：

* File Header中包含了SpaceID，Page offset，LSN，Type，Checksum等通用字段
* File Trailer中包含Checksum和LSN，用来做原子写入的校验，和File Header一样，是通用字段，不局限于Undo
* Undo Page Header用来记录当前的Undo Page存储什么类型的Undo Log，第一条和最后一条Undo Log Record的偏移量，以及双向链表的节点，用来将Undo Page组织成链表。
* Innodb中将每个由Undo Page组成的链表都划分为一个段（物理上尽可能连续，比如Btree的叶子结点就是在一个段中）。Undo Log Segment Header就是维护这个段信息的（从这个段中分配Page）。同时Undo Log Segment Header还纪录了当前这个Undo段的状态，比如Active/ToFree等
* Undo Log Header则是用来串联一个事务的Undo log，里面会保存TrxID，TrxNO等信息

可能一个最直观的问题是，为什么有了Undo Page组成的链表，内部还需要一个Undo Log Header来串联Undo Log呢？这是因为Innodb中为了减少对Undo Segment的申请，不会为每个事务都创建一个Segment，而是复用现有的Segment。那么为了区分不同的事务的Undo Log，就有Undo Log Header来管理他们。大概感觉如下图

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20231001091745.png)

在Innodb中，一个活跃（Active）的Undo Page链表（后面统称Undo Segment）只能属于一个事务。一个Undo Segment中因为Undo Page的复用，可能保存有多个事务的Undo Log。因为系统中肯定会存在很多并发的事务，那么这些并发的事务也暗示了会同时存在多个活跃的Undo Segment，下面看一下Innodb中是如何组织这些活跃的Undo Segment的

<img src="https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20231001093241.png" style="zoom:50%;" />

Innodb中有一种Page叫做Rollback Segment Header，简称为Rseg，其中需要关注的主要有两个位置，一个是Rseg History，用来做Purge/Truncate，后面会提到，另一个是Undo Slots，里面记录了每个Undo Segment的第一个Page的PageID，每个Undo Segment对应一个Undo Slot，一个Rseg Header中会记录1024个Undo Slots。即一个Rollback Segment Header可以维护1024个Undo Segment

一个系统中1024个并发的读写事务肯定是不够的，所以Innodb中会存在多个Rollback Segment Header，维护在系统表空间的5号Page中。这里会存有128个PageID，每个PageID都指向一个Rollback Segment Header，所以系统中一共可以存在128 * 1024个并发的事务（不是很严谨）。

整体的组织结构如下：

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20231001094722.png)

因为Innodb中会存在临时表空间，这些地方在系统崩溃后是不需要进行Undo的，所以Innodb也会区分不同表空间的Undo。具体来说，这128个Rseg中，0号一定在系统表空间中，1-32号属于临时表空间，而33-127号则可以自由分配，既可以在系统表空间，也可以在自己配置的表空间中。

## Undo Log 格式

看完Undo Log在Innodb中是怎么组织之后，来看一下Undo Log的格式

注意Undo Log的作用是回滚用户的操作，以及保存旧版本，所以每次Undo Log的产生一定对应了用户的某次操作。之前介绍过，用户的操作主要是Insert，Update，Delete，对应到Btree上就是Insert，Update，Delete mark。

Innodb的Undo是在写入聚簇索引（主表）之前写入的，写入的信息需要能够Undo主索引以及二级索引上的变更。

Undo Log的类型主要有4种：

* TRX_UNDO_INSERT_REC，对应插入一条数据。
* TRX_UNDO_UPD_EXIST_REC，更新一个没有被del mark标记的record，对应的是原地更新
* TRX_UNDO_UPD_DEL_REC，更新一个被del mark标记的record，比如insert_by_modify的时候会用
* TRX_UNDO_DEL_MARK_REC，给一个record标记del mark，对应的是删除一个record

<img src="https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20231002102120.png" style="zoom:50%;" />

先看TRX_UNDO_INSERT_REC类型：

* start of record和end of record的作用和双向链表相同，可以通过这两个字段快速定位到下一条undo log或者上一条undo log。start of  record记录的时候本条undo log开始的地址，end of record记录的是本条undo log结束，也就是下一条undo log开始的地址
* undo type为TRX_UNDO_INSERT_REC
* undo no表示的是这是当前事务第几条undo log，从0开始
* table id是对应的表。因为undo log是表级别的，我们需要能够找到对应的表去undo
* list of <len, value>则是本次插入数据的主键，通过长度+值的格式来存储。
  * 你可能想问，为什么一次插入操作的Undo不把所有的数据都记录下来呢？在Undo作为旧版本的时候，Insert的旧版本就是空，所以实际上不需要任何数据。
  * 那下一个问题是，既然不需要任何数据，为什么还要记录主键呢。这是因为Undo的作用还有回滚操作，Innodb需要能够通过Undo中记录的信息从表中回滚操作，对于Insert来说回滚操作就是把插入的数据删除掉，删除掉这个数据只需要通过主键定位到他即可，不需要其他的列。
  * 那么你可能又会问，二级索引还有二级索引项呢，为什么不需要记录索引项的数据？这个和Innodb的写入以及Undo的顺序有关，等下在写入流程中会提到。

<img src="https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20231002105405.png" style="zoom:50%;" />

然后是TRX_UNDO_DEL_MARK_REC，删除一个record对应的Undo：

* Headers上面已经讲过，包含end of record, undo type, undo no, table id
* old trx id/old rollback ptr会记录前一个版本的txn id，以及rollback ptr，用来回溯到老版本
* <len, value> list of pk就是本次删除数据的主键
* index info len和<len, value> list of index共同记录了索引列相关的信息，这里是旧值的索引列
  * 这里可能你又有一个疑问，del mark应该只是标记一个record的上flag，不需要修改任何column，为什么还需要记录索引列到undo中呢？因为我们完全可以通过聚簇索引上的数据得到所有的索引列。
  * 对于undo来说，是这样。因为不会有并发的事务修改相同的主键，那么我们完全可以通过主键定位到数据，然后去undo二级索引。
  * 但是undo在作为旧版本的时候，还需要做真删，即不能简单的del mark了一个record就完事了，还需要在合适的时候将这个数据真正的删除掉，从而释放空间。但是由于有insert by modify这种操作，一个del mark的数据仍然可能被修改，这样之前删除他的事务就无法通过这个tombstone定位到索引列的信息了，那么undo就必须记录索引列的信息，才能对二级索引的数据做到“真删”

<img src="https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20231002112514.png" style="zoom:50%;" />

最后是TRX_UNDO_UPD_EXIST_REC，对应了原地更新的undo：

* Headers，old trx id & old rollback ptr还是一样的
* pk和上面也一样，因为总是要定位是那一行被修改了
* n_updated和update list组成了update vector，记录了本次修改的那些数据在更新前的值。我们可以通过这两项构建出来一个“逆向”的update vector，从而将数据从新版本更新称老版本
* index info len和index list和上面也一样，如果本次修改变更了索引项，就会记录在这里，用来在合适的时机对索引列做真删。
* TRX_UNDO_UPD_EXIST_REC和TRX_UNDO_UPD_DEL_REC的格式是一样的，都是通过update vector记录老版本。只不过UPD_DEL_REC不会记录索引项（因为不需要做真删），并且对UPD_DEL_REC类型的操作做undo不是做update，而是delete。
  * 在这里敏锐一点的同学可能会发现，因为TRX_UNDO_UPD_DEL_REC实际上就是Insert操作，那为什么Insert undo只记录了pk，而这里的Update del rec却额外记录了update vector呢。这里记录的数据实际上是del mark的前一个版本的数据。具体的逻辑我们会在写入的时候看到

## Undo 写入

## Undo 多版本

## Purge

## Truncate



## Reference

http://mysql.taobao.org/monthly/2023/05/01/

https://www.alibabacloud.com/blog/an-in-depth-analysis-of-undo-logs-in-innodb_598966

《MySQL是怎样运行的：从根儿上理解MySQL》第22章/23章