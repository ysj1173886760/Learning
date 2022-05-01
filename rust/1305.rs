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
struct BstIterator {
    stk: Vec<Rc<RefCell<TreeNode>>>
}
impl BstIterator {
    pub fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut res = Self { stk: vec![] };
        res.pushLeft(root);
        res
    }
    pub fn hasNext(&self) -> bool {
        !self.stk.is_empty()
    }
    pub fn next(&mut self) -> i32 {
        let cur = self.stk.pop().unwrap();
        self.pushLeft(cur.borrow().right.clone());
        let x = cur.borrow().val;
        x
    }
    pub fn pushLeft(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(ptr) = root {
            self.stk.push(ptr.clone());
            root = ptr.borrow().left.clone();
        }
    }
    pub fn probe(&self) -> i32 {
        self.stk.last().unwrap().borrow().val
    }
}
impl Solution {
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut it1 = BstIterator::new(root1);
        let mut it2 = BstIterator::new(root2);
        let mut ans = vec![];
        
        while it1.hasNext() && it2.hasNext() {
            if it1.probe() < it2.probe() {
                ans.push(it1.next());
            } else {
                ans.push(it2.next());
            }
        }
        while it1.hasNext() {
            ans.push(it1.next());
        }
        while it2.hasNext() {
            ans.push(it2.next());
        }
        ans
    }
}
