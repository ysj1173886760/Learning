impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort();
        let mut ans = vec![0; nums.len()];
        let mut ptr = nums.len() - 1;
        for i in (1..nums.len()).step_by(2) {
            ans[i] = nums[ptr];
            ptr -= 1;
        }
        for i in (0..nums.len()).step_by(2) {
            ans[i] = nums[ptr];
            ptr -= 1;
        }
        *nums = ans;
    }
}
