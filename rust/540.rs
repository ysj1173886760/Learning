impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lb, mut ub) = (0, nums.len() - 1);
        while ub - lb > 1 {
            let mut mid = (ub + lb) / 2;
            // convert to even number
            if mid & 1 == 1 {
                mid -= 1;
            }
            if nums[mid] != nums[mid + 1] {
                ub = mid;
            } else {
                lb = mid + 1;
            }
        }
        nums[ub]
    }
}
