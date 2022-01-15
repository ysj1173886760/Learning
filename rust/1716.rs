impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ans = 0;
        let mut lst = 0;
        let mut lst_mon = 0;
        for i in 0..n {
            if i % 7 == 0 {
                lst_mon += 1;
                ans += lst_mon;
                lst = lst_mon;
            } else {
                lst += 1;
                ans += lst;
            }
        }
        return ans;
    }
}
