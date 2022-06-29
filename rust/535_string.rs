use std::collections::HashMap;
struct Codec {
    mp: HashMap<String, String>,
    cur: usize,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            mp: HashMap::new(),
            cur: 0,
        }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        self.cur += 1;
        let s = self.cur.to_string();
        self.mp.insert(s.clone(), longURL);
        s
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        self.mp.get(&shortURL).unwrap().to_string()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
