use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        nums.iter()
            .enumerate()
            .fold((HashMap::with_capacity(nums.len()), false), |(mut mp, mut res): (HashMap<i32, usize>, bool), (idx, x)| {
                if let Some(prev) = mp.insert(*x, idx) {
                    if (idx - prev) <= k as usize {
                        res = true;
                    }
                }
                (mp, res)
            }).1
    }
}
