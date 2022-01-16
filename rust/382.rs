// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use rand::prelude::*;

struct Solution {
    pub head: Option<Box<ListNode>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }
    
    fn get_random(&self) -> i32 {
        let mut res = 0;
        let mut i = 1;
        let mut node = &self.head;
        let mut rng = thread_rng();
        while let Some(cur) = node {
            if rng.gen_range(0, i) == 0 {
                res = cur.val;
            }
            i += 1;
            node = &cur.next;
        }
        res
    }
}

