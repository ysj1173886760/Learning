impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let mut dp = vec![-1; (1 << max_choosable_integer)];
        if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        Self::dfs(max_choosable_integer, 0, desired_total, 0, &mut dp)
    }

    fn dfs(max_choosable_integer: i32, cur_integer: i32, desired_total: i32, cur_sum: i32, dp: &mut Vec<i32>) -> bool {
        if dp[cur_integer as usize] != -1 {
            return dp[cur_integer as usize] == 1;
        }
        let mut res = false;
        for i in 0..max_choosable_integer {
            if ((cur_integer >> i) & 1) != 0 {
                continue;
            }
            if i + 1 + cur_sum >= desired_total {
                res = true;
                break;
            }
            if !Self::dfs(max_choosable_integer, cur_integer | (1 << i), desired_total, i + 1 + cur_sum, dp) {
                res = true;
                break;
            }
        }
        if (res) {
            dp[cur_integer as usize] = 1;
        } else {
            dp[cur_integer as usize] = 0;
        }
        res
    }
}
