impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut maxx = matrix[0].clone();
        let mut ans = vec![];
        for i in (1..n) {
            for j in (0..m) {
                maxx[j] = std::cmp::max(maxx[j], matrix[i][j]);
            }
        }
        for i in (0..n) {
            let minn = *matrix[i].iter().min().unwrap();
            for j in (0..m) {
                if maxx[j] == minn {
                    ans.push(minn);
                    break;
                }
            }
        }
        ans
    }
}
