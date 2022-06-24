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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q = vec![];
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        q.push(root.unwrap());
        while q.len() > 0 {
            let mut next = vec![];
            let mut maxx = i32::MIN;
            for i in 0..q.len() {
                maxx = maxx.max(q[i].borrow().val);
                if q[i].borrow().left.is_some() {
                    next.push(q[i].borrow().left.clone().unwrap());
                }
                if q[i].borrow().right.is_some() {
                    next.push(q[i].borrow().right.clone().unwrap());
                }
            }
            q = next;
            ans.push(maxx);
        }
        ans
    }
}
