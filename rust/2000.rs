impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            Some(i) => {
                format!("{}{}", &word[..i + 1].chars().rev().collect::<String>(), &word[i + 1..])
            }
            None => {
                word
            }
        }
    }
}
