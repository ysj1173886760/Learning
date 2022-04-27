use std::collections::VecDeque;

impl Solution {
    fn bfs(heights: &Vec<Vec<i32>>, top_left: bool) -> Vec<Vec<bool>> {
        let (n, m) = (heights.len(), heights[0].len());
        let mut vis = vec![vec![false; m]; n];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        if top_left {
            for i in 0..n {
                vis[i][0] = true;
                queue.push_back((i, 0));
            }
            for i in 1..m {
                vis[0][i] = true;
                queue.push_back((0, i));
            }
        } else {
            for i in 0..n {
                vis[i][m - 1] = true;
                queue.push_back((i, m - 1));
            }
            for i in 0..m - 1 {
                vis[n - 1][i] = true;
                queue.push_back((n - 1, i));
            }
        }

        let dir = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        while let Some((x, y)) = queue.pop_front() {
            for &d in dir.iter() {
                let (nx, ny) = (x as i32 + d.0, y as i32 + d.1);
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if heights[nx][ny] >= heights[x][y] && !vis[nx][ny] {
                    vis[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }

        vis
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let (n, m) = (heights.len(), heights[0].len());
        let pa = Self::bfs(&heights, true);
        let at = Self::bfs(&heights, false);

        for i in 0..n {
            for j in 0..m {
                if pa[i][j] && at[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }

        ans
    }
}
