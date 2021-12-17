![20211217093054](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217093054.png)

截图比较诡异

这里老师问的是这段代码里那个for loop是parallel的

答案是没有，这整个函数都是按顺序执行的，真正的并发，或者说叫逻辑上的并发发生在调用函数的时候

这里是SPMD模型，相同的程序对应的是不同的数据。在abstraction层级，我们认为在调用函数的时候，ISPC会生成若干个program instance，每个instance有自己的progranIndex，他们都执行这段代码，只不过是每个instance负责不同的数据。

要注意的是这里是abstraction，ISPC最终还是会将这段代码翻译成SIMD指令，但是在abstraction上，我们认为是若干个instance在处理数据的不同子集

![20211217095125](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217095125.png)

数据并行

以前是向量操作，现在是map reduce这样的，利用function或者算子来对数据进行独立的操作

![20211217100127](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217100127.png)

流式处理的好处：数据是提前准备好的，可以预取。第一个kernel的输出可以立刻应用于第二个kernel，利用了空间局部性，并且这些数据是暂存在cache中的，可以节省带宽。

缺点：operator不够用，需要库来提供更强大的operator，以及依赖编译器能够给出合适的代码

![20211217101215](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217101215.png)

gather和scatter，通过给定的下标对数据进行读写

![20211217102132](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217102132.png)

利用一些特定的程序结构来进行优化，比如functional programming

实际中，很多语言不强制要求这种模型，而是选择了灵活性以及程序员们更熟悉的命令式编程

functional thinking很棒，但是这些语言，这些框架应该利用这些结构来得到高性能的实现，而不是去妨碍他们。

个人理解就是如果我们强制使用了functional的形式，就会方案程序员编写更好的代码，也不容易得到更高效的实现，所以相当于是将一些编写高性能代码的责任从编译器转移到了程序员身上

![20211217104634](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217104634.png)

这里有一个很好的解释

- Decomposition: Most of the time programmer is responsible (does not have a sophisticated compiler to achieve this yet).
- Assignment: Many language/runtimes are able to take the responsibility. It could be done statically by programmer (Pthread workload assignment by programmer), statically by compiler (ISPC foreach) and also can be done dynamically (ISPC tasks).
- Orchestration: Most happens at runtime, but need to be declared or defined by programmer.
- Mapping: maybe OS (mapping pthread to CPU cores); maybe Compiler (ISPC assigns program instances to data lanes); maybe hardware (mapping CUDA thread blocks to GPU cores).

![20211217104814](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217104814.png)

很经典的东西了，加速比被程序中可并行的部分所限制

![20211217140529](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217140529.png)

barrier，所有线程都执行到这里的时候才能继续

可以用一个原子加的计数器来实现

类似这样，就可以阻塞直到所有线程都执行到这里

```c
atomic_ocunter++;
while (atomic_counter < num_threads) {
    // do nothing
}
```

![20211217140836](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211217140836.png)

slide下面的comment

The first Barrier is to prevent the case that after some thread calculates diff, diff gets overwritten by other threads when executing diff = 0.f.

The second Barrier is to make sure that all threads has calculated their own diff, so the final diff is the sum we want before checking convergence.

The third Barrier is to prevent the case that some thread already on next iteration writes diff to 0, which will make the if statement true for other slower threads, leading to a wrong result.