impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut cnt = heights.clone();
        cnt.sort();
        (0..heights.len())
            .map(|i| (heights[i] != cnt[i]) as i32)
            .sum()
    }
}
