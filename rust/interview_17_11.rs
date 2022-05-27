impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut ans = i32::MAX;
        let (mut idx1, mut idx2) = (-1, -1);
        for (idx, word) in words.iter().enumerate() {
            if word == &word1 {
                idx1 = idx as i32;
            } else if word == &word2 {
                idx2 = idx as i32;
            }
            if idx1 != -1 && idx2 != -1 {
                ans = ans.min((idx1 - idx2).abs());
            }
        }
        ans
    }
}
