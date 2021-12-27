![20211227093222](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227093222.png)

cache的包含关系，之前有提到过

![20211227093832](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227093832.png)

由于LRU的算法，可能导致L1和L2中被evicit的cache line不同

导致了L2无法再包含L1，所以我们需要额外的手段来保证包含的特性

![20211227094359](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227094359.png)

处理invalidation的信号，当l1存在这个cache line的时候，也把l1中的copy设置为无效

![20211227094511](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227094511.png)

处理l1的写命中

当l1写命中的时候，设置l2的modify-but-stale位

当需要l2去flush data的时候，检查对应的stale位，并从l1中请求对应的数据

![20211227100437](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227100437.png)

有关活锁，就是不断的abort and retry

一个很经典的例子就是计算机网络中的载波监听

当我们检测到冲突的时候，就回退，并等待一个随机的时间

当冲突次数增多的时候，随机时间的范围也会加大，来保证减少再次冲突的可能性

![20211227101231](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227101231.png)

原子总线上的事务

首先要获取总线的权限，再去发送command，然后接收数据（可能带有回复）

![20211227102358](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227102358.png)

处理器读缓存和snoop controller读缓存出现了冲突

如果总线优先，那么处理器就会被阻塞在读缓存这一步上

如果处理器有限，那么snoop controller就会被阻塞，这样就会导致监听协议被阻塞，从而阻塞其他的处理器

![20211227102659](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227102659.png)

解决方法，把有冲突的地方复制一下，这里我们会在tag上有冲突，所以复制tag

或者是多端口的tag-memory，允许同时的读写

但是注意对于复制的情况，我们仍然需要保证同步，也就是说我们仍然需要阻塞

有点类似MVCC的那种情况，利用副本解决读，但是对于写还是需要阻塞

![20211227105040](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227105040.png)

通过写回缓冲来进行异步的读写，从而防止写回内存的操作阻塞接下来的操作

![20211227105523](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227105523.png)

可以看到，总线上监听的地址以及操作会被controller接收

addr会与cache中的tag进行比较，同时他也会与写回缓冲区的地址进行比较

然后将结果送到controller中，controller再根据结果和总线上的指令进行对应的操作

右边则是processor的cache controller，只会进行总线的读写事务

这里还可以看到，写回缓冲的tag也有一条线在addr那里，表示要进行flush写回缓冲时要处理的数据项地址

所以processor side controller应该是具有总线读，以及写回缓冲写的操作

![20211227130000](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227130000.png)

总线上的deadlock

![20211227130220](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227130220.png)

总线上的livelock，两个processor都想写一个数据，然后开始互相invalidate其他人

我们需要保证在离开M state之前完成我们的写操作

![20211227134020](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227134020.png)

request table，用来match request和response

![20211227133019](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227133019.png)

拆解后的总线事务

首先在request bus上申请，然后arbiter会选择一个processor并grant

得到grant的processor在总线上放地址和操作

然后等待其他processor进行监听

最后得到其他processor的确认（snoop-pending位）

![20211227133216](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227133216.png)

在response bus上，对应的部件（cache，memory）申请回复数据

arbiter选择一个请求并grant access

请求的发送者检查并发送信号表示准备接收（因为有可能请求者正在处理其他的数据）

（个人猜测这里，由于最后的检查这里有可能请求者正在忙，所以返回有可能是拒绝。所以responder可能释放总线并重启事务，或者发送到缓冲区中来防止阻塞总线和自己）

![20211227133900](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227133900.png)

最后就是在数据总线中传输数据，然后释放request table entry

通过使用上面的方法，我们可以流水线化这个过程，从而增大吞吐量

![20211227134250](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227134250.png)

和上面的猜测一样，都有缓冲区来接收数据

如果缓冲区满了就发送NACK，然后过一会儿再重试这个请求

出现冲突时，比如第一个cache请求一个内存中的读，当等待内存返回数据时，第二个cache可能请求一个在相同地址上的写，就出现了冲突

处理方法就是忽略冲突操作

当当前操作与request table中的事务冲突时，就不去申请当前操作（放入队列或阻塞）

![20211227134811](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227134811.png)

这里就是举的例子

这节课的最后，queue的重要性

![20211227134949](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211227134949.png)

或者说是buffer的重要性