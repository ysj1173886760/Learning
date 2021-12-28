![20211228100300](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228100300.png)

cache之间用队列，可能当队列满的时候导致死锁

因为一个队列中的request会需要在另一个队列中进行response，也就是一个队列中的work的完成需要另一个队列的resource

当队列满的时候，两个queue都需要对方queue的resource，导致了死锁

一个解决办法是可以扩大queue的容量，到系统最大事务数量

![20211228100749](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228100749.png)

第二种方法是消除依赖

一个队列依赖另一个队列的空间，实际上是一个队列中的request依赖另一个队列的response

所以我们将request和response拆开，这样队列资源就没有循环依赖，也就不会出现死锁

![20211228101859](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228101859.png)

一个小总结

![20211228125124](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228125124.png)

之前自己一直对这个地方搞不清楚

coherence和consistency是不一样的

coherence让缓存变得透明，在单核中，我们感受不到缓存。在多核中，由于coherence protocol的存在，我们也感受不到缓存

但是对于consistency，保证的是我们读写的值是正确的。在不违反语义的情况下，为了优化性能而对WR操作进行重排序

![20211228130347](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228130347.png)

串行一致性，保证了这四个顺序，也就保证了我们在程序执行过程中，内存的状态始终是符合我们预期的

![20211228131927](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228131927.png)

弱一致性模型，允许违反部分ordering，从而带来更高效的操作

![20211228132048](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228132048.png)

motivation，隐藏延迟

我们不必等一个写操作完全结束后再开启一个新的操作，而是重叠部分操作

多体存储器，重叠访存周期从而提高效率

![20211228131656](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228131656.png)

现代CPU中的一个优化，写缓冲区

通过写缓冲区来延迟写操作，从而允许后续的操作继续进行，这个优化让我们不再保证W-R ordering

和写回缓冲不同，写回缓冲表示去延迟flush cache line的操作，而写本身已经结束了

但是写缓冲则是延迟了写操作本身

![20211228134535](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228134535.png)

![20211228140053](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228140053.png)

acquire和release，相当于一个单向的屏障，防止跨越屏障的reordering发生，以此来保证consistency

![20211228141345](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228141345.png)

![20211228141416](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228141416.png)

主存上的同步，在同步手段的保证下，我们可以实现线程与线程之间的同步，i.e. 读到的内容是相同的，一个的写能被另一个线程读到

当没有使用同步手段的时候，我们不能保证内存的一致性，也就不能保证线程之间的通信是正常的

![20211228142016](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211228142016.png)

所以在多线程中保证程序的同步就是程序员和编译器的责任了

关于consistency，就是保证程序执行的顺序和处理器处理内存中数据的顺序是相同的，这是单个指令流的概念

一旦我们会乱序这些指令，就会导致我们操作的内存是不一致的状态

从而导致其他线程看到的也是不一致的状态，也就导致了同步失效

所以consistency，就是访存的一致性，他发生于一个处理器中，但是影响的是其他的处理器

因为无关指令不会对单个processor造成影响，但会影响其他processor的状态
