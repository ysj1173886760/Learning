impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut dp = vec![0; 26];
        let mut cnt = 1;
        let mut lst = p.as_bytes()[0] - b'a';
        dp[lst as usize] = 1;
        for ch in p.as_bytes().iter().skip(1) {
            let cur = ch - b'a';
            if cur != (lst + 1) % 26 {
                cnt = 1;
            } else {
                cnt += 1;
            }
            dp[cur as usize] = dp[cur as usize].max(cnt);
            lst = cur;
        }
        dp.iter().sum()
    }
}
