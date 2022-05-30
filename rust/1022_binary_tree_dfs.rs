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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: i32) -> i32 {
        if let Some(node) = root {
            let nxt = cur * 2 + node.borrow().val;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                nxt
            } else {
                Self::dfs(node.borrow().left.clone(), nxt) + Self::dfs(node.borrow().right.clone(), nxt)
            }
        } else {
            0
        }
    }
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }
}
