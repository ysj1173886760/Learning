![20211221093804](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221093804.png)

implement simplest solution first

![20211221095913](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221095913.png)

用指定数量的worker去完成任务，实现简单，开销少，但是需要我们提前对工作负载有一定的了解

![20211221100734](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221100734.png)

半静态的分配，每过一段时间就重新分配负载

因为这些问题的负载是相对可以预测的，我们不会突然在下一时刻跳到一个很差的schedule中，如果当前的schedule是比较好的，那么接下来一段时间也会是相对较好的

![20211221102603](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221102603.png)

为了平衡通讯开销，我们需要调整任务的粒度，比如更大的粒度可能会减少通讯的开销，但是也可能导致负载不均衡的问题

![20211221102845](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221102845.png)

每个worker一个工作队列，当本地的worker工作结束，他可以去别人那里偷任务

![20211221103208](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221103208.png)

一些分布式的work queue的优缺点

适用于分布式系统的计算中，这时从其他节点去偷任务的开销就会比较大

但是在本地的计算会有很好的局部性，这个概念同样适用于单个节点（在cache中的任务和在memory中的任务），NUMA架构等

queue还有一个很好的性质就是他是按序的，只有前面的任务完成了才会执行后面的任务

也就是说队列中的任务不必是独立的，他们可以是有依赖的。按照拓扑序完成任务就行

![20211221103953](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221103953.png)

最终目的都是以尽可能小的开销去平衡各个worker的工作负载

对于用户来说，就是根据application去trade off。

![20211221125553](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221125553.png)

天然的适合去并行化一些用分治思想解决问题的程序

fork相当于创建一个异步执行的函数

而join则是用于同步这些异步的函数

![20211221133216](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221133216.png)

最下面的任务对于当前的核来说有更好的局部性

所以让其他的核去偷队列上面的任务

同时队列上面的任务在分治问题上的规模较大，这样我们就可以不用总是去偷别人的任务

这里还提到了一点就是`lock-free dequeue`

![20211221140531](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221140531.png)

优先去执行child，并将后续的部分留给其他的worker来steal

好处就是如果没有spawn的话，执行顺序和串行的执行顺序是一样的

并且还有个好处就是不会由于生成大量的任务导致空间的浪费

因为后续生成其他任务的逻辑可能会被其他worker偷走

![20211221142028](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221142028.png)

run child和从上面偷取任务的结合来提高局部性

![20211221134546](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221134546.png)

实现sync，记录一下目前完成的task数，所有都完成时就达成了sync

spawn child的线程不一定是sync之后继续执行的线程。可能是执行sync的线程来执行后面的逻辑，前面的ppt貌似表示，是最后一个完成task的worker执行后面的东西，包括同步以及后面的逻辑

其实准确的说，就是因为后续的部分被偷走了，谁偷走了后续的部分谁就负责去执行后续的逻辑

![20211221135911](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221135911.png)

偏向于右边的代码，因为他不仅可以以并行的方法生成代码，而且会减少steal的时间

其他的worker会去偷规模较大的问题，以减少偷窃任务的次数，从而减少偷窃的开销

![20211221134509](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211221134509.png)

可以这样想，我们每次将问题分成一半，那么问题就会被划分成一个二叉树

run-child表示我们优先去遍历子节点，那么剩下的一半就留给其他的线程

当其他的线程来要任务的时候，他们总是会找队列最上面的任务，也就是偏向根节点，深度浅的任务。

那么每个线程所执行的任务就相当于是问题树上的一个枝，这样就提高了局部性

这样可以自然的拓展到多叉树，每次挑选问题的一个分支去解决，剩下的就留给其他的分支去解决。不仅有较平衡的工作负载，同时还达成单个线程的局部性，以及减少通讯开销

妙

想到一个比喻，比如任务是几个人一起吃葡萄

第一个人拿到这一把葡萄，摘掉其中的一枝（上面可能有多个葡萄），然后开始吃，然后把剩下的放到自己面前

剩下的人重复同样的操作，他们从有葡萄的人那里拿葡萄过来，摘掉一枝，然后剩下的放到自己面前

这样自己的葡萄吃完的时候，就可以找自己面前还有没有葡萄（局部性）。当自己面前的葡萄也吃完的时候，就可以去拿其他人面前的葡萄吃（平衡负载）

他们由于比较懒，想少去别人那里拿葡萄，所以每次都挑最多的那串葡萄拿（减少通讯开销）

这就是Cilk的implementation