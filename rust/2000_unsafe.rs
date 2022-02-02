impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut word = word;
        match word.find(ch) {
            Some(j) => {
                unsafe {
                    let mut s = word.as_bytes_mut();
                    let mut j = j;
                    let mut i = 0;
                    while i < j {
                        s.swap(i, j);
                        i += 1;
                        j -= 1;
                    }
                    word
                }
            }
            None => {
                word
            }
        }
    }
}
