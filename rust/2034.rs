use std::collections::{BTreeMap, HashMap};

struct StockPrice {
    last: i32,
    mp: BTreeMap<i32, i32>,
    tbl: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {

    fn new() -> Self {
        Self {
            last: 0,
            mp: BTreeMap::new(),
            tbl: HashMap::new(),
        }
    }
    
    fn update(&mut self, timestamp: i32, price: i32) {
        if timestamp > self.last {
            self.last = timestamp;
        }
        if self.tbl.contains_key(&timestamp) {
            let old_val = self.tbl.get(&timestamp).unwrap();
            *self.mp.get_mut(old_val).unwrap() -= 1;
            if (*self.mp.get(old_val).unwrap() == 0) {
                self.mp.remove(old_val);
            }
        }
        self.tbl.insert(timestamp, price);
        *self.mp.entry(price).or_insert(0) += 1;
    }
    
    fn current(&self) -> i32 {
        *self.tbl.get(&self.last).unwrap()
    }
    
    fn maximum(&self) -> i32 {
        *self.mp.iter().rev().next().unwrap().0
    }
    
    fn minimum(&self) -> i32 {
        *self.mp.iter().next().unwrap().0
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */
