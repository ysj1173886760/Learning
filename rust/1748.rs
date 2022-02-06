impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(std::collections::HashMap::new(), |mut mp, x| {
                *mp.entry(x).or_insert(0) += 1;
                mp
            })
            .into_iter()
            .filter(|(_, v)| *v == 1)
            .fold(0, |sum, (k, _)| k + sum)
    }
}
