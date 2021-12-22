![20211222104525](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222104525.png)

异步的send和recv，我们需要返回一个handle来检查数据是否已经被正确的发送/接收了，这样我们才能安全的继续操作数据

![20211222134506](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222134506.png)

即便我们用了pipelined的方法来传递消息，只要缓冲区不是无限大的，我们还是会得到一个没有流水化的执行过程

![20211222140226](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222140226.png)

计算强度，computation / commnunication，越高越好

说明算法的限制在计算，而非通讯

![20211222140010](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222140010.png)

inherent communication，取决于我们的算法，通过优化算法来优化通讯成本

artifactual communication，取决于算法映射到的系统，比如每次访存，我们不是只取到一个元素，而是取一行cache

![20211222141354](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222141354.png)

新的一种cache miss，由于多核之间的缓存一致性导致的cache miss

![20211222141253](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222141253.png)

随着容量增大，cache miss在减少

![20211222145403](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222145403.png)

通过改变遍历的顺序来提高局部性，就是分块遍历

![20211222145442](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222145442.png)

通过融合计算来提高局部性，其实是减少了去访存的次数，将多次访存重叠到一起

![20211222145758](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222145758.png)

communication的粒度就会影响我们之前提到的artifactual communication

![20211222150222](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222150222.png)

冲突

由于对某一个器件的突发使用导致的

中心化的一些结构的通病

之前提到的分布式的工作队列就可以很好的缓解这个问题，因为我们可以找空闲的队列来偷取任务，而不需要一个中心化的调度器

![20211222150731](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222150731.png)

cuda中的contention

所以在cuda编程的时候，一个很好的做法是让一个核内可以有多个block

这样我们可以通过其他block中的线程来隐藏访存延迟

![20211222152154](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222152154.png)

一个优化，首先计算出每个particle的cell，然后对他们排序，然后再对每一个particle，更新他对应cell的起始index和末尾index

这样第一步和第三步都可以并行，并且不需要锁，没有争用

但是需要第二部sort的额外开销

这里的第三步，其实就是检查每一个下标，如果他的前一个元素的cell和当前的cell不同，说明当前这个是当前cell的第一个，前面那个元素是上一个cell的最后一个（因为我们对cell排序了），根据这个更新start和ends就行

![20211222152551](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222152551.png)

通过合并消息来减少通讯的开销

通过提高局部性来减少延迟

通过复制，交错访问冲突资源，或用细粒度锁来减少冲突

提高overlap，比如pipeline，多线程隐藏访存，预取

![20211222152604](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211222152604.png)