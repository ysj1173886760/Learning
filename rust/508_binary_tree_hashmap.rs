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
use std::collections::HashMap;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mp: &mut HashMap<i32, i32>) -> i32 {
        if let Some(cur) = root {
            let sum = Self::dfs(cur.borrow().left.clone(), mp) + 
                      Self::dfs(cur.borrow().right.clone(), mp) + 
                      cur.borrow().val;
            *mp.entry(sum).or_insert(0) += 1;
            sum
        } else {
            0
        }
    }
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        Self::dfs(root, &mut mp);
        let mut maxx = 0;
        let mut ans = vec![];
        for (_, &val) in mp.iter() {
            maxx = maxx.max(val);
        }
        for (&key, &val) in mp.iter() {
            if val == maxx {
                ans.push(key);
            }
        }
        ans
    }
}
