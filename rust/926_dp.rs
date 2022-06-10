impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let (mut dp0, mut dp1) = (0, 0);
        for ch in s.as_bytes() {
            match ch {
                b'0' => {
                    dp1 = dp0.min(dp1) + 1;
                },
                b'1' => {
                    dp1 = dp0.min(dp1);
                    dp0 += 1;
                },
                _ => { panic!("error"); }
            }
        }
        dp0.min(dp1)
    }
}
