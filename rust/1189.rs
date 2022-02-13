use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut mp: HashMap<char, i32> = HashMap::new();
        for ch in text.chars() {
            *mp.entry(ch).or_insert(0) += 1;
        }
        let mut ans = 1e4 as i32;
        ans = std::cmp::min(ans, *mp.get(&'b').unwrap_or(&0));
        ans = std::cmp::min(ans, *mp.get(&'a').unwrap_or(&0));
        ans = std::cmp::min(ans, *mp.get(&'l').unwrap_or(&0) / 2);
        ans = std::cmp::min(ans, *mp.get(&'o').unwrap_or(&0) / 2);
        ans = std::cmp::min(ans, *mp.get(&'n').unwrap_or(&0));
        ans
    }
}
