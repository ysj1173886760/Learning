use std::collections::HashMap;

struct DetectSquares {
    mp: HashMap<i32, HashMap<i32, i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {

    fn new() -> Self {
        Self {
            mp: HashMap::new()
        }
    }
    
    fn add(&mut self, point: Vec<i32>) {
        *self.mp.entry(point[1]).or_default().entry(point[0]).or_default() += 1;
    }
    
    fn count(&self, point: Vec<i32>) -> i32 {
        let (x, y) = (point[0], point[1]);

        if !self.mp.contains_key(&y) {
            return 0;
        }

        let mut res = 0;
        let (mut cnt1, mut cnt2, mut cnt3) = (0, 0, 0);
        for (key, val) in self.mp.iter() {
            if *key == y {
                continue;
            }

            for d in [y - key, key - y] {
                cnt1 = *self.mp.get(&y).unwrap().get(&(x - d)).unwrap_or(&0);
                cnt2 = *val.get(&(x - d)).unwrap_or(&0);
                cnt3 = *val.get(&x).unwrap_or(&0);
                res += cnt1 * cnt2 * cnt3;
            }
        }
        return res;
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
