![20211225103027](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225103027.png)

一个小复习，执行一个写操作在cache层面会发生什么

我们要考虑该地址不在cache中，以及cache满时要evicit一个cache line，并且如果该cache line是dirty的，我们还需要将该cache line写回

![20211225124059](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225124059.png)

单核系统上也会有cache coherence问题

发生在CPU和其他设备通讯的过程中

这也就是为什么有的设备IO空间是不可缓存的

或者需要让操作系统显式的flush这些page，或者这些cache line

![20211225135148](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225135148.png)

所谓的sufficiently separated time应该和cache coherence protocol有关

就是指一致性协议同步cache所需要的时间

这就是弱缓存一致性

![20211225145943](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225145943.png)

![20211225141914](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225141914.png)

MSI protocol

三个状态，失效，共享，修改

在量化研究方法中有，这里就不细说了

![20211225145613](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225145613.png)

MESI，对MSI的一个优化

减少转化到M状态下的总线带宽

比如正常状态下，在没有share的情况下，我们读一次写一次，从I到S，再到M，会有两次总线的事务

但是有了E状态，我们就是从I到E，再到M，只有一次总线事务

优化了无共享状态的总线带宽

但是I到E他是怎么知道其他人没有这个副本的我不太清楚，应该需要其他人回应

注意这里我们是一个cache line对应一个状态

并且读写的单位都是cache line

![20211225150816](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225150816.png)

优化，比如F代表forward，表示哪个processor负责将cache line转发给请求的processor

AMD中的owned表示持有这个cache line，而不会写回给内存

这样可以减少总线带宽，但是owned的processor负责处理其他processor的cache miss

![20211225152139](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225152139.png)

基于更新的一致性协议

就是谁写谁就负责把其他的processor都更新

导致的问题

![20211225152344](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225152344.png)

![20211225153027](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225153027.png)

多级缓存中的应用

L1是L2的子集，我们要保证在L1中的写也会在L2中体现出来，这样就不会出现不一致的情况

用L2去处理总线事务即可

还有一个很有意思的一点是失效协议的false sharing

![20211225152432](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225152432.png)

比如若干个独立的线程，每个线程会更新自己的值

但是这几个线程的值存储到了同一行cache中

失效协议就会导致cache line在不断的切换

下面的实现就为每个线程都用了自己独立的cache line，速度会提高很多

![20211225152716](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225152716.png)

![20211225152808](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211225152808.png)

监听协议的总结

主要就是基于广播来将有可能影响到其他人的操作广播

但是会有scalability的问题，因为我们会受到广播带宽的限制