use std::collections::HashMap;
impl Solution {
    fn check(mp: &HashMap<&str, i32>, cur: &HashMap<&str, i32>) -> bool {
        for (k, v) in mp.iter() {
            match cur.get(k) {
                Some(val) if val == v => (),
                _ => return false,
            }
        }
        return true;
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut mp: HashMap<&str, i32> = HashMap::new();
        let word_num = words.len();
        for word in words.iter() {
            *mp.entry(word).or_insert(0) += 1;
        }
        let mut ans = vec![];
        let word_len = words[0].len();
        for offset in 0..word_len {
            let mut cur: HashMap<&str, i32> = HashMap::new();
            let mut left = offset;
            for right in (offset..s.len()).step_by(word_len) {
                if right + word_len > s.len() {
                    break;
                }
                *cur.entry(&s[right..right + word_len]).or_insert(0) += 1;
                if right - left >= word_num * word_len {
                    *cur.entry(&s[left..left + word_len]).or_insert(0) -= 1;
                    left += word_len;
                }
                if right - left == (word_num - 1) * word_len {
                    if Self::check(&mp, &cur) {
                        ans.push(left as i32);
                    }
                }
            }
        }
        ans
    }
}
