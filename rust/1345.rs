use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut mp: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut vis: Vec<bool> = vec![false; arr.len()];
        let mut q: VecDeque<(usize, i32)> = VecDeque::new();

        for (idx, x) in arr.iter().enumerate() {
            mp.entry(*x).or_default().push(idx);
        }
        
        q.push_back((0, 0));

        while !q.is_empty() {
            let (cur, step) = q.pop_front().unwrap();
            if cur + 1 < arr.len() && !vis[cur + 1] {
                vis[cur + 1] = true;
                if cur + 1 == arr.len() - 1 {
                    return step + 1;
                }
                q.push_back((cur + 1, step + 1));
            }
            if cur > 0 && !vis[cur - 1] {
                vis[cur - 1] = true;
                q.push_back((cur - 1, step + 1));
            }
            if mp.contains_key(&arr[cur]) {
                for x in mp.remove(&arr[cur]).unwrap() {
                    if x == cur {
                        continue;
                    }
                    if !vis[x] {
                        vis[x] = true;
                        if x == arr.len() - 1 {
                            return step + 1;
                        }
                        q.push_back((x, step + 1));
                    }
                }
            }
        }
        arr.len() as i32 - 1
    }
}
