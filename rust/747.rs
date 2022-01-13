impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] > max1 {
                max2 = max1;
                max1 = nums[i];
                index = i;
            } else if nums[i] > max2 {
                max2 = nums[i];
            }
        }
        return if max1 >= max2 * 2 { index as i32 } else { -1 }
    }
}
