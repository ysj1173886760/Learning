use std::collections::HashMap;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut ans = vec![];
        let check = |lhs: &str, rhs: &str| -> bool {
            let mut mp: HashMap<u8, u8> = HashMap::new();
            for i in 0..lhs.len() {
                match mp.insert(lhs.as_bytes()[i], rhs.as_bytes()[i]) {
                    Some(tmp) if tmp != rhs.as_bytes()[i] => { return false; },
                    _ => ()
                }
            }
            true
        };
        for word in words.into_iter() {
            let mut w2p: HashMap<u8, u8> = HashMap::new();
            if check(&word, &pattern) && check(&pattern, &word) {
                ans.push(word);
            }
        }
        ans
    }
}
