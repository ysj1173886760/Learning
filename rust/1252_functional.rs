impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut col = vec![0; n as usize];
        let mut row = vec![0; m as usize];
        for vec in indices {
            row[vec[0] as usize] += 1;
            col[vec[1] as usize] += 1;
        }
        let row_odd = row.iter().filter(|x| (**x % 2) != 0).count() as i32;
        let col_odd = col.iter().filter(|x| (**x % 2) != 0).count() as i32;
        row_odd * (n - col_odd) + col_odd * (m - row_odd)
    }
}
