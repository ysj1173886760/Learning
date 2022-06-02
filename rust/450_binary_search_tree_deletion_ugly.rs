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
        let mut cur = root.clone();
        let mut parent = None;
        while let Some(node) = cur.clone() {
            if node.borrow().val == key {
                break;
            }
            parent = Some(node.clone());
            if node.borrow().val > key {
                cur = node.borrow().left.clone();
            } else {
                cur = node.borrow().right.clone();
            }
        }
        if cur.is_none() {
            return root;
        }
        let left = cur.clone().unwrap().borrow().left.clone();
        let right = cur.clone().unwrap().borrow().right.clone();
        if left.is_none() && right.is_none() {
            cur = None;
        } else if left.is_none() {
            cur = right;
        } else if right.is_none() {
            cur = left;
        } else {
            let mut successor = right;
            let mut successor_parent = cur.clone();
            while let Some(node) = successor.clone().unwrap().borrow().left.clone() {
                successor_parent = successor;
                successor = Some(node);
            }
            if successor_parent.clone().unwrap().borrow().val == key {
                successor_parent.unwrap().borrow_mut().right = successor.clone().unwrap().borrow().right.clone();
            } else {
                successor_parent.unwrap().borrow_mut().left = successor.clone().unwrap().borrow().right.clone();
            }
            successor.clone().unwrap().borrow_mut().right = cur.clone().unwrap().borrow().right.clone();
            successor.clone().unwrap().borrow_mut().left = cur.clone().unwrap().borrow().left.clone();
            cur = successor;
        }
        if parent.is_none() {
            return cur;
        } else {
            if !parent.clone().unwrap().borrow().left.is_none() &&
               parent.clone().unwrap().borrow().left.clone().unwrap().borrow().val == key {
                parent.clone().unwrap().borrow_mut().left = cur;
            } else {
                parent.clone().unwrap().borrow_mut().right = cur;
            }
            return root;
        }
    }
}
