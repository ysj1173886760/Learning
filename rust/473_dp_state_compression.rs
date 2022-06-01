impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            return false;
        }
        let length = sum / 4;
        let n = matchsticks.len();
        let mut dp = vec![-1; (1 << n)];
        matchsticks.sort();
        dp[0] = 0;
        for i in 0..(1 << n) {
            for j in 0..n {
                let nxt = i ^ (1 << j);
                if ((1 << j) & i) != 0 && 
                   dp[nxt] >= 0 && 
                   dp[nxt] + matchsticks[j] <= length {
                    dp[i] = (dp[nxt] + matchsticks[j]) % length;
                    break;
                }
            }
        }
        dp[(1 << n) - 1] == 0
    }
}
