impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lb = -1;
        let mut ub = nums.len() as i32;
        while ub - lb > 1 {
            let mid = (lb + ub) / 2;
            if (nums[mid as usize] >= target) {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        return ub;
    }
}