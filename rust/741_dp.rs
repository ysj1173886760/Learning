impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let k = 2 * n - 1;
        let mut dp = vec![vec![vec![i32::MIN / 2; n]; n]; k];
        dp[0][0][0] = grid[0][0];
        for i in 1..k {
            for x1 in 0..=(i.min(n - 1)) {
                for x2 in 0..=x1 {
                    let y1 = i - x1;
                    let y2 = i - x2;
                    if y1 >= n || y2 >= n {
                        continue;
                    }
                    if grid[x1][y1] == -1 || grid[x2][y2] == -1 {
                        continue;
                    }
                    let mut val = 0;
                    if x1 == x2 {
                        val = grid[x1][y1];
                    } else {
                        val = grid[x1][y1] + grid[x2][y2];
                    }
                    dp[i][x1][x2] = dp[i][x1][x2].max(dp[i - 1][x1][x2]);
                    if x1 > 0 && x2 > 0 {
                        dp[i][x1][x2] = dp[i][x1][x2].max(dp[i - 1][x1 - 1][x2 - 1]);
                    }
                    if x1 > 0 {
                        dp[i][x1][x2] = dp[i][x1][x2].max(dp[i - 1][x1 - 1][x2]);
                    }
                    if x2 > 0 {
                        dp[i][x1][x2] = dp[i][x1][x2].max(dp[i - 1][x1][x2 - 1]);
                    }
                    dp[i][x1][x2] += val;
                }
            }
        }
        if dp[2 * n - 2][n - 1][n - 1] < 0 {
            0
        } else {
            dp[2 * n - 2][n - 1][n - 1]
        }
    }
}
