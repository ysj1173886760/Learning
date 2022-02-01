use std::collections::HashSet;
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        if s.len() < 2 {
            return "".to_string();
        }
        let st: HashSet<char> = s.chars().collect();
        for (i, ch) in s.chars().into_iter().enumerate() {
            if st.contains(&ch.to_ascii_lowercase()) &&
                st.contains(&ch.to_ascii_uppercase()) {
                continue;
            }

            let s1 = Self::longest_nice_substring(s[..i].into());
            let s2 = Self::longest_nice_substring(s[i + 1..].into());
            if s1.len() >= s2.len() {
                return s1;
            } else {
                return s2;
            }
        }
        s
    }
}
