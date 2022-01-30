use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut ret = vec![];
        let mut mp: HashMap<String, i32> = HashMap::new();

        fn process(mp: &mut HashMap<String, i32>, s: &String) {
            for word in s.split(' ') {
                *mp.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        process(&mut mp, &s1);
        process(&mut mp, &s2);

        for (k, v) in mp {
            if v == 1 {
                ret.push(k.to_string());
            }
        }
        ret
    }
}
