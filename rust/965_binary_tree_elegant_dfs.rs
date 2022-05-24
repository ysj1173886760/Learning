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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            if let Some(cur) = root {
                if cur.borrow().val != val {
                    return false;
                }
                return dfs(cur.borrow().left.clone(), val) && dfs(cur.borrow().right.clone(), val);
            }
            true
        }

        let val = root.as_ref().unwrap().borrow().val;
        dfs(root, val)
    }
}
