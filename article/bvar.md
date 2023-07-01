# brpc-bvar

这篇文章介绍一下bvar的实现

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

每次调用`take_sample`的时候，Sampler会根据`InvOp`的类型进行操作，InvOp代表BinaryOp的逆操作，如果有逆操作的话，这里会简单记录当前Reducer的值，并存储到Queue中，否则的话则会通过Reset重置Reducer的数据，并将重置之前的数据存储到Queue中。

这里的逻辑是：如果有逆操作的话，可以通过逆操作来计算两次Sample的Diff。否则的话，就只能存储每个TimeWindow（1s）所聚合的值。比如前5s到前3s的Max，就只能通过聚合5到4秒和4到3秒这两次的聚合值来得到，而如果是取Sum，则可以简单用第3秒的sum减去第5s的sum来得到

## bvar::PassiveStatus

## bvar::LatencyRecorder

