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
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, preorder_seq: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        preorder_seq.push(root.as_ref().unwrap().borrow().val);
        Self::dfs(root.as_ref().unwrap().borrow().left.clone(), preorder_seq);
        Self::dfs(root.as_ref().unwrap().borrow().right.clone(), preorder_seq);
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut seq = vec![];
        Self::dfs(root, &mut seq);
        seq.iter().map(|x| x.to_string() + ",").collect()
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let seq: Vec<_> = data.split(",")
                            .filter_map(|s| s.parse::<i32>().ok())
                            .collect();

        let mut i = 0;
        Self::build(&seq, &mut i, i32::MIN, i32::MAX)
    }

    fn build(seq: &Vec<i32>, ptr: &mut usize, lb: i32, ub: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if *ptr >= seq.len() || seq[*ptr] < lb || seq[*ptr] > ub {
            return None;
        }
        let val = seq[*ptr];
        *ptr += 1;
        let cur_node = Rc::new(RefCell::new(TreeNode::new(val)));
        cur_node.borrow_mut().left = Self::build(seq, ptr, lb, val);
        cur_node.borrow_mut().right = Self::build(seq, ptr, val, ub);
        Some(cur_node)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
