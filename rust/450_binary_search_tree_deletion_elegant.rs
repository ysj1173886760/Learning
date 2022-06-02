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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn find_succ(node: &Rc<RefCell<TreeNode>>) -> i32 {
            let node = node.borrow();
            match node.left.as_ref() {
                None => node.val,
                Some(left) => find_succ(left),
            }
        }
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let val = node.val;
            if val > key {
                node.left = Self::delete_node(node.left.take(), key);
            } else if val < key {
                node.right = Self::delete_node(node.right.take(), key);
            } else {
                match (node.left.is_some(), node.right.is_some()) {
                    (false, false) => { return None; }
                    (false, true) => { return node.right.take(); }
                    (true, false) => { return node.left.take(); }
                    (true, true) => {
                        let succ = find_succ(node.right.as_ref().unwrap());
                        node.val = succ;
                        node.right = Self::delete_node(node.right.take(), succ);
                    }
                }
            }
        }
        root
    }
}
