use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut mp: HashMap<&str, i32> = HashMap::new();
        s1.split(' ').chain(s2.split(' ')).for_each(|x| {
            *mp.entry(x).or_insert(0) += 1;
        });
        mp.iter().filter(|(_, &v)| v == 1).map(|(&k, _)| k.to_string()).collect()
    }
}
