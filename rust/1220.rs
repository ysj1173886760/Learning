impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let md = (1e9 + 7.) as u64;
        let mut dp = vec![1u64; 5];
        for i in 1..n {
            let mut tmp = vec![0u64; 5];
            tmp[0] = (dp[1] + dp[2] + dp[4]) % md;
            tmp[1] = (dp[0] + dp[2]) % md;
            tmp[2] = (dp[1] + dp[3]) % md;
            tmp[3] = dp[2];
            tmp[4] = (dp[2] + dp[3]) % md;
            dp = tmp;
        }
        ((dp[0] + dp[1] + dp[2] + dp[3] + dp[4]) % md) as i32
    }
}
