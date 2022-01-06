![20220106134446](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106134446.png)

run一个thread都涉及到了什么

比较常规的知识点，选择一个execution context，设置状态（pc， stack等），然后开始执行

![20220106135514](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106135514.png)

对于同一个core上的multithread来说，os并不负责

os只负责粗粒度的调度，保证每个进程都能分享硬件资源

但是在硬件中利用多线程隐藏访存延迟的角度来说，用os调度代价太高了。所以在一个核上的若干个execution context，是由硬件来调度的

![20220106142156](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106142156.png)

比如数据库中的latch，利用自旋锁实现就更好一些。因为这些都是轻量级的短时锁，不会等待太长时间。但是如果用blocking的锁，切换execution context，利用系统调度会需要更长的时间

![20220106151102](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106151102.png)

最简单的test-and-set的实现情况下，由于缓存一致性协议，其他的processor会不断的请求独占状态进行更新，从而导致占用大量的总线带宽

![20220106152249](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106152249.png)

减缓traffic的优化，不是每次通过tas来发出read-exclusive的指令，而是先读。当有机会获得锁的时候再进行tas

![20220106165145](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106165145.png)

Ethernet的解决方法，没有成功获得锁的时候就回退

但是不保证公平性，可能会有starvation

但是基于TAS的方式在一个锁释放以后，我们会有P个写操作来自P个处理器，也会出现大量总线带宽

![20220106165345](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106165345.png)

队列的方式，保证公平性

并且不会出现大量总线带宽，因为我们只会进行一次写，而不是P次写

![20220106165954](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106165954.png)

上面的解决方法中，每次释放锁会invalidate P个processor，因为所有processor都在等l->now_serving

但是这里，释放锁的时候只会修改下一个要获得锁的处理器，所以只会出现一次invalidate

从而优化了总线带宽

![20220106171231](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106171231.png)

利用CAS来实现一些原子操作

完成操作后然后利用CAS来写入结果

但是CAS会遇到ABA问题，在这个过程中有可能已经有人该了原本的值又改了回来，但是CAS检测不到

![20220106171436](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106171436.png)

相当于在LL和SC指令之间创建临界区

当有人修改过LL指向的地址的时候，SC将会失败

（不太清楚的一点是老师上课说这个在不支持cache coherence的处理器常用，但是我认为在支持 cache coherence的处理器中这个更加善用，因为我们可以简单的在过程期间检测总线事务，如果有写ll对应地址的事物，则让sc失效即可）

![20220106173758](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106173758.png)

最简单的方法实现barrier

但是在相同barrier上进行多次等待的时候会出现问题

![20220106173835](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106173835.png)

在到达第二个barrier之前，保证所有的processor都离开了第一个barrier

![20220106174142](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106174142.png)

还有一种方法就是设置相反的flag位

比如第一次的barrier，所有processor都在等待flag变成1

第二次的barrier，processor就会等待flag变成0。并且由于当我们可以到达第二个barrier的时候，flag一定变成了1，所以在第一个barrier等待的processor都可以继续，并且都会被第二个barrier阻塞

![20220106174552](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220106174552.png)

但是无论怎么优化，我们都需要写操作来获取锁并增加barrier counter，所以复杂度仍然是O(p)

因为我们用的是中心化的barrier

利用combining tree来让barrier去中心化

每次操作就只需要在logP上的barrier进行操作

感觉counter只会有一次写，只有最后的txn才需要读

所以主要的消耗还是在barrier共享的锁上面

用原子加和TAS来解决这个问题？原子加负责counter，TAS负责flag

但是原子加也需要总线带宽

这里也提到了，用合并树并不会改善总线带宽的情况

但是会减少latency

所以这种做法可能是在其他的interconnect结构中使用，比如在树形结构中，我们的效率就会高很多