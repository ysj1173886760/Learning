use std::rc::Rc;
use std::cell::RefCell;
struct Node {
    children: Vec<Option<Rc<RefCell<Node>>>>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: vec![None; 26],
            is_end: false,
        }
    }
}

struct MagicDictionary {
    root: Rc<RefCell<Node>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new())),
        }
    }

    fn insert(&self, word: &str) {
        let mut cur = self.root.clone();
        for ch in word.bytes() {
            let index = (ch - b'a') as usize;
            if cur.borrow().children[index].is_none() {
                cur.borrow_mut().children[index] = Some(Rc::new(RefCell::new(Node::new())));
            }
            let nxt = cur.borrow().children[index].clone().unwrap();
            cur = nxt;
        }
        cur.borrow_mut().is_end = true;
    }
    
    fn build_dict(&self, dictionary: Vec<String>) {
        for word in dictionary {
            self.insert(&word);
        }
    }

    fn dfs(root: Rc<RefCell<Node>>, word: &str, cur: usize, modified: bool) -> bool {
        if cur == word.len() {
            return modified && root.borrow().is_end;
        }
        let index = (word.as_bytes()[cur] - b'a') as usize;
        if root.borrow().children[index].is_some() {
            if Self::dfs(root.borrow().children[index].clone().unwrap(), word, cur + 1, modified) {
                return true;
            }
        }
        if !modified {
            for idx in 0..26 {
                if idx != index && root.borrow().children[idx].is_some() &&
                    Self::dfs(root.borrow().children[idx].clone().unwrap(), word, cur + 1, true) {
                    return true;
                }
            }
        }
        false
    }
    
    fn search(&self, search_word: String) -> bool {
        Self::dfs(self.root.clone(), &search_word, 0, false)
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
