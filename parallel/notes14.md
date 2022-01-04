![20220104081445](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104081445.png)

决定worker的数量

我们要考虑的因素有：

充分利用机器资源

对于IO heavy的task来说，利用其他的线程来隐藏IO延迟。

提高并发度，我们不希望让长时间的工作去阻塞轻量的工作

但是我们也不希望去创建太多的线程，这可能导致thrashing（频繁切换线程导致的资源浪费），同时创建线程也会成为关键路径上的瓶颈

在web server中，用进程来替换线程的好处就是我们不需要太多的worker之间的通信，但是进程可以提供很好的隔离性

还有一个点就是有些库可能是不可重入的，多线程的时候只能有一个线程去使用这个库

但是多进程情况下，每个进程都有自己的空间，就不会出现瓶颈

![20220104085428](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104085428.png)

利用负载均衡器来分配负载，通过提高机器的数量来提高吞吐量

![20220104091645](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104091645.png)

网站的访问是突发的，而网站的负载决定了我们要使用的服务器的数量

通过检测服务器的负载，并动态的添加或删除服务器节点，来实现弹性的负载

![20220104094215](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104094215.png)

通过cache来缓存访问，从而减轻数据库的压力

也可以在前端加上cache

![20220104094411](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104094411.png)

比如缓存相同的请求，上面是缓存数据库的请求，这里是缓存web server的请求

![20220104094744](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104094744.png)

还有一种很常见的方法就是通过CDN来进行缓存

![20220104095200](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220104095200.png)

对于上面的一个总结