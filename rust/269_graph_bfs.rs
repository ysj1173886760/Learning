use std::collections::VecDeque;
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut G: Vec<Vec<usize>> = vec![vec![]; 26];
        let mut in_degree = vec![0; 26];
        let mut occur = vec![false; 26];
        for i in 0..words.len() {
            for j in 0..words[i].len() {
                occur[(words[i].as_bytes()[j] - b'a') as usize] = true;
            }
        }
        for i in 1..words.len() {
            let len = std::cmp::min(words[i - 1].len(), words[i].len());
            let mut flag = false;
            for j in 0..len {
                let lhs = words[i - 1].as_bytes()[j];
                let rhs = words[i].as_bytes()[j];
                if lhs != rhs {
                    flag = true;
                    G[(lhs - b'a') as usize].push((rhs - b'a') as usize);
                    in_degree[(rhs - b'a') as usize] += 1;
                    break;
                }
            }
            if !flag && words[i - 1].len() > words[i].len() {
                return "".to_string();
            }
        }

        let mut ans = vec![];
        let mut cnt = 0;
        let mut q = VecDeque::new();
        for i in 0..26 {
            if occur[i] && in_degree[i] == 0 {
                q.push_back(i);
            }
            if occur[i] {
                cnt += 1;
            }
        }
        while let Some(cur) = q.pop_front() {
            ans.push(cur);
            for &j in G[cur].iter() {
                in_degree[j] -= 1;
                if in_degree[j] == 0 {
                    q.push_back(j);
                }
            }
        }
        if cnt != ans.len() {
            return "".to_string();
        }
        return ans.iter().map(|&x| (x as u8 + b'a') as char).collect::<String>();
    }
}
