impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if !s[i].is_alphabetic() {
                i += 1;
            } else if !s[j].is_alphabetic() {
                j -= 1;
            } else {
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        s.into_iter().collect()
    }
}
