impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut k = k;
        let mut fib = vec![1, 1];
        loop {
            let x = fib[fib.len() - 1] + fib[fib.len() - 2];
            if x > k {
                break;
            }
            fib.push(x);
        }
        let mut ans = 0;
        for x in fib.iter().rev() {
            if k >= *x {
                k -= x;
                ans += 1;
            }
            if k == 0 {
                break;
            }
        }
        ans
    }
}
