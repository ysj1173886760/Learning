impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let md = 1e9 as i32 + 7;
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for l in 1..=n {
            for i in 0..n {
                let j = i + l - 1;
                if j >= n {
                    break;
                }

                if i == j {
                    dp[i][j] = 1;
                } else if s.as_bytes()[i] == s.as_bytes()[j] {
                    let mut lb = i + 1;
                    let mut ub = j - 1;
                    while lb <= ub && s.as_bytes()[lb] != s.as_bytes()[i] {
                        lb += 1;
                    }
                    while lb <= ub && s.as_bytes()[ub] != s.as_bytes()[i] {
                        ub -= 1;
                    }
                    if lb < ub {
                        dp[i][j] = ((dp[i + 1][j - 1] * 2 - dp[lb + 1][ub - 1]) % md + md) % md;
                    } else if ub == lb {
                        dp[i][j] = (dp[i + 1][j - 1] * 2 + 1) % md;
                    } else {
                        dp[i][j] = (dp[i + 1][j - 1] * 2 + 2) % md;
                    }
                } else {
                    dp[i][j] = ((dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1]) % md + md) % md;
                }
            }
        }
        dp[0][n - 1]
    }
}
