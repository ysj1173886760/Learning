impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![vec![0f64; k + 1]; n]; n];
        let dir = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1)];
        for kk in (0..k + 1) {
            if kk == 0 {
                for i in (0..n) {
                    for j in (0..n) {
                        dp[i][j][kk] = 1f64;
                    }
                }
            } else {
                for i in (0..n) {
                    for j in (0..n) {
                        let mut count = 0f64;
                        for d in dir.iter() {
                            let nx = i as i32 + d.0;
                            let ny = j as i32 + d.1;
                            if nx >= n as i32 || nx < 0 || ny >= n as i32 || ny < 0 {
                                continue;
                            }
                            count += dp[nx as usize][ny as usize][kk - 1];
                        }
                        dp[i][j][kk] = count / 8f64;
                    }
                }
            }
        }
        return dp[row as usize][column as usize][k];
    }
}
