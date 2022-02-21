impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut q = vec![];
        let mut s: Vec<char> = dominoes.chars().collect();
        for (idx, ch) in s.iter().enumerate() {
            if *ch != '.' {
                q.push(idx);
            }
        }
        while q.len() > 0 {
            let mut tmp = vec![];
            for idx in q {
                if s[idx] == '.' {
                    continue;
                }
                if s[idx] == 'L' {
                    if idx - 1 >= s.len() {
                        continue;
                    }
                    if s[idx - 1] == 'Y' {
                        s[idx - 1] = '.';
                    } else if s[idx - 1] == '.' {
                        s[idx - 1] = 'Z';
                        tmp.push(idx - 1);
                    }
                } else if s[idx] == 'R' {
                    if idx + 1 >= s.len() {
                        continue;
                    }
                    if s[idx + 1] == 'Z' {
                        s[idx + 1] = '.';
                    } else if s[idx + 1] == '.' {
                        s[idx + 1] = 'Y';
                        tmp.push(idx + 1);
                    }
                }
            }
            for idx in tmp.iter() {
                if s[*idx] == 'Z' {
                    s[*idx] = 'L';
                } else if s[*idx] == 'Y' {
                    s[*idx] = 'R';
                }
            }
            q = tmp;
        }
        s.into_iter().collect()
    }
}
