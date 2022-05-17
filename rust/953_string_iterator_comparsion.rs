impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut table = vec![0; 26];
        for (i, ch) in order.as_bytes().iter().enumerate() {
            table[(ch - b'a') as usize] = i;
        }
        let mut last = &words[0];
        let mapping = |c| table[(c - b'a') as usize];

        for cur in words.iter().skip(0) {
            if !last.bytes().map(mapping).le(cur.bytes().map(mapping)) {
                return false;
            }
            last = cur;
        }
        true
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.le
    }
}
