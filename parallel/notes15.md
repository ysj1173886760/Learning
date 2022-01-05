![20220105083523](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105083523.png)

一些基础的术语

![20220105083806](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105083806.png)

设计中的问题

节点之间的拓扑关系，他们是怎么相连的

一个节点的消息是怎么到达另一个节点的

节点/router上的缓冲，数据的存储形式等

![20220105084327](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105084327.png)

bisection bandwidth，二分开这个网络，然后看他这两块中的link所构成的bandwidth

当两条消息需要使用相同的硬件资源的时候，就会导致blocking（硬件资源的争用）

![20220105085243](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105085243.png)

随着吞吐量的增加，latency也会增加

三个因素共同确定了这个图表，分别是topology, routing, flow control

latency则是他们影响的因素的和，而对于throughput，则是他们的最小值

然后是不同的topology

![20220105091021](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105091021.png)

总线结构，简单，易于实现coherence protocol

争用高，耗电，带宽低

![20220105091221](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105091221.png)

每一个node都相连，不需要通过node间接发送数据

难以拓展

![20220105091326](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105091326.png)

环状

简单，花费不高

但是有高延迟，并且二分带宽低（无论怎么拓展，都是1）

![20220105092045](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105092045.png)

mesh

局部性好

latency低

路径多

link长度相同

![20220105092514](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105092514.png)

特性更好

但是复杂度更高，不容易放到芯片上

![20220105092855](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105092855.png)

树型

局部性好

通过fat tree来避免根节点处的带宽限制

![20220105093040](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105093040.png)

hypercube

低延迟

需要一定数量的link

不好布线

![20220105093251](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105093251.png)

omega

划分的方式类似树状数组

![20220105094918](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105094918.png)

提前预订好link

但是利用率会很低

![20220105095048](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105095048.png)

常用的packet based routing

遇到冲突了就把packet存到buffer中

但是latency会变高

![20220105100024](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105100024.png)

我们可以pipeline这个操作，在不明显减少latency的情况下提高吞吐量

核心思路就是将操作划分成细粒度的，从而提高硬件资源的利用率

![20220105102149](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105102149.png)

通过更细粒度的划分来减少延迟

![20220105102555](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220105102555.png)

复用物理通道来构成逻辑上的虚通道

防止packet之间相互阻塞