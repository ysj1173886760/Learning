use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut res: Vec<Vec<i32>> = vec![vec![-1; n]; m];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let dir: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    q.push_back((i, j));
                    res[i][j] = 0;
                }
            }
        }

        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            for d in dir.iter() {
                let (nx, ny) = (x as i32 + d.0, y as i32 + d.1);
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if res[nx][ny] != -1 {
                    continue;
                }
                res[nx][ny] = res[x][y] + 1;
                q.push_back((nx, ny));
            }
        }
        res
    }
}
