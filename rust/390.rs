impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let (mut a1, mut cnt, mut step, mut k) = (1, n, 1, 0);
        while cnt > 1 {
            if k % 2 == 0 {
                a1 = a1 + step;
            } else {
                a1 = if cnt % 2 == 0 { a1 } else { a1 + step }
            }
            k += 1;
            cnt >>= 1;
            step <<= 1;
        }
        return a1;
    }
}
