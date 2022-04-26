impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut col_max = vec![0; m];
        let mut row_max = vec![0; n];
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] != 0 {
                    ans += 1;
                }
                col_max[j] = col_max[j].max(grid[i][j]);
                row_max[i] = row_max[i].max(grid[i][j]);
            }
        }

        col_max.iter().sum::<i32>() + row_max.iter().sum::<i32>() + ans
    }
}
