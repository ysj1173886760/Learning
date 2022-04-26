Page Model Transaction

![20220426135852](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220426135852.png)

一个事务内的操作序列只对具有冲突的操作上有偏序的要求

Therefore, in the partial ordering of a transaction's steps, we disallow that a read and write operation on the same data item, or two write operations on the same data item, are unordered.

![20220426141424](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220426141424.png)

这里还是需要再仔细理解一下

页模型的定义中，事务是对页操作的偏序集（或者是对数据项）

而在对象模型中，事务则是由操作构成的树，其中最下层的操作，也就是叶节点就是对页的操作。并且也有偏序关系。叶节点的偏序也构成了内部节点的偏序。

对象模型相对于页模型则是利用了高层次的语义信息。我们平常用的并发控制手段应该是页模型，也就是针对最基础的操作做的控制。并没有考虑高层次的语义信息

我们的冲突可串行化应该就是这么定义的

而对于对象模型，他如果利用了语义信息，就可能有更好的并发效果。比如视图可串行化。并且他的关键在于操作节点之间的可交换性（比如操作可交换，但是在数据项上会有冲突）