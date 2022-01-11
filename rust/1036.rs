use std::collections::HashSet;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        if blocked.is_empty() {
            return true;
        }
        let blocked = blocked.iter()
            .fold(std::collections::HashSet::new(),
            |mut v, i| {
                v.insert((i[0], i[1]));
                v
            });

        let ret1 = Solution::process((source[0], source[1]), &blocked, (target[0], target[1]));
        let ret2 = Solution::process((target[0], target[1]), &blocked, (source[0], source[1]));
        return if ret1 * ret2 > 0 {true} else {false};
    }

    fn process(source: (i32, i32), blocked: &HashSet<(i32, i32)>, other: (i32, i32)) -> i32 {
        let mut cnt = 0;
        let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let maxB = 1e6 as i32;
        let n = blocked.len();
        let mut visited = HashSet::new();
        let mut q = std::collections::VecDeque::new();

        q.push_back(source);
        while cnt < n * n / 2 && !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in dir.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if nx == other.0 && ny == other.1 {
                    return 1;
                }
                if nx < 0 || ny < 0 || nx >= maxB || ny >= maxB {
                    continue;
                }
                if visited.contains(&(nx, ny)) || blocked.contains(&(nx, ny)) {
                    continue;
                }
                q.push_back((nx, ny));
                cnt += 1;
                visited.insert((nx, ny));
            }
        }
        return if cnt < n * n / 2 {0} else {-1};
    }
}
