use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Node {
    l: i32,
    r: i32,
    v: i32,
}

struct MyCalendar {
    mp: BTreeSet<Node>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        let mut mp = BTreeSet::new();
        mp.insert( Node { l: 0, r: 1e9 as i32, v: 0 } );
        Self {
            mp: mp
        }
    }

    fn split(&mut self, pos: i32) {
        use std::ops::Bound::*;
        let tmp = Node { l: pos, r: 0, v: 0 };
        if let Some(node) = self.mp.range((Included(&tmp), Unbounded)).next() {
            if node.l == pos {
                return;
            }
        }
        let it = self.mp.range((Unbounded, Excluded(&tmp))).next_back().unwrap();
        let &Node{ l, r, v } = it;
        self.mp.remove( &Node { l: l, r: r, v: v });
        self.mp.insert( Node { l: l, r: pos - 1, v: v});
        self.mp.insert( Node { l: pos, r: r, v: v });
    }

    fn assign(&mut self, l: i32, r: i32, v: i32) {
        use std::ops::Bound::*;
        self.split(r + 1);
        self.split(l);
        let tmp_l = Node { l: l, r: 0, v: 0 };
        let tmp_r = Node { l: r + 1, r: 0, v: 0 };
        let iter: Vec<Node> = self.mp.range((Included(&tmp_l), Excluded(&tmp_r))).map(|node| node.clone()).collect();
        for node in iter.iter() {
            self.mp.remove( node );
        }
        self.mp.insert( Node { l: l, r: r, v: v });
    }    

    fn book(&mut self, start: i32, end: i32) -> bool {
        use std::ops::Bound::*;
        self.split(start);
        self.split(end - 1);
        let tmp_l = Node { l: start, r: 0, v: 0 };
        let tmp_r = Node { l: end, r: 0, v: 0 };
        for node in self.mp.range((Included(&tmp_l), Excluded(&tmp_r))) {
            if node.v == 1 {
                return false;
            }
        }
        self.assign(start, end - 1, 1);
        return true;
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
