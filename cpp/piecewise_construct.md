# piecewise_construct

之前就被piecewise construct折磨住过。后来重新看一下发现其实在cppreference中写的很明白了

piecewise constuct实际上就是一个empty class tag，他的作用就是为函数添加一个可重载的参数，从而避免歧义

The overloads that do not use std::piecewise_construct_t assume that each tuple argument becomes the element of a pair. The overloads that use std::piecewise_construct_t assume that each tuple argument is used to construct, piecewise, a new object of specified type, which will become the element of the pair.

用了piecewise construct的重载会将传入的tuple拆开作为参数传入。

他所针对的场景主要是针对map，pair这样的构造函数

在我们通过`map.emplace(key, value)`插入键值对的时候，如果我们希望原地构建kv pair，而非传入两个const reference做拷贝的话。我们就需要在emplace中传入构造函数的参数。

比如`map.emplace(1, 2, 3, 4)`，这时候问题就很明显了，这四个参数，我们都不知道谁是用来构造key，谁是用来构造value的。

那么一个解决的思路就是通过tuple传入，比如`map.emplace(make_tuple(1, 2), make_tuple(3, 4))`

这时候另一个问题就出现了，两个参数传入的方式是通过tuple。即调用的是`key(std::tuple<int, int>)`，而非`key(int, int)`

假如我们同时含有这两种构造函数，这时候我们会调用第一个而非第二个。那我们要怎么才能在这种情况下（即key和value同时构造）调用第二种构造函数呢？

这就是piecewise construct的作用

```cpp
#include <iostream>
#include <utility>
#include <tuple>
 
struct Foo {
    Foo(std::tuple<int, float>) 
    {
        std::cout << "Constructed a Foo from a tuple\n";
    }
    Foo(int, float) 
    {
        std::cout << "Constructed a Foo from an int and a float\n";
    }
};
 
int main()
{
    std::tuple<int, float> t(1, 3.14);
    std::pair<Foo, Foo> p1(t, t);
    std::pair<Foo, Foo> p2(std::piecewise_construct, t, t);
}
```

Output:
```
Constructed a Foo from a tuple
Constructed a Foo from a tuple
Constructed a Foo from an int and a float
Constructed a Foo from an int and a float
```

可以看到在传入piecewise construct的时候，相当于是一个指示（flag），构造函数会将tuple拆开并将其包含的参数作为构造函数的参数传入。即所谓的piecewise，也就是将tuple解包，将tuple拆成一片一片的作为构造函数。