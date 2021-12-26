![20211226104209](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211226104209.png)

分布式的目录

每个节点的目录上存储自己内存中的副本都在谁那里

通过维护那个节点持有这个副本来保证数据的一致性

感觉可以适用于memory level的一致性，也就是多个节点之间的共享地址的一致性

局部性友好，本地的数据不需要广播

但是一个节点内部在不受总线带宽影响下，还是用监听协议好一些？

![20211226105135](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211226105135.png)

主要还是防止在总线上广播，而是进行点对点的

因为拓扑结构不一定是全都相连的，所以广播是需要占用大量带宽的

![20211226110503](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211226110503.png)

目录协议很大的一个缺点就是目录需要大量的空间

![20211226122113](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211226122113.png)

不存储全部的位，而是限制一下副本的数量

因为根据数据可以看到，需要很多副本的数据的情况是非常少的

![20211226134311](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211226134311.png)

point to point需要复杂的走线，而且不容易拓展

Ring-based容易拓展，并且更加简单