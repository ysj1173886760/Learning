impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut ans = vec![];
        let (mut lb, mut ub) = (0, s.len());
        for ch in s.chars() {
            match ch {
                'I' => {
                    ans.push(lb as i32);
                    lb += 1;
                }
                'D' => {
                    ans.push(ub as i32);
                    ub -= 1;
                }
                _ => {
                    panic!("wtf");
                }
            }
        }
        ans.push(lb);
        ans
    }
}
