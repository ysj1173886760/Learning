解释一个系统

![20211218100032](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218100032.png)

![20211218100118](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218100118.png)

渲染流水线，和之前提到的结构化的程序类似。这里是在流数据上去应用这些kernel

![20211218130054](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218130054.png)

CUDA中，一个grid上有若干个block，每个block有若干个thread

thread可以通过一个最高为3维的多维向量threadID表示

代码中可以看到，threadPerBlock是(4, 3, 1)，对应到图中就是一个block里，第一维是4个thread，第二维是3个thread

假设A，B，C是长度为(Nx, Ny)的矩阵

然后定义了numBlocks的第一维为(Nx / threadsPerBlock.x)，第二维是(Ny / threadsPerBlock.y)

也就是说，每一个cuda线程负责了数组中的一个数据

具体的位置可以由cuda线程的threadID和blockID来标识

个人猜测cuda线程可能对应的是SIMD中的一个lane

![20211218130734](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218130734.png)

核函数具体的实现，可以看到数据的位置是由blockID和threadID共同确认的

调用核函数会启动一堆cuda线程，并在所有线程都完成后返回调用点

十分类似ISPC

根据课堂上将的，cuda和ISPC的不同在于，ISPC会创建一系列的instance然后映射到程序中，具体数量不确定？

但是cuda的线程数是确定的，在这个例子中，每个数据都是一个线程，一共就是72个线程

当然了这里都是abstraction，并不是硬件内部具体的实现

![20211218131908](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218131908.png)

这里提到了. number of SPMD threads is explicit in program

![20211218132626](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218132626.png)

memory分为线程私有的memory，block内共享的memory以及全局共享的global memory

![20211218134932](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218134932.png)

一个对cuda的总结

这里要注意的一点是这个

![20211218135049](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218135049.png)

barrier是对block中的thread同步，跨block的thread不能通过这条指令同步

![20211218140338](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218140338.png)

GPU的设计，block作为task，GPU core作为worker。通过代码可以提前得知每个block需要的资源，动态调度器就可以根据这些需求来将block分发到core上执行

![20211218140508](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218140508.png)

一个worker就是一个GPU core，或则一个execution context

每个核可能需要多个worker来隐藏访存延迟（多线程处理器）

不是遇到一个task就创建一个thread，而是提前分配好worker资源，然后再动态的调度task给worker

![20211218142342](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218142342.png)

64个execution context。每一个核可以同时执行4个warp，每一个warp可以同时发射两条指令，并且有32个ALU单元

也就是说每一个clock，我们可以执行4*32=128次运算。

在刚才的例子中，4个warp就可以运行一个block，一个核就可以运行16个block。

所以一个cuda thread就相当于一个lane，一个warp就相当于一个thread

指令级并行：一个warp发射两条指令

线程级并行：一个clock可以运行4个warp

数据级并行：每个warp有32个ALU，也就是32个lane

看起来并没有hyper threading，因为貌似一次只能选4个warp

![20211218144515](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218144515.png)

一个block上的所有warp都要在同一个core上运行

![20211218150900](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218150900.png)

发射固定数量的block，task由程序员来调度

这里每个block就相当于一个worker，他们通过原子加的方式，以block为粒度来执行这些task。 i.e. 每个block每次都会申请threadPerBlock这么大的工作量执行，内部就由cuda thread执行（SIMD + 多线程）

也就是说，发射block的数量不取决于工作负载，而取决于硬件资源

![20211218151319](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211218151319.png)

abstraction中，每一个block对应的就是SPMD中的一个program instance。而cuda thread则是对应不同的数据，或者叫不同的lane

implementation中，block被分割为同一个GPU core上的若干个warp。分布于同一个core中允许不同的cuda thread之间执行高效的通信以及同步。warp则是一个execution context，他们执行的是SIMD指令

这样看的话，一个block中线程的数量最小就是warp中lane的大小，最大则是整个core中lane的大小

量化研究方法中，说GPU是多线程多SIMD处理器

按照上面的例子

第一个多就是多线程，也就是一个core中可以存储64个execution context，并最多有4个execution context执行

第二个多就是多核，一个GPU上有很多的GPU core

SIMD便表示每个warp执行的指令都是SIMD指令，多个lane，同时操作多条数据

还有一个隐含的点就是指令级并行，每个warp同时发射两条指令，图中只画了执行单元，可能另一条指令做标量运算，或者进行访存

有关上面说的hyperThreading，或者叫SMT。因为SMT是用来发现指令之间的独立性，以利用execution unit的。GPU中我们已经在cuda上表达了代码的独立性，因此GPU有足够的线程和指令来利用这些资源。所以GPU不需要SMT

说白了就是GPU线程已经很多了，不需要再自己去想方设法提高并行度了。

还有一种理解的方法，就是GPU已经支持了SMT了，因为我们本身一个clock就可以执行4个warp，也就相当于4个线程了