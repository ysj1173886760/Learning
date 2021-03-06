记录一些我认为并不常见但是比较有用或者比较有趣的知识点

广义的系统内存空间包括IO空间和内存空间，不同指令集对系统内存空间的定义各不相同。X86指令集包含独立的IO空间和内存空间，对这两部分空间的访问需要使用不同的指令：内存空间使用一般的访存指令，IO空间使用专门的in/out指令。而MIPS、ARM、LoongArch等RISC指令集则通常不区分IO空间和内存空间，把它们都映射到同一个系统内存空间进行访问，使用相同的load/store指令。处理器对IO空间的访问不能经过Cache，因此在使用相同的load/store指令既访问IO空间又访问内存空间的情况下，就需要定义load/store指令访问地址的存储访问类型，用来决定该访问能否经过Cache

![20211207125302](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207125302.png)

除了X86还保留堆栈型和累加器型指令系统外，当今的指令系统主要是寄存器型，并且是寄存器-寄存器型。使用寄存器的优势在于，寄存器的访问速度快，便于编译器的调度优化，并可以充分利用局部性原理，大量的操作可以在寄存器中完成。此外，寄存器-寄存器型的另一个优势是寄存器之间的相关性容易判断，容易实现流水线、多发射和乱序执行等方法。

![20211207125354](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207125354.png)

*应该是可以代表RISC和CISC中word类型的不同

在执行访存指令时，必须考虑的问题是访存地址是否对齐和指令系统是否支持不对齐访问。所谓对齐访问是指对该数据的访问起始地址是其数据长度的整数倍，例如访问一个4字节数，其访存地址的低两位都应为0。对齐访问的硬件实现较为简单，若支持不对齐访问，硬件需要完成数据的拆分和拼合。但若只支持对齐访问，又会使指令系统丧失一些灵活性，例如串操作经常需要进行不对齐访问，只支持对齐访问会让串操作的软件实现变得较为复杂

RISC指令集中很多条件转移采用了转移延迟槽（Delay Slot）技术，程序中条件转移指令的后一条指令为转移延迟槽指令。在早期的静态流水线中，条件转移指令在译码时，后一条指令即进入取指流水级。为避免流水线效率的浪费，有些指令集规定转移延迟槽指令无论是否跳转都要执行。MIPS、SPARC和PA-RISC都实现了延迟槽，但对延迟槽指令是否一定执行有不同的规定。对于当今常用的动态流水线和多发射技术而言，延迟槽技术则没有使用的必要，反而成为指令流水线实现时需要特殊考虑的负担。Alpha、PowerPC和LoongArch均没有采用转移延迟槽技术。

*也可以在分支指令后面加上一些不依赖分支的指令，这样可以最大化利用流水线

二进制翻译是一种跨指令软件兼容的技术，它把二进制软件代码从一种指令集翻译到另一种指令集以实现跨平台运行。新指令集发展过程中经常会利用这个技术来运行现有的软件，为新生态的建设实现平稳过渡。↩︎

![20211207132849](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207132849.png)

1）异常处理准备。当异常发生时，CPU在转而执行异常处理前，硬件需要进行一系列准备工作。

首先，需要记录被异常打断的指令的地址（记为EPTR）。这里涉及精确异常的概念，指发生任何异常时，被异常打断的指令之前的所有指令都执行完，而该指令之后的所有指令都像没执行一样。在实现精确异常的处理器中，异常处理程序可忽略因处理器流水线带来的异常发生位置问题。异常处理结束后将返回EPTR所在地址，重新执行被异常打断的指令[这只是通常的处理流程，但并非始终如此，存在某些异常处理场景，其结束后返回执行的并非最初被该异常打断的指令。例如，当执行SYSCALL指令而陷入系统调用异常处理时，肯定不能在处理结束后返回触发异常的SYSCALL指令，否则将陷入死循环。再譬如，当发生中断并陷入操作系统核心进行处理时，处理结束后，操作系统可能将其他进程或线程调度到该CPU上执行，显然此时返回执行的并不是最初被中断打断的那条指令。]，因此需要将EPTR记录下来。EPTR存放的位置因不同指令集而不同，LoongArch存于CSR.ERA[其实TLB重填异常发生时，这一信息将被记录在CSR.TLBRBERA中;机器错误异常发生时，这一信息将被记录在CSR.MERRERA中。更多细节请见下文中的说明。]，PowerPC存于SRR0/CSRR0，SPARC存于TPC[TL]，X86则用栈存放CS和EIP组合。

其次，调整CPU的权限等级（通常调整至最高特权等级）并关闭中断响应。在LoongArch指令系统中，当异常发生时，硬件会将CSR.PLV置0以进入最高特权等级，并将CSR.CRMD的IE域置0以屏蔽所有中断输入。

再次，硬件保存异常发生现场的部分信息。在LoongArch指令系统中，异常发生时会将CSR.CRMD中的PLV和IE域的旧值分别记录到CSR.PRMD的PPLV和PIE域中，供后续异常返回时使用。

最后，记录异常的相关信息。异常处理程序将利用这些信息完成或加速异常的处理。最常见的如记录异常编号以用于确定异常来源。在LoongArch指令系统中，这一信息将被记录在CSR.ESTAT的Ecode和EsubCode域，前者存放异常的一级编号，后者存放异常的二级编号。除此以外，有些情况下还会将引发异常的指令的机器码记录在CSR.BADI中，或是将造成异常的访存虚地址记录在CSR.BADV中。

2）确定异常来源。不同类型的异常需要各自对应的异常处理。处理器确定异常来源主要有两种方式：一种是将不同的异常进行编号，异常处理程序据此进行区分并跳转到指定的处理入口；另一种是为不同的异常指定不同的异常处理程序入口地址，这样每个入口处的异常处理程序自然知晓待处理的异常来源。X86由硬件进行异常和中断号的查询，根据编号查询预设好的中断描述符表（Interrupt Descriptor Table，简称IDT），得到不同异常处理的入口地址，并将CS/EIP等压栈。LoongArch将不同的异常进行编号，其异常处理程序入口地址采用“入口页号与页内偏移进行按位逻辑或”的计算方式，入口页号通过CSR.EENTRY配置，每个普通异常处理程序入口页内偏移是其异常编号乘以一个可配置间隔（通过CSR.ECFG的VS域配置）。通过合理配置EENTRY和ECFG控制状态寄存器中相关的域，可以使得不同异常处理程序入口地址不同。当然，也可以通过配置使得所有异常处理程序入口为同一个地址，但是实际使用中通常不这样处理。

3）保存执行状态。在操作系统进行异常处理前，软件要先保存被打断的程序状态，通常至少需要将通用寄存器和程序状态字寄存器的值保存到栈中。

4）处理异常。跳转到对应异常处理程序进行异常处理。

5）恢复执行状态并返回。在异常处理返回前，软件需要先将前面第3个步骤中保存的执行状态从栈中恢复出来，在最后执行异常返回指令。之所以要采用专用的异常返回指令，是因为该指令需要原子地完成恢复权限等级、恢复中断使能状态、跳转至异常返回目标等多个操作。在LoongArch中，异常返回的指令是ERTN，该指令会将CSR.PRMD的PPLV和PIE域分别回填至CSR.CRMD的PLV和IE域，从而使得CPU的权限等级和全局中断响应状态恢复到异常发生时的状态，同时该指令还会将CSR.ERA中的值作为目标地址跳转过去。X86的IRET指令有类似效果。

中断从系统中各个中断源传递到处理器主要有两种形式：中断线和消息中断。

用中断线传递是最简便直接的方式。当系统的中断源不多时，直接连到处理器引脚即可。若中断源较多，可使用中断控制器汇总后再与处理器引脚相连。由于连线会占用引脚资源，一般只在片上系统(System On Chip,简称SoC)中才会给每个外设连接单独的中断线，板级的中断线一般采用共享的方式。比如PCI上有四根中断线，供所有的设备共享。中断处理程序在定位到哪根中断线发生中断后，逐个调用注册在该中断线的设备中断服务。

用中断线方式传递中断有一些限制。首先是扩展性不够强，在搭建较复杂的板级系统时会引入过多的共享，降低中断处理的效率。其次，中断处理过程需要通过查询中断控制器以及设备上的状态寄存器来确认中断和中断原因，中间有较长的延迟，同样不利于提高效率。在多处理器平台中，高性能外设（如万兆网卡）对中断处理的性能有更高的要求，需要实现多处理器的负载均衡、中断绑定等功能，传统的中断线方式难以做到。而这正好是消息中断的长处。

消息中断以数据的方式在总线上传递。发中断就是向指定的地址写一个指定的数。相比总线外增加专门的中断线的“带外”（Side-Band)传输形式，消息中断在“带内”(In-Band）传输。增加中断时不需要改动消息传递的数据通路，因而有较高的扩展性和灵活性，也为更高程度的优化提供了可能。比如一个设备可以申请更多的中断号，使中断处理程序无须查询设备状态，只根据中断号就能知道应当做什么处理

![20211207142916](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207142916.png)

*ASID感觉像是进程ID，因为进程之间的地址空间是独立的

TLB中存储的内容包括虚拟地址、物理地址和保护位，可分别对应于Cache的Tag、Data和状态位

*这里我感觉不太一样，TLB中的保护位应该是指定了该页的权限，而Cache中的状态位标识了该条目是否可用

![20211207145633](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207145633.png)

![20211207152724](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207152724.png)

和malloc类似，用一个数据结构管理已分配的内存段。

猜想：条目中应该有当前映射到的虚拟地址，以便换入换出时对TLB进行快速的更新，设置有效位等

![20211207153106](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207153106.png)

*分发共享库的基础，比如libc.so，我们要使用就必须要保证有相同的ABI。这个应该是编译器做的？也可能取决于操作系统或分发商，比如GNU或者MS

![20211207154723](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207154723.png)

![20211207191237](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207191237.png)

![20211207191257](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207191257.png)

*位域这里有点奇怪

操作系统把当前进程的运行上下文信息保存到内存中，再把选中的下一个进程的上下文信息装载到CPU中。特定时刻只能由一个进程使用的处理器状态信息，包括通用寄存器、eflags等用户态的专有寄存器以及当前程序计数器（PC）、处理器模式和状态、页表基址（例如X86指令系统的CR3寄存器和LoongArch的PGD寄存器）等控制信息，都需要被保存起来，以便下次运行时恢复到同样的状态

*每个进程都有自己的页表，但是对于TLB来说是不变的。也就是说进程自己的页表维护了这个进程的虚拟地址到物理地址的映射。TLB中维护了进程的标识，这样就不需要在切换上下文时也刷新TLB了

![20211207200257](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207200257.png)

![20211207201237](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207201237.png)

*主要是同步多核，修改同一个地址，并用缓存一致性来保证多核之间读取到的结果相同。这样就可以保证同一时间只能有一个核修改成功

![20211207202104](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211207202104.png)

*和数据库中的并发控制一样，执行过程中检测冲突。但是这应该需要一个中心化的数据结构来检测冲突？否则有可能出现ABA问题？同时还要有abort的操作，回滚之前的修改等。

普通MMU为CPU提供物理内存的虚拟化，IOMMU则为外设提供物理内存的虚拟化，让外设访问内存时可通过虚实地址转换

*结构体对齐，其实只有一个原则，就是每个元素都认为内存是以他的大小来划分的。那么元素的地址就要和他自身的大小对齐。同时结构体的大小还要和结构体中最大的元素对齐，因为当结构体作为一个元素出现的时候，后面结构体中的成员也需要对齐，所以我们需要保证前面结构体本身就是对齐的。

![20211208132704](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208132704.png)

![20211208132714](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208132714.png)

![20211208132741](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208132741.png)

![20211208132823](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208132823.png)

![20211208133208](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208133208.png)

![20211208134314](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208134314.png)

![20211208134541](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208134541.png)

*IO控制设备应该就是南桥

![20211208134930](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208134930.png)

*集显为什么慢

![20211208135251](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208135251.png)

![20211208135623](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208135623.png)

![20211208140925](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208140925.png)

![20211208141056](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208141056.png)

![20211208141310](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208141310.png)

![20211208141438](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208141438.png)

*Soc 树莓派这样的

![20211208151134](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208151134.png)

![20211208152001](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208152001.png)

![20211208154940](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208154940.png)

![20211208180137](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208180137.png)

*当时树莓派就是只用了主核，其他核直接死循环挂起。几个核同样的执行这段初始化指令

![20211208180628](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208180628.png)

内存映射的情况下，不走缓存

![20211208191035](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208191035.png)

![20211208191201](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211208191201.png)

![20211209191319](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211209191319.png)

*这个妙

![20211209191441](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211209191441.png)

*保证不会出现竞争

![20211209194510](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211209194510.png)

![20211211154045](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211211154045.png)

*将若干个数的加法转化成`sum + (carry << 1)`。巧妙的地方在于在合并加法的同时，掩盖了进位的延迟

利用这一事实————上一级的进位就是这一级的加数。将进位和合并加法融合在了一起

比如计算32个32位加法

并行进位加法器相当于是以一个周期的延迟实现了两个32位加法

所以我们需要16个加法器在第一级，然后8个加法器在第二级...

而华莱士树则是纵向的优化

一个加法单元里是在若干个（log个）周期中实现的1位32个数的加法

最后利用32个加法单元可以将32个数相加转化成两个数相加

优点是没有等待进位的延迟，同时层级结构可以流水

相对于并行加法器的优点是门电路用的更少

![20211212090611](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212090611.png)

*实现精确异常

![20211212093229](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212093229.png)

*这里的逻辑是这样的

我们为了提高CPI，开辟多条车道，也就是多发射（超标量处理器）

但是由于指令执行速度快慢不同，导致流水线不能被很好的利用。所以用乱序执行

指令放到保留站中等待他依赖的结果准备完毕后再去执行

所以后面的指令就有可能先于前面的指令执行

就出现了WAR和WAW类型的冒险，其本质是因为不相关的利用了相同的寄存器，导致出现了假相关

用寄存器重命名的方法，找空闲的寄存器来执行。这样解决了假相关

乱序执行还有一个问题就是精确异常

所以通过ROB，重排序缓冲区来按序提交指令，在提交阶段处理异常即可

![20211212094028](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212094028.png)

![20211212100307](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212100307.png)

*隐式的数据同步很奇怪，我之前没有简单过这样的说法。编译器自动添加的barrier吗？

不对，应该按照SIMD的形式理解，一个指令就是多条数据，数据内部会同步。（因为这是单任务）

![20211212100611](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212100611.png)

*这个才是我们常见的并行模型，显式的进行同步

![20211212100704](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212100704.png)

*多进程的形式，同步要通过阻塞通信等形式完成（栅障是什么？）

![20211212101120](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212101120.png)

![20211212125542](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212125542.png)

![20211212130900](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212130900.png)

![20211212131949](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212131949.png)

![20211212132318](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212132318.png)

![20211212133759](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212133759.png)

*我们通常用的应该就是弱一致性

![20211212133912](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212133912.png)

*memory fence？貌似不是，这里好像只是对cache coheercy的优化

![20211212134230](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212134230.png)

*这个好像才是memory fence，保证访存和同步指令的执行顺序，通过同步指令来进行多核的同步

![20211212134449](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212134449.png)

*保证acquire和release包裹住访存操作，也就是保证访存操作能够被同步

![20211212134659](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212134659.png)

![20211212135239](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212135239.png)

*集中式的目录不是也会造成瓶颈么？只是优化了总线带宽

存储开销是因为需要为每一个存储条目都维护持有此行的处理器号

![20211212140724](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212140724.png)

![20211212140743](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212140743.png)

![20211212140755](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212140755.png)

![20211212140814](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212140814.png)

![20211212141013](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212141013.png)

![20211212141033](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212141033.png)

![20211212141054](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212141054.png)

![20211212141518](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212141518.png)

![20211212142838](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212142838.png)

*LL/SC更适合配合缓存一致性协议来实现

![20211212143250](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212143250.png)

![20211212143502](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212143502.png)

![20211212144013](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212144013.png)

*硬件的话只维护回退地址么？那写操作要怎么回退？

或者是在txn cache中维护需要回退的写信息

而且在内存中操作不像数据库那么复杂

![20211212150118](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212150118.png)

*SIMT，单个指令多个线程，对应不同的数据

![20211212151154](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212151154.png)

对于电路交换，当数据的传输速率及其突发性变化很大时，交换的控制就变得十分复杂;对于分组交换，当数据传输速率很高时，协议数据单元在各层的处理成为很大的开销，无法满足实时性很强的业务的时延要求。异步传输模式ATM(Asynchronous Transfer Mode)就是建立在电路交换和分组交换基础上的一种新的交换技术，他可以很好地进行宽带信息交换。 ATM信元格式及速率

*虚通道相关，就是提前分配通道，而不是等数据包来的时候再选择

链接更稳定

![20211212184430](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212184430.png)

![20211212184444](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20211212184444.png)