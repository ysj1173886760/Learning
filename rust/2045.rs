use std::collections::VecDeque;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut G: Vec<Vec<usize>> = vec![vec![]; n as usize + 1];
        for edge in edges.iter() {
            G[edge[0] as usize].push(edge[1] as usize);
            G[edge[1] as usize].push(edge[0] as usize);
        }
        let mut vis: Vec<bool> = vec![false; n as usize + 1];
        let mut dis: Vec<Vec<i32>> = vec![vec![0x3f3f3f3f; 2]; n as usize + 1];
        let mut q: VecDeque<usize> = VecDeque::with_capacity(n as usize);

        vis[1] = true;
        dis[1][0] = 0;
        q.push_back(1);

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let (mut cost0, mut cost1) = (0, 0);

            if ((dis[cur][0] / change) % 2) != 0 {
                cost0 = change - dis[cur][0] % change
            }
            if ((dis[cur][1] / change) % 2) != 0 {
                cost1 = change - dis[cur][1] % change
            }

            // println!("{} {} {}", cur, dis[cur][0], dis[cur][1]);
            for to in G[cur].iter() {
                if dis[cur][0] + time + cost0 < dis[*to][0] {
                    dis[*to][0] = dis[cur][0] + time + cost0;
                    if !vis[*to] {
                        q.push_back(*to);
                        vis[*to] = true;
                    }
                }
                if (dis[cur][0] + time + cost0 < dis[*to][1]) &&
                    (dis[cur][0] + time + cost0 > dis[*to][0]) {
                    dis[*to][1] = dis[cur][0] + time + cost0;
                    if !vis[*to] {
                        q.push_back(*to);
                        vis[*to] = true;
                    }
                }
                if (dis[cur][1] + time + cost1 < dis[*to][1]) &&
                    (dis[cur][1] + time + cost1 > dis[*to][0]) {
                    dis[*to][1] = dis[cur][1] + time + cost1;
                    if !vis[*to] {
                        q.push_back(*to);
                        vis[*to] = true;
                    }
                }
            }
            vis[cur] = false;
        }
        dis[n as usize][1]
    }
}
