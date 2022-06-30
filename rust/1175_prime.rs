impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let n = n as usize;
        let mut prime = vec![true; n + 1];
        let mut cnt = 0;
        let md = 1e9 as usize + 7;
        for i in 2..=n {
            if prime[i] {
                cnt += 1;
                let mut j = i;
                while j <= n {
                    prime[j] = false;
                    j += i;
                }
            }
        }
        (((1..=cnt).fold(1, |prod, x| (prod * x) % md) * (1..=n - cnt).fold(1, |prod, x| (prod * x) % md)) % md) as i32
    }
}
