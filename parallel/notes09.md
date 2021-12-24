![20211224135025](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224135025.png)

注意到，当我们提高处理器的数量的时候，我们会降低计算强度

这样就有可能导致communication bound

![20211224135400](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224135400.png)

超线性的加速比

这是因为当我们提高处理器的数量的时候，会在某一时刻让这个处理器所处理的所有数据都存到cache中

这样可以大大提高我们的访存效率，以导致在这样的情况下工作的速度是大于之前相同规模下单核处理器的

因为单核处理器要处理cache miss等问题

所以在曲线的前半段，多核处理器也需要处理cache miss问题，而到某一个点的时候，cache能够装下整个workload，所以我们有了超线性的加速比

![20211224140000](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224140000.png)

量化研究方法中提到的，强拓展性和弱拓展性

评估一个机器/算法的时候，在我们拓展核的数量的同时，也要考虑问题的规模是不是也要拓展

![20211224143214](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224143214.png)

相同的问题，希望做的更快，用并行计算机来计算

也就是强拓展

![20211224143252](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224143252.png)

在一定的时间内做出尽量多的工作

比如实时渲染，在一秒内渲染足够多的帧

![20211224143423](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224143423.png)

对于大的问题，希望能把整个workload放入主存

所以需要大型的计算机来进行计算

又叫弱拓展，问题规模的增大和机器规模的增大是同步的

比如计算大规模的N体问题，就需要大规模的计算设备来把workload放到主存中，同时计算并不受时间限制

![20211224150655](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224150655.png)

energy-efficiency和performance的图像

选择在pareto曲线上的配置来达到最佳的效果

![20211224154013](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224154013.png)

检测性能瓶颈的一些技巧

![20211224155005](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224155005.png)

![20211224155032](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211224155032.png)

roofline模型中，之所以不同访存带宽的斜率是相同的，是因为我们用的是对数的坐标系

所以不同的访存贷款被取log后会转化成偏移量