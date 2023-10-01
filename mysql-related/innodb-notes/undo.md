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

Innodb的Undo是在写入聚簇索引（主表）之前写入的，写入的信息需要能够Undo主索引以及二级索引。

## Reference

http://mysql.taobao.org/monthly/2023/05/01/

https://www.alibabacloud.com/blog/an-in-depth-analysis-of-undo-logs-in-innodb_598966

《MySQL是怎样运行的：从根儿上理解MySQL》第22章/23章