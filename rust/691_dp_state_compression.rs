impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let len = target.len();
        let mut dp = vec![-1; (1 << len)];
        dp[0] = 0;
        let ans = Self::dfs(&mut dp, (1 << len) - 1, &stickers, &target);
        println!("{}", ans);
        if ans < target.len() as i32 { ans } else { -1 }
    }
    fn dfs(dp: &mut Vec<i32>, cur: i32, stickers: &Vec<String>, target: &str) -> i32 {
        if dp[cur as usize] != -1 {
            return dp[cur as usize];
        }
        let mut res = target.len() as i32 + 1;
        for stick in stickers {
            let mut cnt = vec![0; 26];
            for ch in stick.as_bytes().iter() {
                cnt[(ch - b'a') as usize] += 1;
            }
            let mut prev_state = cur;
            for i in 0..target.len() {
                let idx = (target.as_bytes()[i] - b'a') as usize;
                if ((1 << i) & cur) != 0 && cnt[idx] > 0 {
                    cnt[idx] -= 1;
                    prev_state ^= (1 << i);
                }
            }
            if prev_state >= cur {
                continue;
            }
            res = res.min(Self::dfs(dp, prev_state, stickers, target) + 1);
        }
        dp[cur as usize] = res;

        dp[cur as usize]
    }
}
