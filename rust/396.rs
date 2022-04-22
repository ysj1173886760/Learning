impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let cur = nums.iter()
                        .enumerate()
                        .fold(0, |sum, (i, &val)| {
                            sum + i as i32 * val
                        });
        nums.iter()
            .rev()
            .fold((cur, cur), |(maxx, cur_sum), &val| {
                let nxt_sum = cur_sum + sum - nums.len() as i32 * val;
                (maxx.max(nxt_sum), nxt_sum)
            }).0
    }
}
