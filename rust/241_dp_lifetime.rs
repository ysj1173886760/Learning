impl Solution {
    fn dfs<'a>(l: usize, r: usize, dp: &'a mut Vec<Vec<Vec<i32>>>, ops: &Vec<i32>) -> &'a Vec<i32> {
        if dp[l][r].len() == 0 {
            if l == r {
                dp[l][r].push(ops[l]);
            } else {
                let mut i = l;
                let mut tmp = vec![];
                while i < r {
                    Self::dfs(l, i, dp, ops);
                    Self::dfs(i + 2, r, dp, ops);
                    let left = &dp[l][i];
                    let right = &dp[i + 2][r];
                    for l_elem in left {
                        for r_elem in right {
                            match ops[i + 1] {
                                -1 => { tmp.push(l_elem + r_elem); },
                                -2 => { tmp.push(l_elem - r_elem); },
                                -3 => { tmp.push(l_elem * r_elem); },
                                _ => { panic!("logic error") },
                            }
                        }
                    }
                    i += 2;
                }
                dp[l][r] = tmp;
            }
        }
        &dp[l][r]
    }

    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut ops: Vec<i32> = vec![];
        let mut i = 0;
        while i < expression.len() {
            if expression.as_bytes()[i] == b'+' {
                ops.push(-1);
                i += 1;
            } else if expression.as_bytes()[i] == b'-' {
                ops.push(-2);
                i += 1;
            } else if expression.as_bytes()[i] == b'*' {
                ops.push(-3);
                i += 1;
            } else {
                let mut tmp = 0;
                while i < expression.len() && 
                      expression.as_bytes()[i] >= b'0' &&
                      expression.as_bytes()[i] <= b'9' {
                    tmp = tmp * 10 + (expression.as_bytes()[i] - b'0');
                    i += 1;
                }
                ops.push(tmp as i32);
            }
        }
        println!("{:?}", ops);
        let mut dp = vec![vec![vec![]; ops.len()]; ops.len()];
        Self::dfs(0, ops.len() - 1, &mut dp, &ops).clone()
    }
}
