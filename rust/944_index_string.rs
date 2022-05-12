impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut cnt = 0;
        let len = strs[0].len();
        for i in 0..len {
            let mut lst = 0;
            for j in 0..strs.len() {
                if strs[j].as_bytes()[i] < lst {
                    cnt += 1;
                    break;
                }
                lst = strs[j].as_bytes()[i];
            }
        }
        cnt
    }
}
