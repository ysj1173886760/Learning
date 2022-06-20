# Rust ODT

ODT主要处理的是包含区间赋值的问题。以前的名字叫Old Driver Tree。核心思路就是通过随机数据的情况下，区间赋值的操作可以合并很多区间为一个整块。我们通过三元组的形式，即left，right，value来表示区间。在查询的时候就只查询范围内的三元组。由于区间赋值可以合并区间，所以总共要查询的数量很少。

可以从这道[题](https://leetcode.cn/problems/range-module/)来了解这个数据结构

ODT的核心就是两个函数。一个是split，将我们维护的区间中在一个点处断开。还有一个是assign，即区间赋值

```rust
fn split(&mut self, pos: i32) {
    use std::ops::Bound::*;
    let tmp = Node { l: pos, r: 0, v: 0 };
    if let Some(node) = self.set.range((Included(&tmp), Unbounded)).next() {
        if node.l == pos {
            return;
        }
    }
    let it = self.set.range((Unbounded, Excluded(&tmp))).next_back().unwrap();
    let &Node{ l, r, v } = it;
    self.set.remove( &Node { l: l, r: r, v: v });
    self.set.insert( Node { l: l, r: pos - 1, v: v});
    self.set.insert( Node { l: pos, r: r, v: v });
}
```

先通过lowerbound查找这个点是不是已经被断开了。如果是直接返回就行。

否则的话我们找到包含这个点的那个区间，将它删除，并插入断开后的区间。

```rust
fn assign(&mut self, l: i32, r: i32, v: i32) {
    use std::ops::Bound::*;
    self.split(r + 1);
    self.split(l);
    let tmp_l = Node { l: l, r: 0, v: 0 };
    let tmp_r = Node { l: r + 1, r: 0, v: 0 };
    let iter: Vec<Node> = self.set.range((Included(&tmp_l), Excluded(&tmp_r))).map(|node| node.clone()).collect();
    for node in iter.iter() {
        self.set.remove( node );
    }
    self.set.insert( Node { l: l, r: r, v: v });
}
```

区间赋值上，这里是`[l, r]`，所以从r + 1和l中断开。这样我们的区间就会变成`[l, x1], [x1, x2], ..., [xn, r], [r + 1, y]`。我们删除中间这一段，并让`[l, r] = v`替换他

查询的时候也需要断开区间，然后查询`[l, r]`中的三元组即可

这里的设置有一个前提，就是区间必须都是有效的。而不能是空的。也就是所有的区间并起来得到的就是整体的值域。

最后看一下整体的代码

```rust
use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Node {
    l: i32,
    r: i32,
    v: i32
}

struct RangeModule {
    set: BTreeSet<Node>,
}

impl RangeModule {
    fn new() -> Self {
        let mut set = BTreeSet::new();
        set.insert( Node { l: 1, r: 1e9 as i32, v: 0 });
        Self {
            set: set
        }
    }

    fn split(&mut self, pos: i32) {
        use std::ops::Bound::*;
        let tmp = Node { l: pos, r: 0, v: 0 };
        if let Some(node) = self.set.range((Included(&tmp), Unbounded)).next() {
            if node.l == pos {
                return;
            }
        }
        let it = self.set.range((Unbounded, Excluded(&tmp))).next_back().unwrap();
        let &Node{ l, r, v } = it;
        self.set.remove( &Node { l: l, r: r, v: v });
        self.set.insert( Node { l: l, r: pos - 1, v: v});
        self.set.insert( Node { l: pos, r: r, v: v });
    }

    fn assign(&mut self, l: i32, r: i32, v: i32) {
        use std::ops::Bound::*;
        self.split(r + 1);
        self.split(l);
        let tmp_l = Node { l: l, r: 0, v: 0 };
        let tmp_r = Node { l: r + 1, r: 0, v: 0 };
        let iter: Vec<Node> = self.set.range((Included(&tmp_l), Excluded(&tmp_r))).map(|node| node.clone()).collect();
        for node in iter.iter() {
            self.set.remove( node );
        }
        self.set.insert( Node { l: l, r: r, v: v });
    }

    fn check(&mut self, l: i32, r: i32) -> bool {
        use std::ops::Bound::*;
        self.split(r + 1);
        self.split(l);
        let tmp_l = Node { l: l, r: 0, v: 0 };
        let tmp_r = Node { l: r + 1, r: 0, v: 0 };
        for node in self.set.range((Included(&tmp_l), Excluded(&tmp_r))) {
            if node.v == 0 {
                return false;
            }
        }
        return true;
    }
    
    fn add_range(&mut self, left: i32, right: i32) {
        self.assign(left, right - 1, 1);
    }
    
    fn query_range(&mut self, left: i32, right: i32) -> bool {
        return self.check(left, right - 1);
    }
    
    fn remove_range(&mut self, left: i32, right: i32) {
        self.assign(left, right - 1, 0);
    }
}
```