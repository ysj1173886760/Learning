impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        // sigma {i=1..m} min(n, x / i)
        // x / i >= n
        let calc = |x: i32| -> i32 {
            (x / n) * n + (x / n + 1 ..=m).map(|i: i32| x / i).sum::<i32>()
        };
        let mut lb = 0;
        let mut ub = m * n;
        while ub - lb > 1 {
            let mid = (ub + lb) / 2;
            // println!("{} {}", calc(mid), mid);
            if calc(mid) >= k {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        ub
    }
}
