impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
            let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut maxx = 0;
            let n = grid.len();
            let m = grid[0].len();

            let tmp = grid[x][y];
            grid[x][y] = 0;
            for d in dir {
                let nx = (x as i32 + d.0) as usize;
                let ny = (y as i32 + d.1) as usize;
                if nx >= n || ny >= m || grid[nx][ny] == 0 {
                    continue;
                }
                maxx = std::cmp::max(maxx, dfs(nx, ny, grid));
            }
            grid[x][y] = tmp;
            return maxx + grid[x][y];
        }

        let mut ans = 0;
        let mut grid = grid;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] != 0 {
                    ans = std::cmp::max(ans, dfs(i, j, &mut grid));
                }
            }
        }
        ans
    }
}
