![20211215181514](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215181514.png)

这里的指令流一致性，指的是多个数据执行的指令流是相同的

比如一个SIMD指令中的若干个数据都有相同的指令流（相同的分支），那么我们就可以跳过不必要的分支来加速执行

反过来，如果指令流分布的不好，我们就要把分支都执行一遍，再用位掩码来处理分支的结果，从而导致并行性较差

![20211215181437](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215181437.png)

延迟和带宽的区别

![20211215183744](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215183744.png)

![20211215185807](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215185807.png)

利用多线程隐藏高延迟操作的延迟

Interleaved multi-threading，这里根据我之前看量化研究方法的猜测，应该指的是相对粗粒度一些的多线程操作

每个核仍然是在执行一个指令流，出现延迟操作的时候就可以切换线程

更加粗粒度一些的是磁盘IO，等待锁等，这时候就是用操作系统来进行调度

这里的提到的调度是硬件级别的，是CPU自己进行调度

SMT就是多线程和超标量的结合版

一个核在一个周期中不只是执行一个指令流中的命令，而是可以执行若干个线程中的指令，相当于是指令级别的粒度

![20211215190549](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215190549.png)

一个线程的ILP不够时就可以用SMT来达到更高的ILP

需要更高的带宽，因为多线程导致每个线程的空间减少了

![20211215191331](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215191331.png)

这就是为什么GPU需要自己的Memory，而不是去访问主存

![20211215191506](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215191506.png)

最后一点，GPU的主要目的应该是隐藏延迟，所以需要大带宽

CPU中利用了很多Cache机制，以及预测机制来减少Latency

但是都是在利用多线程来提高性能

![20211215192111](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215192111.png)

Matlab这个例子中也看出来，不能一味的去尝试多线程，因为切换上下文的延迟也会有一定的开销。但是这里有点奇怪的是，如果设置的线程数和core的数量相同的话，应该不会出现切换线程。也可能是因为运算或访存的代价太低了？

个人感觉是Matlab的原因，或则是他的代码的原因

因为context switch应该已经很快了

![20211215192948](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211215192948.png)