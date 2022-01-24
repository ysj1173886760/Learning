![20220124144910](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220124144910.png)

事务内存的优点

主要是提高开发效率，想对于锁来说代码更加安全，并且performance也不低

![20220124151258](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220124151258.png)

如何存储数据

eager versioning就是直接写，然后通过log来回退

lazy versioning则是写到buffer中，commit的时候再真的写进去

![20220124152811](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220124152811.png)

乐观和悲观的conflict detection

悲观的detection和eager的配合，因为每一次load and store都会检测，所以写到内存中没问题

乐观的detection和lazy versioning配合，最后commit的时候检测缓冲区

![20220124154826](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220124154826.png)

硬件中的设计

为cache line添加R/W位，用来指示cache line在读写集合中

![20220124154933](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220124154933.png)

lazy + optimistic

对于load，读取数据到cache line中，并标记为R

对于store，同样读取数据到cache line中，但是在缓存一致性协议下，不进行总线读请求（exclusive）

因为我们不想让别人知道我们在写（写在本地的l1中，不想让其他核看到）

然后在提交的时候，对于cache line中的数据发出总线请求，其他的处理器会看到这里的请求，并检测是否有冲突，然后abort

特别的，如果我们的数据超过了l1的容量，那txn必须abort

![20220124155907](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20220124155907.png)

通过硬件支持的txn memory来加速事务处理，然后通过软件在这个基础上进行利用

比如拆分一系列操作为小的操作，然后用txn memory来加速