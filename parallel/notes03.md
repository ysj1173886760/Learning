![20211216105926](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216105926.png)

![20211216105948](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216105948.png)

第一种方案有更好的局部性，因为只有第一次访问会出现cache miss，其他的访存都会出现缓存命中

这里有个问答解决了我的一个疑惑

![20211216110326](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216110326.png)

所以具体那个表现更好其实也取决于Cache的大小

第一种方案下对cache的要求更低

但是这只是考虑到了缓存了，在其他方便，比如平衡工作负载等，还是要根据情况去选择实现方式的

![20211216132425](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216132425.png)

这里有一个评论总结的挺好的

the difference between SPMD and SIMD is simply abstraction vs. implementation, respectively? That is to say, SPMD is the ISPC model that we're using, with gangs and program instances, whereas SIMD is the implementation with multiple ALUs

我们使用的是SPMD的model，我们写代码的时候认为我们会生成若干个program instances，然后为这些instance分配数据。对于ISPC实际的实现，他使用了SIMD指令，而不是生成若干个线程来计算。

![20211216133543](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216133543.png)

这里也有相关的描述，foreach表示这段代码是独立的，我们认为他会在执行的过程中映射到若干个program instance中，但具体实现上，他是SIMD parallelism的

一个很有意思的comment, burger analogy

In my opinion, abstraction is what you do not need to do whereas implementation is what you have to do. For example, when you order a BigMac at McDonalds, you don't need to know how the BigMac is made. You only need to tell the employee, "I want a BigMac and here is 8 bucks". Then after a while you get one. Here, the employee is an interface where you can get burgers from. But if you are a cook at McDonalds you have to know how to make different burgers. Here, as a cook, you "implement" a burger according to the customer's need. And you also use some abstraction others provide. For example, you don't need to know how to grow vegetables, how to keep cattle, ... You only use those materials as you need, which means you are using abstraction provided by farmers.

共享地址，通过读写共享的变量来进行通信

我们常用的线程间通信

但是拓展比较难，因为拓展的代价是在硬件上

![20211216162338](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216162338.png)

![20211216163620](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216163620.png)

消息传递模型

用在进程间通信，RPC

![20211216163731](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216163731.png)

共享地址和消息传递可以相互实现

![20211216165226](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211216165226.png)

whenever you look at a system. First ask yourself what is the code actually mean, what is the semantics of the program. And then ask yourself how are these actually going to be implemented