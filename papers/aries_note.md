If more than one log is used for storing the log records relating to different pieces of data, then a form of two-phase commit protocol (e. g., the current industrystandard Presumed Abort protocol [63, 641) must be used

可能指的是逻辑上的undo导致的相同操作的多log

The undo (respectively, redo) portion of a log record provides information on how to undo (respectively, redo) changes performed by the transaction. A log record which contains both the undo and the redo information is called an undo-redo log record. Sometimes, a log record may be written to contain only the redo information or only the undo information. Such a record is called a redo-only log record or an undo-only log record, respectively. Depending on the action that is performed, the undo-redo information may be recorded physically (e.g., before the update and after the update images or values of specific fields within the object) or operationally (e.g., add 5 to field 3 of record 15, subtract 3 from field 4 of record 10). 

所以说是有三种record？redo-undo, redo-only, undo-only，数据库系统概念中有提到，redo-only就是CLR。保存着什么信息就是什么类型的record，比如一个record保存了要redo的操作，那么就是redo的，如果保存了如何去undo，就是undo record

The WAL protocol asserts that the log records representing changes to some data must already be on stable storage before the changed data is allowed to replace the previous version of that data on nonvolatile storage. That is, the system is not allowed to write an updated page to the nonvolatile storage version of the database until at least the undo portions of the log records which describe the updates to the page have been written to stable storage. 

这里的问题就是为什么我们在将数据flush到磁盘之前，至少要将带有undo信息的log record也flush到磁盘呢？

思考的角度不在于事务提交的时机，而在于log的policy

log的policy分为是否抢占(steal)和是否强制(force)

force policy表示事务提交时，所有修改过的块是否要flush到磁盘

steal policy表示磁盘中是否允许有未提交的事务所修改的块

所以这时候回过头来看我们的redo record，目的是恢复已经提交的事务在磁盘中的状态。那什么时候会出现事务提交了，但却没有写入到磁盘呢？

答案就是no-force policy

同样的，undo record则是储存着能够让我们把磁盘中的数据恢复到之前的状态的信息。那什么时候会出现磁盘中的信息是需要rollback的呢？

答案就是steal policy

所以如果我们使用的是force no-steal的policy，我们其实是不需要记录undo和redo的信息的

但是对于ARIES来说，或者说对于目前所有的高性能数据库，no-force和steal都是必须的。所以redo undo也就都是必须的

那有同学可能会说，那这样的话，force no-steal的日志，还有啥用？

既然我们都是force no-steal了，是不会出现数据丢失的情况的，磁盘中的数据也都是一致的。日志其实就可以简单的看作是一种对于数据读写的优化

我们先不写具体的，而是把每个步骤写出来，日志是顺序IO，速度较快。等到提交的时候一起写，可以用排序等手段优化。（这段基本没用，因为不会有数据库用这个方法）

回过头来看，我们以policy的角度来看待WAL的要求

当一个块flush到磁盘中的时候，这个块的数据有可能是未提交的。如果这时候我们的系统崩溃，同时我们没有将undo信息写入到磁盘中。那么就会导致我们没有办法去回滚这个事务，因为我们没有对应的undo信息来做undo操作。这也是steal policy所导致的结果

为了保证我们可以保持非易失性存储介质的一致性，我们需要在数据块写入之前存入可以让我们回滚的信息

那下一个问题，写入数据块都只需要undo的信息，那redo咋办呢？难道我们不需要redo了吗？

根据上面的描述，我们可以尝试推导，redo信息是为了恢复已经提交的事务的数据的，那么理所当然，redo信息会在事务提交的时候被写入

根据数据库系统概念中的描述，这个要求和undo信息和redo信息存储在不同的日志记录中的系统是相关的描述来看。

对于一般的update日志记录，里面会包含undo和redo的信息，不需要特意的去关注

可能有的系统将undo和redo的信息分开存储到不同的日志记录中，或者是存储到不同的位置，这样我们需要替换dirty block的时候，只需要更新undo区

当事务提交的时候，再统一更新redo区，或者是统一添加含有redo信息的record到日志区

Page-oriented redo is said to occur if the log record whose update is being redone describes which page of the database was originally modified during normal processing and if the same page is modified during the redo processing. No internal descriptors of tables or indexes need to be accessed to redo the update. That is, no other page of the database needs to be examined. 

类似于physical redo，但是是物理逻辑的，数据库系统概念中提到，他是物理的指的是受到影响的页被物理的标记出来，但是页内可以是逻辑的

页内逻辑可以帮我们很好的处理添加删除等操作，而物理的标记页可以让我们不在去根据索引或者其他metadata标识的信息来寻找我们需要的页，因为有的时候metadata或者index已经变了

毕竟逻辑操作不具有幂等性，个人认为能够实现这样的机制和ARIES维护LSN的机制是有密切联系的。因为LSN保证了这些操作只会被做一次，所以在重复历史的过程中不会出现和曾经的操作不同的地方

Being able to perform page-oriented redo allows the system to provide recovery independence amongst objects. That is, the recovery of one page’s contents does not require accesses to any other (data or catalog) pages of the database. As we will describe later, this makes media recovery very simple.

page-oriented redo是页间独立的，让恢复过程更简单。因为我们不需要在去考虑页与页之间的关联性了，比如index和data

In a similar fashion, we can define page-oriented undo and logical undo. Being able to perform logical undos allows the system to provide higher levels of concurrency than what would be possible if the system were to be restricted only to page-oriented undos. This is because the former, with appropriate concurrency control protocols, would permit uncommitted updates of one transaction to be moved to a different page by another transaction. If one were restricted to only page-oriented undos, then the latter transaction would have had to wait for the former to commit. Page-oriented redo and page-oriented undo permit faster recovery since pages of the database other than the pages mentioned in the log records are not accessed. In the interest of efficiency, ARIES supports page-oriented redo and its supports, in the interest of high concurrency, logical undos

对于undo来说，逻辑上的undo可以让我们拥有更高的并发度，因为如果使用page-oriented的undo的话，如果两个事务要修改同一个page，那么后面的事务必须要等待前面的事务提交后才能修改。因为如果前面的事务abort，那么后面的事务的日志也会出问题。

举个例子就是比如前面和后面的事务都要插入一个元组，如果后面的事务导致移动了前面的元组，那么当前面的事务abort的时候，因为使用的是page-oriented undo，所以就会导致page找不到我们的元组的情况

如果我们只是记录logical的话，简单的记录一个插入即可

page-oriented可以提高速度，因为只有日志中记录到的page才会被访问

对于ARIES来说，使用的是page-oriented的redo和logical的undo

这里他没说，但是我们可以推导出这样的结论。由于page-oriented的log需要保证是commited的，WAL在事务提交的时候再去写redo日志，所以这里使用的page-oriented redo。而任意时刻，不论是提交或者中止，我们都有可能使用undo record来刷入dirty block。所以用并行度更高的logical undo更好

其实最主要的原因是redo和undo的性质导致的。redo的时候，我们只需要重复历史，所以即使记录在后面变了位置，也是无关紧要的。

但是对于undo来说，如果使用的是page-oriented的undo，虽然速度快，但是我们需要保证位置不能变，否则之后回滚的时候就有可能找不到这个元组，导致回滚失败

所以对于要求更低的redo信息来说，我们使用速度更快的page-oriented redo

而要求更高，并且可能造成并发瓶颈的undo信息，我们使用logical undo

Buffer management policies differ among the many systems in existence(see Section 11, “Other WAL-Based Methods”). If a page modified by a transaction is allowed to be written to the permanent database on nonvolatile storage before that transaction commits, then the steal policy is said to be followed by the buffer manager (see [361 for such terminologies). Otherwise, a no-steal policy is said to be in effect. Steal implies that during normal or restart rollback, some undo work might have to be performed on the nonvolatile storage version of the database. If a transaction is not allowed to commit until all pages modified by it are written to the permanent version of the database, then a force policy is said to be in effect. Otherwise, a no-force policy is said to be in effect. With a force policy, during restart recovery, no redo work will be necessary for committed transactions. Deferred updating is said to occur if, even in the virtual storage database buffers, the updates are not performed in-place when the transaction issues the corresponding database calls. The updates are kept in a pending list elsewhere and are performed in-place, using the pending list information, only after it is determined that the transaction is definitely committing. If the transaction needs to be rolled back, then the pending list is discarded or ignored. The deferred updating policy has implications on whether a transaction can “see” its own updates or not, and on whether partial rollbacks are possible or not.

这块讲的就是之前说的policy，具体的就是buffer pool的policy，其中也涉及到延迟修改，也就是所有的修改会在最终commit的时候才显现出来，有点类似快照隔离

This also means that, if one or more transactions that had previously modified a page start rolling back, then we need to know precisely how the page has been affected during the rollbacks and how much of each of the rollbacks had been accomplished so far. This requires that updates performed during rollbacks also be logged via the so-called compensation log records (CLRS). The LSN concept lets us avoid attempting to redo an operation when the operation’s effect is already present in the page. It also lets us avoid attempting to undo an operation when the operation’s effect is not present in the page. Operation logging lets us perform, if found desirable, logical logging, which means that not everything that was changed on a page needs to be logged explicitly, thereby saving log space. 

逻辑操作，好处是可以省空间，带来更高的并行度，但是需要我们利用CLR和LSN来保证不会redo那些已经redo过的操作。也就是保证每个操作只做一次

Flexible storage management. Efficient support for the storage and manipulation of varying length data is important. In contrast to systems like IMS, the intent here is to be able to avoid the need for off-line reorganization of the data to garbage collect any space that might have been freed up because of deletions and updates that caused data shrinkage. It is desirable that the recovery method and the concurrency control method be such that the logging and locking is logical in nature so that movements of the data within a page for garbage collection reasons do not cause the moved data to be locked or the movements to be logged. For an index, this also means that one transaction must be able to split a leaf page even if that page currently has some uncommitted data inserted by another transaction. This may lead to problems in performing page-oriented undos using the log; logical undos may be necessary. Further, we would like to be able to let a transaction that has freed up some space be able to use, if necessary, that space during its later insert activity 

仍然是logical undo

Recovery independence. It should be possible to image copy (archive dump), and perform media recovery or restart recovery at different granularities, rather than only at the entire database level. The recovery of one object should not force the concurrent or lock-step recovery of another object. Contrast this with what happens in the shadow page technique as implemented in System R, where index and space management information are recovered lock-step with user and catalog table (relation) data by starting from an internally consistent state of the whole database and redoing changes to all the related objects of the database simultaneously, as in normal processing. Recovery independence means that, during the restart recovery of some object, catalog information in the database cannot be accessed for descriptors of that object and its related objects, since that information itself may be undergoing recovery in parallel with the object being recovered and the two may be out of synchronization [141. During restart recovery, it should be possible to do selective recovery and defer recovery of some objects to a later point in time to speed up restart and also to accommodate some offline devices. Page-oriented recovery means that even if one page in the database is corrupted because of a process failure or a media problem, it should be possible to recover that page alone. To be able to do this efficiently, we need to log every page’s change individually, even if the object being updated spans multiple pages and the update affects more than one page. This, in conjunction with the writing of CLRS for updates performed during rollbacks, will make media recovery very simple (see Section 8). This will also permit image copying of different objects to be performed independently and at different frequencies.

这里说的是恢复的独立性，上面也有提到过。page-oriented redo可以带来恢复独立性，因为我们记录的是具体的每个page的修改，而redo过程中不需要读取其他page的内容。

恢复独立性的好处就是我们可以拥有不同的恢复粒度，而非每次都在整个数据库上恢复。比如我们可能只是损坏了一个page或者一个table，这时候如果是需要整个数据库都进行一遍redo的话，不仅会阻塞其他表的事务，还会耗费大量无用的时间。而恢复独立性可以让我们进行细粒度的恢复，i.e.只恢复损坏的表。同时我们也可以进行并行的恢复，整理出每个page按顺序的恢复操作后，我们就可以并行的重做这些page上的操作

logical undo的一个必要的要求就是我们需要为undo操作log，否则的话由于logical操作没有幂等性，我们可能会得到错误的结果

Minimal overhead. Our goal is to have good performance both during normal and restart recovery processing. The overhead (log data volume, storage consumption, etc.) imposed by the recovery method in virtual and nonvolatile storages for accomplishing the above goals should be minimal. Contrast this with the space overhead caused by the shadow page technique. This goal also implied that we should minimize the number of pages that are modified (dirtied) during restart. The idea is to reduce the number of pages that have to be written back to nonvolatile storage and also to reduce CPU overhead. This rules out methods which, during restart recovery, first undo some committed changes that had already reached the nonvolatile storage before the failure and then redo them (see, e.g., [16, 21, 72, 78, 881). It also rules out methods in which updates that are not present in a page on nonvolatile storage are undone unnecessarily (see, e.g., [41, 71, 881). The method should not cause deadlocks involving transactions that are already rolling back. Further, the writing of CLRS should not result in an unbounded number of log records having to be written for a transaction because of the undoing of CLRS, if there were nested rollbacks or repeated system failures during rollbacks. It should also be possible to take checkpoints and image copies without quiescing significant activities in the system. The impact of these operations on other activities should be minimal. To contrast, checkpointing and image copying in System R cause major perturbations in the rest of the system

减少开销，这里有提到了ARIES的另一个特性，即便是具有幂等性的page-oriented操作，也不会重复做。充分利用dirty page table和lsn，我们可以保证每个操作只会被做一次，这样大大减少了恢复过程中redo的次数，从而提升效率。

具体的，上面这段话提到了，ARIES排除了撤销未在非易失性存储介质上的操作，以及排除了撤销已经提交的更改并再次重做他们。

ARIES还防止了由于恢复期间系统崩溃导致的写入大量的日志，比如不断的进行undo和redo

以及最后提到的，不会在进行checkpoint的时候停止掉系统的事务

到这里，ARIES的大体背景其实已经介绍完了，如果已经了解过ARIES的同学在这里可以停一下思考一下

上面提出了我们遇到的大部分问题，在已经有一定了解的情况下，我们是不是也可以尝试自己推导一下ARIES的操作，或者尝试揣测一下前辈解决问题的思路

学习嘛，开始只是学知识。我们也都知道，知识其实来源于解决实际问题的过程，我们在解决这些问题的时候不断探索，找到新的方法，进行总结整理，得到了我们今天学到的知识

学习过程中，如果有机会看到这些曾经前辈遇见的问题，在结合这些知识，肯定会有更深入的理解。这就是知其然，知其所以然。我们知道前辈是这样做的，通过这些问题，我们可以了解到，前辈为什么这样做

到这一步其实已经可以有很多的收获了，并且对于知识的掌握也肯定会有更深入的理解

可是，这里还有一层是我们没有学习到的，前辈能够解决这些问题，不仅仅是因为他们遇到了这些问题，我还希望能够通过前辈解决问题的过程，揣测和总结前辈解决问题的方法。这样我们才能知道当我们遇到这样的问题的时候，我们要怎么解决。

这一点很难，倘若有前辈愿意总结他们解决问题的方法，肯定是对我会有很大的帮助。

很幸运的是，我曾在两本书中都读到过这样一个思想，利用抽象层解决问题。并且在很多其他的地方都找到过应用这一思想的痕迹。

这个思想很简单，我对他的看法是较为矛盾的，我即希望找到更多类似这样的伟大思想，可以帮助我以多元的角度看待问题，从而得到更好的理解。但是我又觉得，如果这样的思想有很多，他们会不会是重复的，或者有那个地方具有共性，等待着有人发现并找到更加具有统治力的思想。

回到我们的问题中来，我们也在这里尝试应用一下这一伟大思想

首先，我们的问题的要做日志和恢复，这两个其实是等价的，因为如果我们的系统不会崩溃的话，我们大可以一直在主存上进行高速的事务处理，所以日志是帮住我们可以在主存上进行处理的同时，保证我们事务的持久性

然后就是与并发之间的协调，由于我们可能会出现事务的回滚，以及多个事务同时操作的情况，所以我们希望有更加细粒度的日志操作。

如果我们的日志只是简单的记录每个tuple在每次操作前后的区别，虽然这样对于undo和redo速度都很快。但是这种直接面向数据的操作要求我们在事务提交之前不能释放在对应项上的锁。对应到日志中，就是强两阶段封锁的page，而非tuple

这样大粒度的封锁会导致我们的并行度降低，并且如果我们以强两阶段的方式来封锁page，那么事务的瓶颈就会落在日志上

所以这里，我们需要增加间接层，将具体的位置信息抽象起来，这样就可以从封锁page中解脱出来

但是，如果我们使用的仍然是操作前后的信息，就有可能在回滚的时候影响到其他的事务，所以这一点仍然要求我们封锁page

所以我们将值信息也抽象出来，最终就是我们的logical operation，逻辑操作可以解决事务之间冲突的问题，这样可以带来更高的并发度。

但是逻辑操作的缺点是他依赖其他表的信息，比如逻辑操作插入一个tuple，这至少需要我们去metadata中查找这个表，然后再插入tuple，期间还有可能涉及到索引

同时，逻辑操作是比较慢的，因为我们需要复现当时的操作，而不是简单的进行值的替换

这些点都提醒我们，我们不能只用单一类型的日志，而是将他们结合到一起

逻辑操作的优点在于并行度高，但是恢复慢，并且恢复具有依赖性

而物理操作的优点是速度块，但是对于未提交的事务，我们需要封锁对应的page

那我们就结合这两个，在运行时，我们可以应用逻辑操作来进行undo，这时我们不需要redo

对于redo信息，我们不需要保证什么性质，恢复的时候会重复历史。所以之后的更改不会影响到redo信息，所以我们使用独立性更强，并且速度更快的物理redo

还有一个问题就是逻辑操作不具有幂等性，那我们怎么保证undo操作只会被做一次呢？

其实就是为逻辑操作添加日志，在撤销阶段，我们会为已经undo过的操作添加CLR记录，这个类型的日志告诉我们都已经做了那些undo操作了，并且还告诉我们还有哪些undo操作没做

而这个CLR记录也是一个redo操作，因为对于事务来说，无论是commit还是abort，都代表了事务的结束。这时我们就不再需要逻辑操作了，因为我们之后不会再进行更改

所以在这里就可以放心的使用物理的redo操作

至此，算法的正确性已经可以得到保证了

剩下的，ARIES会利用dirty page table，LSN等信息去维护，对于每个page来说，都有多少日志已经被应用到这个page上了。

这样我们可以跳过基本上所有的不必要的重复的操作，从而加速恢复的速度。

后面的加速操作其实更像是engineering的技巧，但是前面将logical undo和 page-oriented redo，以及CLR的配合确实十分精彩