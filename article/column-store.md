# The Design and Implementation of Modern Column-Oriented Database Systems 笔记

刚刚入门AP相关的技术，很多地方不是很懂，所以读一读这本册子来获取一个大概的overview

## Introduction

一些背景介绍什么的就不贴在这里了。这篇文章主要就是结合MonetDB，C-Store以及VectoeWise这三个系统讲述了列存数据库的大体架构以及一些主要使用的技术。不过要注意的是这里面说的技术虽然整体方向仍然是不变的，但是很多技术的细节以及选择都有所改变。所以我的感觉是通过这篇文章去把握列存数据库的基本思路，然后看看在2000-2010这段时间列存数据库的这些设计决策，再看看新的系统(duckdb/Umbra)的这些设计决策，并且理解这些变化。

开头提到了Access Pattern/Data Layout的trade off，基本意思就是：如果workload访问的数据比较少，那么对于ColumnStore来说，就需要多次的Seek（随机IO），而Transfer时间比较少。而对于RowStore来说，尽管可能多访问了一些数据，但是占比较小，所以更优。随着访问的数据逐渐增加，Transfer的时间占大头，这时候选择ColumnStore的优势则会变大，因为需要访问的数据量整体变少了，Transfer时间更短。

数据库存储数据的格式大概可以分为3种(15721有一节课专门讲这个)：

* N-ary Storage Model（NSM）：就是行存
* Decomposition Storage Model（DSM）：就是列存
* Hybrid Storage Model：混合版，PAX，比如一个block内还是存若干个Tuple的所有数据，只不过是block内部是按列划分的

从NSM到DSM之间混合版的存储格式就有很多种了，除了上面说的PAX，还有一些别的存储方式，也有的格式并不是完全持久化的，而是一些中间结果的格式，在后面会提到

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230624164156.png)

## Technology and Application Trends

这里列了一下磁盘规格随着时间发展的变化，得出的结论是（直接贴原文了）：

* the transfer bandwidth per available byte (assuming the entire disk is used), which has been reduced over the years by two orders of magnitude
* the ratio of sequential access speed over random access speed, which has increased one order of magnitude.

进一步导出：DBMSs need to not only avoid random disk I/Os whenever possible, but, most importantly, preserve disk bandwidth.

还有一点是，在1980s，执行CPU指令和访存延迟基本上是一样的，而随着技术的发展，执行CPU指令的速度越来越快，访存延迟和CPU指令执行延迟比例越来越大，所以1990s有一波研究in-memory data layout的热潮来解决这个问题。

> The original motivation behind MonetDB, which was initially developed as a main-memory only system, was to address the memory-bandwidth problem and also improve computational e"ciency by avoiding an expression interpreter

从这里也可以看到，列存数据库并不是简单的通过列存减少读取数据的数量来获得优势的。还有这些针对现代硬件（超标量处理器，缓存层级的比例）来做的优化。

对于Application Trends来说，就是随着互联网发展，数据规模变大，用户希望搞一些分析bulabula这种。我没有这块相关背景，就不多说了。

然后后面列了列一些工业界的数据库在列存方面的发展，现在来看（由于Snowflake的成功？）是到处都是，回到10年前看的话，貌似也是到处都是？（`e.g., Vertica, Ingres VectorWise, Paraccel, Infobright, Kickﬁre, and many others`）。大公司的话，IBM Blink project，SAP HANA，SQL Server都搞了列存相关的技术集成到他们已有的数据库中。

## Column-store Architectures

这一节是讲一下C-Store，MonetDB以及VectorWise的架构

### C-Store

> In C-Store, the primary representation of data on disk is as a set of column ﬁles. Each column-ﬁle contains data from one column, compressed using a column-speciﬁc compression method, and sorted according to some attribute in the table that the column belongs to

上面这段说的是C-Store的ROS（Read Optimized Store），他还有一个WOS用来优化写入，是行存/不压缩的。具体数据结构不清楚，应该类似MemTable，C-Store会周期性的将数据批量从WOS移动到ROS，用来均摊开销。

C-Store的特点：

> Each column in C-Store may be stored several times in several different sort orders. Groups of columns sorted on the same attribute are referred to as “projections”. Typically there is at least one projection containing all columns that can be used to answer any query.

DBA去针对workload来定义Projection，C-Store会根据请求来选择Projection。并且看这句话的意思，C-Store应该是不会跨越Projection来读取数据的。至于具体选择Projection的逻辑，我脑补的一个可能的算法就是优先选择请求中“选择率”低的列为主排序键的Projection，这样可以提前prune掉很多数据。

> Each column in C-Store is compressed and for each column a different compression method may be used. The choice of a compression method for each column depends on a) whether the column is sorted or not, b) on the data type and c) on the number of distinct values in the column.

C-Store不支持二级索引，但是支持一些在主排序键上的稀疏索引。

> A sparse index is a small tree-based index that stores the ﬁrst value contained on each physical page of a column.
>
> A similar sparse index is stored on tuple position

可以根据排序键/Tuple Poisition快速定位Page。C-Store Page的大小是MB级别，所以索引还是非常稀疏的



