use std::collections::VecDeque;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        fn bfs(forest: &Vec<Vec<i32>>, from: (usize, usize), to: (usize, usize)) -> i32 {
            let (n, m) = (forest.len(), forest[0].len());
            let mut vis = vec![vec![i32::MAX; m]; n];
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            vis[from.0][from.1] = 0;
            queue.push_back((from.0, from.1));
            
            let dir = vec![0, 1, 0, -1, 0];
            while let Some((x, y)) = queue.pop_front() {
                (0..4)
                    .map(|idx| {
                        (
                            (x as i32 + dir[idx]) as usize,
                            (y as i32 + dir[idx + 1]) as usize
                        )
                    })
                    .filter(|&(nx, ny)| {
                        nx < n && ny < m && forest[nx][ny] != 0
                    })
                    .for_each(|(nx, ny)| {
                        if vis[nx][ny] == i32::MAX {
                            vis[nx][ny] = vis[x][y] + 1;
                            queue.push_back((nx, ny));
                        }
                    })
            }
            vis[to.0][to.1]
        }
        let mut trees = vec![];
        for i in 0..forest.len() {
            for j in 0..forest[0].len() {
                if forest[i][j] > 1 {
                    trees.push((i, j));
                }
            }
        }
        trees.sort_by(|lhs, rhs| {
            return forest[lhs.0][lhs.1].cmp(&forest[rhs.0][rhs.1]);
        });
        
        let mut ans = 0;
        let mut cur = (0, 0);
        for tree in trees.iter() {
            let x = bfs(&forest, cur, *tree);
            if x == i32::MAX {
                return -1;
            }
            ans += x;
            cur = *tree;
        }
        ans
    }
}
