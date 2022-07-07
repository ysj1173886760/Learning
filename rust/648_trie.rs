use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
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

impl Solution {
    fn insert(root: Rc<RefCell<Node>>, word: &str) {
        let mut cur = root;
        for ch in word.as_bytes() {
            let index = (ch - b'a') as usize;
            if cur.borrow().children[index].is_none() {
                cur.borrow_mut().children[index] = Some(Rc::new(RefCell::new(Node::new())));
            }
            let nxt = cur.borrow().children[index].clone().unwrap();
            cur = nxt
        }
        cur.borrow_mut().is_end = true;
    }

    fn get(root: Rc<RefCell<Node>>, word: &str) -> String {
        let mut cur = root;
        let mut res = String::new();
        for ch in word.as_bytes() {
            let index = (ch - b'a') as usize;
            if cur.borrow().is_end {
                return res;
            }
            if cur.borrow().children[index].is_none() {
                return word.to_string();
            }
            res.push(*ch as char);
            let nxt = cur.borrow().children[index].clone().unwrap();
            cur = nxt
        }
        if cur.borrow().is_end {
            return res;
        } else {
            return word.to_string();
        }
    }

    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new()));
        for word in dictionary.iter() {
            Self::insert(root.clone(), word);
        }
        sentence.split(' ').map(|s| Self::get(root.clone(), s)).collect::<Vec<String>>().join(" ")
    }
}
