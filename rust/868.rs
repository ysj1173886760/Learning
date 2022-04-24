impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut maxx = 0;
        let mut lst: Option<usize> = None;
        let mut ptr = 0;
        while n != 0 {
            if (n & 1) == 1 {
                match lst {
                    Some(pos) => { maxx = maxx.max(ptr - pos) }
                    None => ()
                }
                lst = Some(ptr);
            }

            ptr += 1;
            n >>= 1;
        }

        return maxx as i32;
    }
}
