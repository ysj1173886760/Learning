Page Model Transaction

![20220426135852](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220426135852.png)

一个事务内的操作序列只对具有冲突的操作上有偏序的要求

Therefore, in the partial ordering of a transaction's steps, we disallow that a read and write operation on the same data item, or two write operations on the same data item, are unordered.

![20220426141424](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220426141424.png)

