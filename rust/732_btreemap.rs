use std::collections::BTreeMap;
struct MyCalendarThree {
    map: BTreeMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.map.entry(start).or_insert(0) += 1;
        *self.map.entry(end).or_insert(0) -= 1;
        let mut res = 0;
        let mut cur = 0;
        for (_, v) in self.map.iter() {
            cur += *v;
            res = res.max(cur);
        }
        res
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
