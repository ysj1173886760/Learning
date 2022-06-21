// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut q = vec![];
        q.push(root.unwrap());
        while q.len() > 0 {
            ans = q[0].borrow().val;
            let mut tmp = vec![];
            for i in 0..q.len() {
                if q[i].borrow().left.is_some() {
                    tmp.push(q[i].borrow().left.clone().unwrap());
                }
                if q[i].borrow().right.is_some() {
                    tmp.push(q[i].borrow().right.clone().unwrap());
                }
            }
            q = tmp;
        }
        ans
    }
}
