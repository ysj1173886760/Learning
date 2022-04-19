impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let vec = s.chars().collect::<Vec<_>>();
        let mut ans = vec![i32::MAX; s.len()];
        let mut cur = -0x3f3f3f3f;
        for (i, ch) in vec.iter().enumerate() {
            if (*ch == c) {
                cur = i as i32;
            }
            ans[i] = i as i32 - cur;
        }
        cur = 0x3f3f3f3f;
        for (i, ch) in vec.iter().enumerate().rev() {
            if (*ch == c) {
                cur = i as i32;
            }
            ans[i] = std::cmp::min(cur - i as i32, ans[i]);
        }
        ans
    }
}
