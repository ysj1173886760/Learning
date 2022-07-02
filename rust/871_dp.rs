impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let n = stations.len();
        let mut dp = vec![i32::MIN; n + 1];
        dp[0] = start_fuel;
        for i in 0..n {
            let mut nxt = vec![i32::MIN; n + 1];
            nxt[0] = dp[0];
            for j in 1..=n {
                if dp[j - 1] >= stations[i][0] {
                    nxt[j] = dp[j - 1] + stations[i][1];
                }
                if dp[j] >= stations[i][0] {
                    nxt[j] = nxt[j].max(dp[j]);
                }
            }
            dp = nxt;
        }
        for i in 0..n + 1 {
            if dp[i] >= target {
                return i as i32;
            }
        }
        return -1;
    }
}
