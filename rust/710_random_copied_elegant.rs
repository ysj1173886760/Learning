use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;

struct Solution {
    // blacklist number in [0, n - m) -> whitelist number in [n - m, n)
    mp: HashMap<i32, i32>,
    // n - m
    range: i32,
    rng: rand::rngs::ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let range = n - blacklist.len() as i32;
        let st = blacklist.iter().copied().collect::<HashSet<i32>>();
        let mut mp = HashMap::new();
        let mut last = n - 1;
        for &num in blacklist.iter() {
            if num >= range {
                continue;
            }
            while st.contains(&last) {
                last -= 1;
            }
            mp.insert(num, last);
            last -= 1;
        }
        Self {
            mp: mp,
            range: range,
            rng: rand::thread_rng(),
        }
    }
    
    fn pick(&mut self) -> i32 {
        let x = self.rng.gen_range(0, self.range);
        self.mp.get(&x).copied().unwrap_or(x)
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */
