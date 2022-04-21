impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut res = String::from("");
        for (i, word) in sentence.split(" ").enumerate() {
            let first = word.as_bytes()[0] as char;
            if String::from("aeiouAEIOU").contains(first) {
                res.push_str(word);
            } else {
                res.push_str(&word[1..]);
                res.push(first);
            }
            res.push_str("maa");
            res.push_str(&"a".repeat(i));
            res.push(' ');
        }
        res.pop();
        res
    }
}
