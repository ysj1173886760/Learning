impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mid = nums[nums.len() / 2];
        nums.iter().map(|&x| (x - mid).abs()).sum()
    }
}
