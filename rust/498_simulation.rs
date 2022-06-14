impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let n = mat.len();
        let m = mat[0].len();
        for i in 0..(n + m - 1) {
            if i % 2 == 0 {
                let mut col = 0;
                let mut row = i - col;
                for j in 0..m.max(n) {
                    if row < n && col < m {
                        ans.push(mat[row][col]);
                    }
                    col += 1;
                    row -= 1;
                }
            } else {
                let mut row = 0;
                let mut col = i - row;
                for j in 0..m.max(n) {
                    if row < n && col < m {
                        ans.push(mat[row][col]);
                    }
                    row += 1;
                    col -= 1;
                }
            }
        }
        ans
    }
}
