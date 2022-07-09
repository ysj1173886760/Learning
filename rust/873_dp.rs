use std::collections::HashMap;
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut mp: HashMap<i32, usize> = HashMap::new();
        arr.iter().enumerate().for_each(|(i, x)| { mp.insert(*x, i); });
        let n = arr.len();
        let mut dp = vec![vec![0; n]; n];
        let mut ans = 0;
        for i in 0..n {
            for j in (0..i).rev() {
                if arr[j] * 2 < arr[i] {
                    break;
                }
                let prev = arr[i] - arr[j];
                if let Some(&k) = mp.get(&prev) {
                    if k != j {
                        dp[i][j] = (dp[j][k] + 1).max(3);
                    }
                }
                ans = ans.max(dp[i][j]);
            }
        }
        ans
    }
}
