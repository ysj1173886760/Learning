# brpc-bvar

这篇文章介绍一下bvar的实现，之前有写过一些brpc的文章放到了我的博客中，有同学感兴趣也可以去看看：http://heavensheep.xyz/?s=brpc

有关bvar的实现和使用方法可以看这里：

https://github.com/apache/brpc/blob/master/docs/cn/bvar_c%2B%2B.md

https://github.com/apache/brpc/blob/master/docs/cn/bvar.md

因为我也没用过多维度的bvar，所以就只是读读单维度的bvar了

为了防止有同学不愿意点进去连接，我贴一下bvar的简单介绍：

> [bvar](https://github.com/apache/brpc/tree/master/src/bvar/)是多线程环境下的计数器类库，支持[单维度bvar](https://github.com/apache/brpc/blob/master/docs/cn/bvar_c++.md)和[多维度mbvar](https://github.com/apache/brpc/blob/master/docs/cn/mbvar_c++.md)，方便记录和查看用户程序中的各类数值，它利用了thread local存储减少了cache bouncing，相比UbMonitor(百度内的老计数器库)几乎不会给程序增加性能开销，也快于竞争频繁的原子操作。brpc集成了bvar，[/vars](https://github.com/apache/brpc/blob/master/docs/cn/vars.md)可查看所有曝光的bvar，[/vars/VARNAME](https://github.com/apache/brpc/blob/master/docs/cn/vars.md)可查阅某个bvar，在brpc中的使用方法请查看[vars](https://github.com/apache/brpc/blob/master/docs/cn/vars.md)。brpc大量使用了bvar提供统计数值，当你需要在多线程环境中计数并展现时，应该第一时间想到bvar。但bvar不能代替所有的计数器，它的本质是把写时的竞争转移到了读：读得合并所有写过的线程中的数据，而不可避免地变慢了。当你读写都很频繁或得基于最新值做一些逻辑判断时，你不应该用bvar。

bvar的应用场景就是写多读少，一般用来做监控。实现原理就是每个线程写thread local的变量，这样不会导致cacheline来回invalid，然后读取的时候聚合所有线程的结果即可。

这里介绍的流程也和上面文档中标识的相同，我会跳过一些不常用的bvar

## bvar::Variable

variable是所有bvar的父类，提供了expose/describe/hide/name这几个接口。

expose之后bvar会被注册到一个全局的表中，后续可以通过list等方式查询这个全局的表。

describe则是打印出当前bvar的状态，比如里面的值什么的，有点类似DebugString

expose/hide相关的就不多说了

## bvar::Reducer

Reducer表示的是将多个值Reduce成一个值，比如`Adder`,`Maxer`,`Miner`这些都是继承了Reducer，只不过是不同的聚合函数而已

对于每个bvar来说，我们主要关注的就是两个函数:

* `get_value()`，用于聚合所有线程的值并输出
* `operator<<(value)`，用于写入一个值

这里贴出Reducer的这两个函数作为示例：

```cpp
template <typename T, typename Op, typename InvOp>
inline Reducer<T, Op, InvOp>& Reducer<T, Op, InvOp>::operator<<(
    typename butil::add_cr_non_integral<T>::type value) {
    // It's wait-free for most time
    agent_type* agent = _combiner.get_or_create_tls_agent();
    agent->element.modify(_combiner.op(), value);
    return *this;
}
```

```cpp
T get_value() const {
    return _combiner.combine_agents();
}
```

get_value就是简单的调用了combiner的combine_agents，就是聚合所有线程的数据，等下我们会细看这个combiner

而写入接口则是先通过combiner获得一个tls的agent，然后将数据写入到agent的element中

对于Combiner来说，在bvar中使用的Combiner叫做AgentCombiner，每个AgentCombiner有一个自己的ID，每个线程在使用的时候，会有一个thread local的Block，每种不同类型的element对应一种Agent，每种Agent都会有自己的Thread local block。在访问的时候根据AgentCombiner的ID计算出对应的Agent在Thread local block中的位置，就可以访问这个Agent。

注意这里的Agent并没有进行Cacheline Alignment，原因是他不会和其他线程共享，也就不需要考虑false sharing什么的问题。

Agent中只会存储一个值，以及一个combiner的指针。这个值是通过一个叫做ElementContainer的结构包起来的，起作用是让element的访问变成线程安全的。具体来说，如果element是atomic类型的，那么在访问的时候会直接通过原子指令访问，否则的话，则会在每个element中都存储一个pthread mutex，然后在锁内访问数据。

AgentCombiner中还存储了一个global value，其作用主要是存储一些额外的数据。比如删除掉一个Agent的时候，可以将这个Agent的数据存到global value中。或者在Reduce的过程中发生了溢出，也会将溢出值存储到global value中

AgentCombiner提供的对数据操作的接口包括：

* combine_agents：聚合所有agent的数据
* reset_all_agents：聚合并清空所有agent的数据
* commit_and_erase：将给定agent删除，并将值存储到global value中
* commit_and_clear：将给定agent的数据写入到global value中，并清空该agent的数据
* clear_all_agents：删除所有agent

注意因为global value是全局共享的，所以AgentCombiner在操作他的时候会上一把锁

刚才提到了，不同类型的Reducer只是把BinaryOp的类型替换了而已，比如Adder就是加和，Maxer就是取max。

注意Reducer本身不会丢弃任何值，如果用户提供的BinaryOp不会丢失数据的话，那么Reducer就是精确的

## bvar::IntRecorder

IntRecorder的作用是统计平均值

其实现的基本思路就是把sum和num压到一个64位的整数中

```cpp
// Compressing format:
// | 20 bits (unsigned) | sign bit | 43 bits |
//       num                   sum
```

所以除了提供get value（作用是求sum），还提供了average，作用是求平均。

IntRecorder的写入和Reducer相比主要多了num的处理逻辑。在写入的时候虽然参数是`int64_t`，但是bvar会检查，如果值域超过了`int32_t`会被截断。

在写入的时候，如果num或者sum超过了这些位所能表达的最大值，这里会先将当前的Agent的结果提交到global value中，然后再去写入，从而避免丢失数据。

代码实现如下：

```cpp
    uint64_t n;
    agent->element.load(&n);
    const uint64_t complement = _get_complement(sample);
    uint64_t num;
    uint64_t sum;
    do {
        num = _get_num(n);
        sum = _get_sum(n);
        if (BAIDU_UNLIKELY((num + 1 > MAX_NUM_PER_THREAD) ||
                           _will_overflow(_extend_sign_bit(sum), sample))) {
            agent->combiner->commit_and_clear(agent);
            sum = 0;
            num = 0;
            n = 0;
        }
    } while (!agent->element.compare_exchange_weak(
                 n, _compress(num + 1, sum + complement)));
```

## bvar::Window

在介绍Window之前，我们需要先介绍一下Sampler，因为Window只是相当于对Sampler的一些Wrap，并没有很多实质的逻辑

Sampler的作用就是周期性的进行采样，接口如下：

```cpp
class Sampler : public butil::LinkNode<Sampler> {
public:
    Sampler();
    // This function will be called every second(approximately) in a
    // dedicated thread if schedule() is called.
    virtual void take_sample() = 0;
    // Register this sampler globally so that take_sample() will be called
    // periodically.
    void schedule();
    // Call this function instead of delete to destroy the sampler. Deletion
    // of the sampler may be delayed for seconds.
    void destroy();
};
```

Schedule就是将Sampler提交上去，然后`take_sample()`就会每秒调用一次，相当于是注册一个定时器定时调用callback

这里用Reducer的Sampler做ReducerSampler为例子

ReducerSampler中的核心结构就是一个`butil::BoundedQueue`，和普通队列的区别就是在push的时候，如果队列数量超过上限，就会truncate老的元素。

每次调用`take_sample`的时候，Sampler会根据`InvOp`的类型进行操作，InvOp代表BinaryOp的逆操作，如果有逆操作的话，这里会简单记录当前Reducer的值，并存储到Queue中，否则的话则会通过`reset`重置Reducer的数据，并将重置之前的数据存储到Queue中。

这里的逻辑是：如果有逆操作的话，可以通过逆操作来计算两次Sample的Diff。否则的话，就只能存储每个TimeWindow（1s）所聚合的值。比如前5s到前3s的Max，就只能通过聚合5到4秒和4到3秒这两次的聚合值来得到，而如果是取Sum，则可以简单用第3秒的sum减去第5s的sum来得到

代码如下：

```cpp
Sample<T> latest;
if (butil::is_same<InvOp, VoidOp>::value) {
    latest.data = _reducer->reset();
} else {
    latest.data = _reducer->get_value();
}
latest.time_us = butil::gettimeofday_us();
_q.elim_push(latest);
```

Sampler提供一个`get_value()`的方法，作用就是取出队列中至多`n`个元素，如果有InvOp的话，这里会直接通过InvOp计算第一个和最后一个采样结果的差值，否则的话则会通过BinaryOp将这n个元素聚合起来返回

介绍完Sampler，Window上面也提到过，就是对Sampler的包装，Window的构造函数中需要给出Window聚合的粒度，以秒为单位。Window的`get_value`实际上就是直接调用Sampler的`get_value`，即聚合给定窗口内的数据。

还有另一个和Window比较像的Wrapper，叫做PerSecond，他和Window的区别是，PerSecond在聚合完结果后，会用结果除以时间，从而得到每秒钟的变化量。举个例子：

```cpp
bvar::Adder<int> sum;
bvar::PerSecond<bvar::Adder<int>> sum_per_second(&sum, 60);
bvar::Window<bvar::Adder<int>> sum_minute(&sum, 64);
```

一个是统计60秒的总和，一个是统计60秒内，每秒钟的变化量

## bvar::PassiveStatus

bvar提供一个叫Status的东西，主要作用就是`set_value`，用来统计一个恒定的值

因为在某些情况下，我们不知道何时去执行`set_value`，所以bvar引入了PassiveStatus，会传入一个callback，只有在需要输出的时候，bvar才会通过callback计算需要输出的值。在下面的LatencyRecorder中会看到具体的应用

## bvar::LatencyRecorder

LatencyRecorder应该是bvar最常用的一个结构了，他用来存储延迟，并提供qps/latency_avg/latency_p99等多个统计值

为了统计p99等信息，LatencyRecorder需要一个结构来记录数据分布，这个也是在读LatencyRecorder之前所需要了解的最后一个结构，叫做Percentile

Percentile中也包含一个combiner和一个sampler，其中Combiner就是之前看到的AgentCombiner，而Sampler也是之前看到的ReducerSampler，唯一有不同的就是Element的类型不同，这里的Element为PercentileSamples

每个PercentileSamples包含32个PercentileInterval，每个PercentileInterval包含若干个Sample，每个Sample就是用户给的Latency，Sample的数量由PercentileInterval的模版类型所指定（减少动态内存分配次数）

之所以每个PercentileSamples包含32个PercentileInterval，是因为LatencyRecorder里通过`uint32_t`来记录Latency，并且会根据`log(latency)`的值来计算桶的位置，所以就最多需要32个桶。

写入逻辑如下：

```cpp
void operator()(GlobalValue<Percentile::combiner_type>& global_value,
                ThreadLocalPercentileSamples& local_value) const {
    int64_t latency = _latency;
    const size_t index = get_interval_index(latency);
    PercentileInterval<ThreadLocalPercentileSamples::SAMPLE_SIZE>&
        interval = local_value.get_interval_at(index);
    if (interval.full()) {
        GlobalPercentileSamples* g = global_value.lock();
        g->get_interval_at(index).merge(interval);
        g->_num_added += interval.added_count();
        global_value.unlock();
        local_value._num_added -= interval.added_count();
        interval.clear();
    }
    interval.add64(latency);
    ++local_value._num_added;
}
```

即先根据latency计算出桶的位置，也就是定位具体的Interval，如果该Interval已经满了（因为不会涉及动态内存分配，所以Interval是定长的），就会将该Interval中的采样信息合并到全局的采样数据中，然后将本次数据加入到interval中。

这里在将局部数据Merge到全局数据的时候，如果全局数据的空间也不足了，会保证老数据的数量一定是会变少的，即用新的数据来替换旧的数据，否则相当于只是将新数据采样加入进来，这样老数据会被一直保留。

然后另一个值的关注的函数就是`get_number`了，他的作用是get_pxx，比如获取p50，p99等。因为我们已经有Latency的统计数据了，这里就是计算一下我们需要获得第几个latency作为结果输出。之前看到的`_num_added`等信息也是在这个地方用到的。

其实我个人直观感觉用`_num_sampled`也不是不行，不过这块我也不太懂

最后计算出是第几个Interval的第几个Sample，然后通过`interval.get_sample_at(index)`读取数据，里面会判断一下，如果数据是unsorted，就会先进行一下sort再读取。

这里的sorted其实也只是一个大概的样子，因为在记录latency的时候，并不会维护这个`_sorted`量，`merge`的时候也不会。只有在clear interval，即从局部merge到全局的时候才会清空一下。

Percentile结束后，看LatencyRecorder就比较容易了，核心就是三个计数器，以及对应的三个Window：

```cpp
class LatencyRecorderBase {
public:
    explicit LatencyRecorderBase(time_t window_size);
protected:
    IntRecorder _latency;
    Maxer<int64_t> _max_latency;
    Percentile _latency_percentile;
  
    RecorderWindow _latency_window;
    MaxWindow _max_latency_window;
    PercentileWindow _latency_percentile_window;
}
```

其中IntRecorder的作用是统计qps和平均值，因为IntRecorder中记录了sum和num

MaxWindow则是统计窗口内延迟最大值

PercentileWindow则是用来统计p99等信息的，这里还有一些PassiveStatus用来打印出默认的一些统计值

嗯LatencyRecorder差不多就这些

本来还想画个图梳理一下，但是实际上这么看bvar还是比较清晰的，每个bvar就是一个combiner一个sampler，然后不同bvar有不同类型的数据和聚合方式。

这篇文章没有提到一个叫做SeriesSampler的东西，在阅读代码的时候可能会常看到，他的作用是统计过去30天的数据，实现也比较简单，因为我用的不多这里就不说了。

