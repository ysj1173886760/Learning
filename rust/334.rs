impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut first = nums[0];
        let mut second = std::i32::MAX;
        for i in 1..nums.len() {
            if nums[i] > second {
                return true;
            } else if nums[i] > first {
                second = nums[i];
            } else {
                first = nums[i];
            }
        }
        return false;
    }
}
