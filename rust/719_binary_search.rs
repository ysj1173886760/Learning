impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut lb = -1;
        let mut ub = 1e6 as i32;
        while ub - lb > 1 {
            let mid = (lb + ub) / 2;
            let mut left = 0;
            let mut cnt = 0;
            for right in (0..nums.len()) {
                while left < right && nums[right] - nums[left] > mid {
                    left += 1;
                }
                cnt += right - left;
            }
            if cnt >= k as usize {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        ub
    }
}
