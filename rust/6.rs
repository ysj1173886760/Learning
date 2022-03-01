impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut res = vec![String::from(""); num_rows];
        let mut i = 0;
        let s: Vec<char> = s.chars().collect();
        while i < s.len() {
            for j in 0..num_rows {
                if i == s.len() {
                    break;
                }
                res[j].push(s[i]);
                i += 1;
            }
            for j in (1..num_rows - 1).rev() {
                if i == s.len() {
                    break;
                }
                res[j].push(s[i]);
                i += 1;
            }
        }
        res.into_iter().fold(String::from(""), |mut acc, x| {
            acc.push_str(&x);
            acc
        })
    }
}
