# async_simple Future/Promise 概览

简单看看async_simple的Future/Promise。注意这里不会关注coroutine/uthread部分

Future/Promise这块和folly，以及seastar的接口都是类似的，之后可以再写一下他们实现上的区别。

简单来说，就是Promise可以获取一个对应的Future，对应生产者和消费者。然后Future可以通过`then(Func &&func)`来创建一个continuation，表示在这个future获取到结果后，就会将future的结果传给continuation。这样在写代码的时候就可以通过`then`来将回调串起来，可读性就更强一些。

大概比对一下就是这样的

```cpp
// callback
void A(ParamA pa, CallbackA cb);
void B(ParamB pb, CallbackB cb);
void C(ParamC pc);

A(pa, [](ParamB pb){ B(pb, [](ParamC pc) { C(pc)}); });

// future/promise/continuation
Future<ParamB> A(ParamA pa);
Future<ParamC> B(ParamB pb);
void C(ParamC pc);
A(pa).then([](ParamB pb) { return B(pb); }).then([](ParamC pc) { C(pc); })

```

所以其实只是不同的写法而已，如果写Callback流的作者对代码组织的比较好的话，也是可以达到类似的效果的，不过也要看这个自动机的复杂程度了。

这个库提供的主体就是Future/Promise，以及一些辅助的工具，比如Try，有点类似`StatusOr<T>`这样，然后还有Executor，表示设置Callback的执行位置。

比如可以这样写

```cpp
A(pa).via(executorB).then([](ParamB pb) { return B(pb); }).via(executorC).then([](ParamC pc) { C(pc); })
```

这样可以让B执行在executorB上，让C执行在executorC上。

然后看一下实现

```cpp
class Promise {
    using value_type = std::conditional_t<std::is_void_v<T>, Unit, T>;
    FutureState<value_type>* _sharedState = nullptr;
    bool _hasFuture = false;
};
```

Promise的构造函数会构造一个FutureState，FutureState内部自己记录了自己的refcnt。

对于Void类型的Promise，async_simple搞了一个空的类型叫Unit。

```cpp
Future<T> getFuture() {
  logicAssert(valid(), "Promise is broken");
  logicAssert(!_hasFuture, "Promise already has a future");
  _hasFuture = true;
  return Future<T>(_sharedState);
}
```

getFuture的作用就是将`_hasFuture`设为true，然后用`FutureState`初始化一个Future。这样这一对Future/Promise就通过`_sharedState`链接起来了。

```cpp
void setValue(value_type&& v) requires(!std::is_void_v<T>) {
  logicAssert(valid(), "Promise is broken");
  _sharedState->setResult(Try<value_type>(std::forward<T>(v)));
}
```

Promise中的大多数操作都是直接操作的`_sharedState`，因为Promise的操作都是对Future有影响的，所以也就需要操作他们共享的`FutureState`

```cpp
template <typename T>
class FutureState {
    using Continuation = std::function<void(Try<T>&& value)>;
    std::atomic<detail::State> _state;
    std::atomic<uint8_t> _attached;
    std::atomic<uint8_t> _continuationRef;
    Try<T> _try_value;
    union {
        Continuation _continuation;
    };
    Executor* _executor;
    Executor::Context _context;
    std::atomic<std::size_t> _promiseRef;
    bool _forceSched;
};
```

看一下FutureState，`_attached`，`_continuationRef`以及`_promiseRef`就是他的引用计数，引用计数到0的时候就会把自己析构掉。

`_try_value`就是这个Future的返回值。`_executor`是该Future的Continuation的执行器。`_context`是执行器相关的东西，不用太关注。

这里有个union的`_continuation`，其作用是不会主动的做构造，因为不是每个Future都有Continuation。

核心的函数主要有三个：

```cpp
template <typename F>
void setContinuation(F&& func) {
  logicAssert(!hasContinuation(),
              "FutureState already has a continuation");
  MoveWrapper<F> lambdaFunc(std::move(func));
  new (&_continuation) Continuation([lambdaFunc](Try<T>&& v) mutable {
    auto& lambda = lambdaFunc.get();
    lambda(std::forward<Try<T>>(v));
  });

  auto state = _state.load(std::memory_order_acquire);
  switch (state) {
    case detail::State::START:
      if (_state.compare_exchange_strong(
        state, detail::State::ONLY_CONTINUATION,
        std::memory_order_release)) {
        return;
      }
      // state has already transferred, fallthrough
      assert(_state.load(std::memory_order_relaxed) ==
             detail::State::ONLY_RESULT);
    case detail::State::ONLY_RESULT:
      if (_state.compare_exchange_strong(state, detail::State::DONE,
                                         std::memory_order_release)) {
        scheduleContinuation(true);
        return;
      }
    default:
      logicAssert(false, "State Transfer Error");
  }
}
```

在刚才提到的union上构造一个Continuation，然后去CAS当前state。这个state有四种状态，分别是START，ONLY_CONTINUATION，ONLY_RESULT，DONE。

如果当前为START，说明目前还没有SetValue，那么就直接返回。如果是ONLY_RESULT，说明目前已经SetValue了，那么就需要我们去手动触发Continuation的调度，所以这里调用了`scheduleContinuation`

```cpp
void setResult(Try<T>&& value) {
  _try_value = std::move(value);

  auto state = _state.load(std::memory_order_acquire);
  switch (state) {
    case detail::State::START:
      if (_state.compare_exchange_strong(state,
                                         detail::State::ONLY_RESULT,
                                         std::memory_order_release)) {
        return;
      }
      // state has already transfered, fallthrough
      assert(_state.load(std::memory_order_relaxed) ==
             detail::State::ONLY_CONTINUATION);
    case detail::State::ONLY_CONTINUATION:
      if (_state.compare_exchange_strong(state, detail::State::DONE,
                                         std::memory_order_release)) {
        scheduleContinuation(false);
        return;
      }
    default:
      logicAssert(false, "State Transfer Error");
  }
}
```

这里也是去CAS state，如果是先变成ONLY_RESULT，那么调度Continuation的责任就是调用`setContinuation`的人。如果已经设置了Continuation，那么setResult的人就要负责调用`scheduleContinuation`。

```cpp
void scheduleContinuation(bool triggerByContinuation) {
  logicAssert(
    _state.load(std::memory_order_relaxed) == detail::State::DONE,
    "FutureState is not DONE");
  if (!_forceSched && (!_executor || triggerByContinuation ||
                       currentThreadInExecutor())) {
    // execute inplace for better performance
    ContinuationReference guard(this);
    _continuation(std::move(_try_value));
  } else {
    ContinuationReference guard(this);
    ContinuationReference guardForException(this);
    try {
      bool ret;
      if (Executor::NULLCTX == _context) {
        ret = _executor->schedule(
          [fsRef = std::move(guard)]() mutable {
            auto ref = std::move(fsRef);
            auto fs = ref.getFutureState();
            fs->_continuation(std::move(fs->_try_value));
          });
      } else {
        ScheduleOptions opts;
        opts.prompt = !_forceSched;
        // schedule continuation in the same context before
        // checkout()
        ret = _executor->checkin(
          [fsRef = std::move(guard)]() mutable {
            auto ref = std::move(fsRef);
            auto fs = ref.getFutureState();
            fs->_continuation(std::move(fs->_try_value));
          },
          _context, opts);
      }
      if (!ret) {
        throw std::runtime_error(
          "schedule continuation in executor failed");
      }
    } catch (std::exception& e) {
      // reschedule failed, execute inplace
      _continuation(std::move(_try_value));
    }
  }
}
```

这里有一个选项，可以尝试原地执行Continuation。这里会避免在SetResult的时候做Inplace Execution，我怀疑可能是因为SetResult可能是在一些其他不受控制的Callback中，比如libaio，在里面做Inplace Execution会影响他的Callback线程。

然后就调`_executor->schedule()`来执行Continuation，这里会把FutureState引用住，然后调他的Continuation。

最后就是看一下Future的实现了。最简单的一个就是`via()`，就是会把Executor赋给刚才FutureState中的Executor，用来指定当前Future的Continuation的Executor是谁。

```cpp
// continuation returns a future
template <typename F, typename R>
Future<typename R::ReturnsFuture::Inner> thenImpl(F&& func) {
  logicAssert(valid(), "Future is broken");
  using T2 = typename R::ReturnsFuture::Inner;

  Promise<T2> promise;
  auto newFuture = promise.getFuture();
  newFuture.setExecutor(_sharedState->getExecutor());
  _sharedState->setContinuation(
    [p = std::move(promise),
     f = std::forward<F>(func)](Try<T>&& t) mutable {
      if (!R::isTry && t.hasError()) {
        p.setException(t.getException());
      } else {
        if constexpr (R::ReturnsFuture::value) {
          try {
            auto f2 = f(std::move(t));
            f2.setContinuation(
              [pm = std::move(p)](Try<T2>&& t2) mutable {
                pm.setValue(std::move(t2));
              });
          } catch (...) {
            p.setException(std::current_exception());
          }
        } else {
          p.setValue(makeTryCall(std::forward<F>(f),
                                 std::move(t)));  // Try<Unit>
        }
      }
    });
  return newFuture;
}
```

这里会先创建一对Future/Promise，然后将当前Future的Continuation设置为一个Lambda，然后返回新的Future。

这个Lambda会在当前Future的值被填充好后调用，他会得到当前Future的值，然后判断传入的Continuation的类型。如果下一个Continuation的返回值是普通的值（而非Future），那么他会填充刚才新创建的Promise，这样返回的Future就Ready了。

如果下一个Continuation的返回值是Future，我们就会先执行f，来得到这个新的Future，然后将这个新的Future的Continuation设置成给刚才的Promise填充值，这样当新的Future Ready之后，返回的Future也就Ready了。

对于Continuation的返回值是普通值的图示大概是这样：

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230416154255.png)

而对于Continuation的返回值是Future的图示大概是这样

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230416154843.png)

```cpp
void wait() {
  if (hasResult()) {
    return;
  }

  // wait in the same executor may cause deadlock
  assert(!currentThreadInExecutor());

  // The state is a shared state
  Promise<T> promise;
  auto future = promise.getFuture();

  _sharedState->setExecutor(
    nullptr);  // following continuation is simple, execute inplace
  std::mutex mtx;
  std::condition_variable cv;
  std::atomic<bool> done{false};
  _sharedState->setContinuation(
    [&mtx, &cv, &done, p = std::move(promise)](Try<T>&& t) mutable {
      std::unique_lock<std::mutex> lock(mtx);
      p.setValue(std::move(t));
      done.store(true, std::memory_order_relaxed);
      cv.notify_one();
    });
  std::unique_lock<std::mutex> lock(mtx);
  cv.wait(lock,
          [&done]() { return done.load(std::memory_order_relaxed); });
  *this = std::move(future);
  assert(_sharedState->hasResult());
}
```

这里Wait的实现我个人感觉还挺巧妙的，就是将Continuation设置成对于CV的唤醒，然后让当前线程睡在CV上。感觉是一个对Continuation的灵活运用了。

![](https://picsheep.oss-cn-beijing.aliyuncs.com/pic/20230416155743.png)

连续的`then`调用的链路大概就是这样的，用户拿到的是最下面的future。

个人感觉这里和seastar实现的还是有点不同的，这里并没有做Promise/Future的Detach，而是通过一对新的Future/Promise，让这个嵌套的Future在Ready的时候可以填充新生成的Future/Promise。我感觉async_simple实现的更优雅些，不过seastar那个是单线程的，并且性能方面可能也有差异。

