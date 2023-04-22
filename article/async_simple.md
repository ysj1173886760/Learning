# async_simple Coroutine

简单看看async simple的coroutine的实现。

async_simple的coroutine是通过`Lazy<T>`实现的，即每一个Coroutine的返回值都是`Lazy<T>`，然后我们可以通过`co_await Lazy`来等待一个coroutine的返回值。

所以这里要关注的有两个点，一个是Lazy是怎么开始执行一个coroutine的，即关注Lazy的`promise_type`，另一个就是在coroutine内去co_await一个Lazy是怎么实现的，即关注Lazy的`Awaiter`。

`Lazy`继承`LazyBase`，定义了promise_type的类型：

```cpp
using promise_type = detail::LazyPromise<T>;
```

在LazyPromise中保存了三个值，分别是：

```c++
std::coroutine_handle<> _continuation;
Executor* _executor;
std::variant<std::monostate, T, std::exception_ptr> _value;
```

_continuation是其父coroutine的handle，用来恢复其父coroutine的执行。

_executor就是当前coroutine要执行的位置。

_value则是保存的值，在最后co_return的时候，会通过`promise.return_value`来将值保存在这里面，然后在await_resume的时候就会将值传出去。

```cpp
std::suspend_always initial_suspend() noexcept { return {}; }
FinalAwaiter final_suspend() noexcept { return {}; }
```

按照推荐的，initial suspend总是会挂起。

```cpp
struct FinalAwaiter {
  bool await_ready() const noexcept { return false; }
  template <typename PromiseType>
  auto await_suspend(std::coroutine_handle<PromiseType> h) noexcept {
    return h.promise()._continuation;
  }
  void await_resume() noexcept {}
};
```

FinalAwaiter也是一样，总是会挂起，并在suspend的时候恢复其父coroutine的执行。

在coroutine里co_await一个Awaitable的时候，如果对应的promise实现了await_transform，就会调用promise.await_transform。这里返回的就是Awaiter

```cpp
template <typename Awaitable>
auto await_transform(Awaitable&& awaitable) {
  return detail::coAwait(_executor, std::forward<Awaitable>(awaitable));
}
```

里面的实现是：

1. 如果这个awaitable有coAwait这个方法的话，就会调用awaitable.coAwait(executor)。
2. 否则返回`ViaAsyncAwaiter(executor, awaitable)`，里面会先按照标准所述的方法，通过Awaitable的operator co_await得到对应的awaiter，然后将Awaiter和executor都保存下来。这里还会初始化一个ViaCoroutine，这个稍微复杂点，等下再看。

在我们`co_await`一个Lazy的时候，就会走上面的第一个路径，即调用awaitable.coAwait。

```cpp
auto coAwait(Executor* ex) {
  this->_coro.promise()._executor = ex;
  return typename Base::ValueAwaiter(std::exchange(this->_coro, nullptr));
}
```

Lazy中只保存了当前coroutine的handle，这里Lazy的初始化是promise.get_return_object

```cpp
template <typename T>
inline Lazy<T> detail::LazyPromise<T>::get_return_object() noexcept {
  return Lazy<T>(Lazy<T>::Handle::from_promise(*this));
}
```

回到上面看，coAwait就是设置这个Lazy的executor，这里是继承了调用者的executor。

可以这么想，假设我们当前是在父Coroutine中，有Promise1，然后我们尝试co_await一个返回Lazy的expr，首先这里会eval expr，返回对应的Lazy，这里Lazy对应的是Promise2，然后会调用Promise1.await_transform(Lazy)，接着调用Lazy.coAwait(Promise1.executor)，即获取Lazy的promise，也就是Promise2，将其executor设为Promise1的executor。

```cpp
AS_INLINE auto await_suspend(
  std::coroutine_handle<> continuation) noexcept(!reschedule) {
  // current coro started, caller becomes my continuation
  this->_handle.promise()._continuation = continuation;

  return awaitSuspendImpl();
}

auto awaitSuspendImpl() noexcept(!reschedule) {
  if constexpr (reschedule) {
    // executor schedule performed
    auto& pr = this->_handle.promise();
    logicAssert(pr._executor, "RescheduleLazy need executor");
    pr._executor->schedule(
      [h = this->_handle]() mutable { h.resume(); });
  } else {
    return this->_handle;
  }
}
```

Lazy返回的Awaiter的await_suspend，比如在我们co_await一个Lazy的时候，就会得到这个ValueAwaiter，然后会调用这个await_suspend将当前coroutine挂起，并开始执行子coroutine。

Awaiter中保存了子coroutine的handle。这里会先将子coroutine的continuation设置为当前挂起点。然后判断，对于reschedule的情况，即coroutine的执行要切换线程，这里会将子coroutine的handle丢到线程池中去执行。否则则会直接返回子coroutine的handle，即原地执行子coroutine。

注意到在初始化ValueAwaiter的时候，实际上将co_await Lazy的返回值中的handle已经换走了，因为在上面的子coroutine执行结束后，我们会调用await_resume，这里会将该子coroutine的执行结果返回，并将该handle destroy掉，这样该coroutine就执行完毕了。而之所以exchange掉那个Lazy中的handle，是因为他只是一个临时的Awaitable，我们会立刻使用它的Awaiter，而不会用Awaitable自己。

回过头来看刚才的coAwait，在没实现coAwait方法的Awaitable上，我们会构造一个ViaAsyncAwaiter。

ViaAsyncAwaiter的作用就是让一个任意类型的Awaiter有可以在executor中执行的能力。

```cpp
template <typename Awaiter>
struct [[nodiscard]] ViaAsyncAwaiter {
    template <typename Awaitable>
    ViaAsyncAwaiter(Executor * ex, Awaitable && awaitable)
        : _ex(ex),
          _awaiter(detail::getAwaiter(std::forward<Awaitable>(awaitable))),
          _viaCoroutine(ViaCoroutine::create(ex)) {}

    using HandleType = std::coroutine_handle<>;
    using AwaitSuspendResultType = decltype(
        std::declval<Awaiter&>().await_suspend(std::declval<HandleType>()));
    bool await_ready() { return _awaiter.await_ready(); }

    AwaitSuspendResultType await_suspend(HandleType continuation) {
        if constexpr (std::is_same_v<AwaitSuspendResultType, bool>) {
            bool should_suspend = _awaiter.await_suspend(
                _viaCoroutine.getWrappedContinuation(continuation));
            if (should_suspend == false) {
                _viaCoroutine.checkin();
            }
            return should_suspend;
        } else {
            return _awaiter.await_suspend(
                _viaCoroutine.getWrappedContinuation(continuation));
        }
    }

    auto await_resume() { return _awaiter.await_resume(); }

    Executor* _ex;
    Awaiter _awaiter;
    ViaCoroutine _viaCoroutine;
};  // ViaAsyncAwaiter
```

这里的ViaCoroutine就是一个空的coroutine，我们可以通过getWrappedContinuation来保存当前的continuation到ViaCoroutine中。

```cpp
template <typename PromiseType>
auto await_suspend(std::coroutine_handle<PromiseType> h) noexcept {
  auto& pr = h.promise();
  // promise will remain valid across final_suspend point
  if (pr._ex) {
    std::function<void()> func = [&pr]() {
      pr._continuation.resume();
    };
    pr._ex->checkin(func, _ctx);
  } else {
    pr._continuation.resume();
  }
}
```

在ViaCoroutine的final suspend的时候，这里会将用户的awaiter放到executor中去resume。

并且额外的一点是在suspend当前coroutine的时候，我们会通过executor记录当前continuation在executor中的执行线程ID，然后在恢复continuation的时候，我们会根据当时记录的线程ID，以及continuation一起丢入线程池，这样continuation的执行就不是跨线程的了。

最后看一下coroutine的起始点`start`

```cpp
template <typename F>
void start(F&& callback) requires(std::is_invocable_v<F&&, Try<T>>) {
  // callback should take a single Try<T> as parameter, return value will
  // be ignored. a detached coroutine will not suspend at initial/final
  // suspend point.
  auto launchCoro = [](LazyBase lazy,
                       std::decay_t<F> cb) -> detail::DetachedCoroutine {
    cb(co_await lazy.coAwaitTry());
  };
  [[maybe_unused]] auto detached =
    launchCoro(std::move(*this), std::forward<F>(callback));
}
```

一般用户的用法就是`Lazy<T>.via(executor).start()`

注意这里的LaunchCoro中使用了co_await，所以他也是一个coroutine，我们会根据返回值和参数判断promise的类型，这里是DetachedCoroutine

```cpp
struct DetachedCoroutine {
    struct promise_type {
        std::suspend_never initial_suspend() noexcept { return {}; }
        std::suspend_never final_suspend() noexcept { return {}; }
        void return_void() noexcept {}
        void unhandled_exception() {
            try {
                std::rethrow_exception(std::current_exception());
            } catch (const std::exception& e) {
                fprintf(stderr, "find exception %s\n", e.what());
                fflush(stderr);
                std::rethrow_exception(std::current_exception());
            }
        }
        DetachedCoroutine get_return_object() noexcept {
            return DetachedCoroutine();
        }

        // Hint to gdb script for that there is no continuation for
        // DetachedCoroutine.
        std::coroutine_handle<> _continuation = nullptr;
    };
};
```

DetachedCoroutine啥也没干，就是一个空的coroutine，并且不会在initial suspend和final suspend中挂起。所以在执行这个lambda的时候我们会原地执行。

Lazy在指定了executor后就变成了RescheduleLazy。这里我们会先co_await这个RescheduleLazy。

这里的Awaiter是TryAwaiter，其实和ValueAwaiter没什么区别，就不看了。

TryAwaiter中拿到的continuation就是用户传入的Lazy的initial suspend点，所以这里我们会在co await的时候将LaunchCoro这个lambda挂起，然后通过executor执行用户的Lazy。

在用户的Lazy执行完后，在executor的线程中，会resume回来最初始的LaunchCoro，然后调用callback。

