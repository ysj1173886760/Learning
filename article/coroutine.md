# C++ Coroutines

这个文章算是一个个人的笔记，想简单记录一下Coroutine相关的细节，以便后续回忆。如果想从头学习coroutine的话，我更推荐这个[系列](https://lewissbaker.github.io/)，本文也是从这个系列中摘取一些关键点记录而已。

## Awaiter

A type that supports the `co_await` operator is called an **Awaitable** type.

An **Awaiter** type is a type that implements the three special methods that are called as part of a `co_await` expression: `await_ready`, `await_suspend` and `await_resume`.

在调用`co_await expr`的时候，我们会先尝试通过expr获取一个awaitable，然后在通过这个awaitable获取一个awaiter。编译器大概翻译的代码如下

```cpp
template<typename P, typename T>
decltype(auto) get_awaitable(P& promise, T&& expr)
{
  if constexpr (has_any_await_transform_member_v<P>)
    return promise.await_transform(static_cast<T&&>(expr));
  else
    return static_cast<T&&>(expr);
}

template<typename Awaitable>
decltype(auto) get_awaiter(Awaitable&& awaitable)
{
  if constexpr (has_member_operator_co_await_v<Awaitable>)
    return static_cast<Awaitable&&>(awaitable).operator co_await();
  else if constexpr (has_non_member_operator_co_await_v<Awaitable&&>)
    return operator co_await(static_cast<Awaitable&&>(awaitable));
  else
    return static_cast<Awaitable&&>(awaitable);
}
```

在得到Awaiter之后，`co_await awaiter`就会被转化为：

```cpp
{
  auto&& value = <expr>;
  auto&& awaitable = get_awaitable(promise, static_cast<decltype(value)>(value));
  auto&& awaiter = get_awaiter(static_cast<decltype(awaitable)>(awaitable));
  if (!awaiter.await_ready())
  {
    using handle_t = std::experimental::coroutine_handle<P>;

    using await_suspend_result_t =
      decltype(awaiter.await_suspend(handle_t::from_promise(p)));

    <suspend-coroutine>

    if constexpr (std::is_void_v<await_suspend_result_t>)
    {
      awaiter.await_suspend(handle_t::from_promise(p));
      <return-to-caller-or-resumer>
    }
    else
    {
      static_assert(
         std::is_same_v<await_suspend_result_t, bool>,
         "await_suspend() must return 'void' or 'bool'.");

      if (awaiter.await_suspend(handle_t::from_promise(p)))
      {
        <return-to-caller-or-resumer>
      }
    }

    <resume-point>
  }

  return awaiter.await_resume();
}
```

At the `<suspend-coroutine>` point the compiler generates some code to save the current state of the coroutine and prepare it for resumption. This includes storing the location of the `<resume-point>` as well as spilling any values currently held in registers into the coroutine frame memory.

At the `<return-to-caller-or-resumer>` point execution is transferred back to the caller or resumer, popping the local stack frame but keeping the coroutine frame alive.

在将Coroutine挂起后，传给await suspend的参数是coroutine_handle，我们可以通过`coroutine_handle.resume()`来恢复coroutine的执行到`<resume-point>`，而`.resumt()`会在Coroutine下一次到达`<return-to-caller-or-resumer>`的时候返回。



