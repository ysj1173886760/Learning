use rand;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        loop {
            let x1: usize = rand::random::<usize>() % nums.len();
            let x2: usize = rand::random::<usize>() % nums.len();
            if x1 != x2 && nums[x1] == nums[x2] {
                return nums[x1];
            }
        }
    }
}
