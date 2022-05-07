use std::collections::{VecDeque, HashMap};
impl Solution {
    fn check(lhs: &str, rhs: &str) -> bool {
        let lhs_ch: Vec<char> = lhs.chars().collect();
        let rhs_ch: Vec<char> = rhs.chars().collect();
        let mut cnt = 0;
        for i in 0..lhs_ch.len() {
            if lhs_ch[i] != rhs_ch[i] {
                cnt += 1;
            }
        }
        return cnt <= 1;
    }
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut mp: HashMap<&str, i32> = HashMap::new();
        let mut queue: VecDeque<&str> = VecDeque::new();
        mp.insert(&start, 0);
        queue.push_back(&start);

        while let Some(front) = queue.pop_front() {
            for s in bank.iter() {
                if Self::check(front, s) && !mp.contains_key(s.as_str()) {
                    mp.insert(s, mp.get(front).unwrap() + 1);
                    queue.push_back(s);
                }
            }
        }

        *mp.get(end.as_str()).unwrap_or(&-1)
    }
}
