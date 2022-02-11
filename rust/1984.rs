impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let k = k as usize;
        let mut ans = i32::MAX;
        let mut i = 0;
        while i + k - 1 < nums.len() {
            ans = std::cmp::min(ans, nums[i + k - 1] - nums[i]);
            i += 1;
        }
        ans
    }
}
