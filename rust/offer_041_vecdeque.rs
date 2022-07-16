use std::collections::VecDeque;

struct MovingAverage {
    q: VecDeque<i32>,
    size: usize,
    sum: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        Self {
            q: VecDeque::new(),
            size: size as usize,
            sum: 0
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        self.q.push_back(val);
        self.sum += val;
        if (self.q.len() > self.size) {
            self.sum -= self.q.pop_front().unwrap();
        }
        self.sum as f64 / self.q.len() as f64
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */
