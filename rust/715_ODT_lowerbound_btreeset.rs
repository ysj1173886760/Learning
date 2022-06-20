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


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
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

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */
