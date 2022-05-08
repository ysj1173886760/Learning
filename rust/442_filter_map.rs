impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .filter_map(|i| {
                let cur = nums[i].abs();
                let idx = cur as usize - 1;
                if nums[idx].is_negative() {
                    Some(cur)
                } else {
                    nums[idx] *= -1;
                    None
                }
            })
            .collect()

    }
}
