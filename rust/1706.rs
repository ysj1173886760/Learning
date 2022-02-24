impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans: Vec<i32> = vec![-1; m];
        for j in (0..m) {
            let mut cur = j as i32;
            for i in (0..n) {
                let pre = grid[i][cur as usize];
                cur += pre;
                if cur < 0 || cur >= m as i32 {
                    cur = -1;
                    break;
                }
                if grid[i][cur as usize] != pre {
                    cur = -1;
                    break;
                }
            }
            if cur >= 0 {
                ans[j] = cur;
            }
        }
        ans
    }
}
