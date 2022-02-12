use std::collections::VecDeque;
impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let n = grid.len();
        let m = grid[0].len();
        for i in (0..n) {
            if grid[i][0] == 1 {
                q.push_back((i, 0));
            }
            if grid[i][m - 1] == 1 {
                q.push_back((i, m - 1));
            }
        }

        for i in (1..m - 1) {
            if grid[0][i] == 1 {
                q.push_back((0, i));
            }
            if grid[n - 1][i] == 1 {
                q.push_back((n - 1, i));
            }
        }
        let dir: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            grid[x][y] = 0;
            for d in dir.iter() {
                let (nx, ny) = ((x as i32 + d.0) as usize, (y as i32 + d.1) as usize);
                if nx >= n || ny >= m || grid[nx][ny] == 0 {
                    continue;
                }
                q.push_back((nx, ny));
            }
        }
        grid.iter().map(|vec| -> i32 {vec.iter().sum()}).sum()
    }
}
