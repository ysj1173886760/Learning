use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        nums.iter()
            .enumerate()
            .fold((HashMap::new(), false), |(mut mp, mut res): (HashMap<i32, usize>, bool), (idx, x)| {
                if mp.contains_key(x) && idx - mp.get(x).unwrap() <= (k as usize)  {
                    res = true
                }
                mp.insert(*x, idx);
                (mp, res)
            }).1
    }
}
