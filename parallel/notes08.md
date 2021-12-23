![20211223142012](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223142012.png)

改变数据布局，从row-major到block-major

可以发现我们有了更少的cache miss，从而减少了同步时间

同时worker的busy time分布也更加均匀

课上老师推测是因为访存时间减少，使得每个处理器的步调更加一致，从而减少了同步开销

所以这些开销都是相互关联的，分析的时候要综合起来分析

![20211223152124](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223152124.png)

work balance和locality的trade off

右边虽然有更多的冗余计算，但是很好的利用了局部性，所以访存时间减少了很多

![20211223155058](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223155058.png)

并行的计算前缀和

![20211223155755](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223155755.png)

cuda中的实现

可以用很少的几个周期就计算出前缀和，主要是用于适用SIMD指令

![20211223160446](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223160446.png)

所以我们可以根据SIMD长度分块，块内使用更加快速的SIMD来计算前缀和

块间用一个线程计算块间的前缀和，再把前缀和分发到每一个块

![20211223160623](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223160623.png)

代码就是这样的

用scan_warp计算块内的前缀和，线程0计算块间前缀和，最后每个元素再和块间前缀和累加

还可以继续扩大粒度

![20211223160855](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223160855.png)

因为一个块内相互通讯代价少，所以先计算块内，然后再计算块间

![20211223162720](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223162720.png)

segmented scan，我们有一个sequence of sequence，计算每一个sequence的partial sum

和前面不同的是，这里的sequence的长度是变化的

通过head-flag的表示法来划分每一个sequence，然后对前面的算法进行一些修改。我们根据head-flag来判断是否应该将前一块的和加到当前这一块

应用就是可以做稀疏矩阵的乘法

![20211223163019](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211223163019.png)

首先对x用cols中的元素gather一下（gather就是一个primitive，就是根据下标进行load，与之对应的还有scatter，就是根据下标进行store）

然后我们再根据row_starts创建head-flag数组

然后用上面提到的segment-scan来计算每个sequence的前缀和

最终就得到了最后的y

这里之所以不用SIMD加，是因为sequence是变长的