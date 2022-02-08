use std::collections::HashMap;
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut mp = HashMap::new();
        let mut ans = 0;
        for x in nums {
            if mp.contains_key(&(x + k)) {
                ans += *mp.get(&(x + k)).unwrap();
            }
            if mp.contains_key(&(x - k)) {
                ans += *mp.get(&(x - k)).unwrap();
            }
            *mp.entry(x).or_insert(0) += 1;
        }
        ans
    }
}
