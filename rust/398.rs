use std::collections::HashMap;
use rand;

struct Solution {
    table: HashMap<i32, Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut tmp: HashMap<i32,Vec<i32>> = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            let array = tmp.entry(x).or_insert(vec![]);
            array.push(i as i32);
        }

        Solution {
            table: tmp
        }
    }
    
    fn pick(&self, target: i32) -> i32 {
        let array = self.table.get(&target).unwrap();
        let index = rand::random::<usize>() % array.len();
        array[index]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
