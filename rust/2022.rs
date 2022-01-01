impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;
        if original.len() != m * n {
            return vec![];
        }

        let mut res = vec![vec![0; n]; m];

        for i in 0..original.len() {
            res[i / n][i % n] = original[i];
        }
        return res;
    }
}