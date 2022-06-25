impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = costs[0].clone();
        for i in 1..costs.len() {
            let mut new = vec![0; 3];
            for j in 0..3 {
                new[j] = dp[(j + 1) % 3].min(dp[(j + 2) % 3]) + costs[i][j];
            }
            dp = new;
        }
        dp[0].min(dp[1]).min(dp[2])
    }
}
