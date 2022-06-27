impl Solution {
    fn check(s: &str, sub: &str) -> bool {
        if sub.len() > s.len() {
            return false;
        }
        let mut ptr = 0;
        for i in 0..sub.len() {
            while ptr < s.len() && 
                  s.as_bytes()[ptr] != sub.as_bytes()[i] {
                ptr += 1;
            }
            if ptr == s.len() {
                return false;
            }
            ptr += 1;
        }
        true
    }
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut ans = -1;
        for i in 0..strs.len() {
            let mut flag = true;
            for j in 0..strs.len() {
                if i == j {
                    continue;
                }
                if Self::check(&strs[j], &strs[i]) {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans = ans.max(strs[i].len() as i32);
            }
        }
        ans
    }
}
