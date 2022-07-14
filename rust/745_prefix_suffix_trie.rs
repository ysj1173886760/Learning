use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
struct Node {
    children: HashMap<String, Rc<RefCell<Node>>>,
    idx: usize,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            idx: 0,
        }
    }
}

struct WordFilter {
    root: Rc<RefCell<Node>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {

    fn insert(root: Rc<RefCell<Node>>, word: &str, idx: usize) {
        let mut cur = root.clone();
        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            let mut s = String::new();
            s.push(bytes[i] as char);
            s.push(bytes[bytes.len() - i - 1] as char);
            // println!("{:?}", s);
            if cur.borrow().children.get(&s).is_none() {
                cur.borrow_mut().children.insert(s.clone(), Rc::new(RefCell::new(Node::new())));
            }
            let nxt = (*cur.borrow().children.get(&s).clone().unwrap()).clone();
            let mut tmp = cur.clone();
            for j in i..bytes.len() {
                let mut s = String::new();
                s.push(bytes[j] as char);
                s.push('#' as char);
                // println!("{:?}", s);
                if tmp.borrow().children.get(&s).is_none() {
                    tmp.borrow_mut().children.insert(s.clone(), Rc::new(RefCell::new(Node::new())));
                }
                let nxt = (*tmp.borrow().children.get(&s).clone().unwrap()).clone();
                tmp = nxt;
                tmp.borrow_mut().idx = idx;
            }
            tmp = cur.clone();
            for j in i..bytes.len() {
                let mut s = String::new();
                s.push('#' as char);
                s.push(bytes[bytes.len() - j - 1] as char);
                // println!("{:?}", s);
                if tmp.borrow().children.get(&s).is_none() {
                    tmp.borrow_mut().children.insert(s.clone(), Rc::new(RefCell::new(Node::new())));
                }
                let nxt = (*tmp.borrow().children.get(&s).clone().unwrap()).clone();
                tmp = nxt;
                tmp.borrow_mut().idx = idx;
            }
            cur = nxt;
            cur.borrow_mut().idx = idx;
        }
    }
    fn new(words: Vec<String>) -> Self {
        let root = Rc::new(RefCell::new(Node::new()));
        for (i, word) in words.iter().enumerate() {
            Self::insert(root.clone(), word, i);
        }
        Self {
            root: root
        }
    }
    
    fn f(&self, pref: String, suff: String) -> i32 {
        let mut cur = self.root.clone();
        let pre = pref.as_bytes();
        let suf = suff.as_bytes();
        for i in 0..pre.len().max(suf.len()) {
            let mut s = String::new();
            if i >= pre.len() {
                s.push('#');
            } else {
                s.push(pre[i] as char);
            }
            if i >= suf.len() {
                s.push('#');
            } else {
                s.push(suf[suf.len() - i - 1] as char);
            }
            // println!("{:?}", s);
            if cur.borrow().children.get(&s).is_none() {
                return -1;
            }
            let nxt = (*cur.borrow().children.get(&s).clone().unwrap()).clone();
            cur = nxt;
        }
        let x = cur.borrow().idx as i32;
        x
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */
