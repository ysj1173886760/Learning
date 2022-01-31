use std::collections::{
    HashMap,
    HashSet,
    VecDeque
};
impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        fn to_bit(s: &String) -> i32 {
            let mut x = 0;
            for ch in s.chars() {
                x |= 1 << (ch as usize - 'a' as usize);
            }
            x
        }
        let mut mp: HashMap<i32, i32> = HashMap::new();
        for word in words.iter() {
            *mp.entry(to_bit(word)).or_insert(0) += 1
        }

        fn get_edges(x: i32) -> Vec<i32> {
            let mut vec = vec![];
            for i in 0..26 {
                vec.push(x ^ (1 << i));
                if ((x >> i) & 1) != 0 {
                    for j in 0..26 {
                        if ((x >> j) & 1) == 0 {
                            vec.push(x ^ (1 << i) ^ (1 << j));
                        }
                    }
                }
            }
            vec
        }

        let mut ans = vec![0, 0];
        let mut vis = HashSet::new();
        for (k, v) in mp.iter() {
            if vis.contains(k) {
                continue;
            }

            let mut q: VecDeque<i32> = VecDeque::new();
            q.push_back(*k);
            let mut cnt = *v;
            vis.insert(*k);
            
            while !q.is_empty() {
                let x = q.pop_front().unwrap();
                for to in get_edges(x) {
                    if mp.contains_key(&to) && !vis.contains(&to){
                        vis.insert(to);
                        cnt += mp.get(&to).unwrap();
                        q.push_back(to);
                    }
                }
            }
            println!("{}", cnt);
            ans[0] += 1;
            ans[1] = std::cmp::max(ans[1], cnt);
        }
        ans
    }
}
