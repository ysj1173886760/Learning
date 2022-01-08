impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![0, 1];
        for i in 1..n {
            let len = res.len();
            for j in 0..len {
                res.push(res[len - j - 1]);
            }
            let len = res.len();
            for j in len / 2..len {
                res[j] |= (1 << i);
            }
        }
        res
  }
}

// more fancy one

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![0, 1];
        for i in 1..n {
            let len = res.len();
            for j in 0..len {
                res.push(res[len - j - 1]);
            }
            let len = res.len();
            for j in len / 2..len {
                res[j] |= (1 << i);
            }
        }
        res
  }
}
