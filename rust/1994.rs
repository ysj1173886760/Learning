impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut dp = vec![0; (1 << primes.len())];
        let modd = 1e9 as usize + 7;
        let mut cnt = vec![0; 31];
        for i in nums.iter() {
            cnt[*i as usize] += 1;
        }
        dp[0] = 1;
        for i in (0..cnt[1]) {
            dp[0] = dp[0] * 2 % modd;
        }
        for i in (1..=30) {
            if cnt[i] == 0 {
                continue;
            }
            let mut flag = false;
            let mut state = 0;
            for (idx, &prime) in primes.iter().enumerate() {
                if i % (prime * prime) == 0 {
                    flag = true;
                    break;
                }
                if i % prime == 0 {
                    state |= (1 << idx);
                }
            }
            if flag {
                continue;
            }

            for mask in (1..(1 << primes.len())).rev() {
                if (mask & state) == state {
                    dp[mask] = (dp[mask] + dp[mask ^ state] * cnt[i]) % modd;
                }
            }
        }
        let mut ans = 0;
        for mask in (1..(1 << primes.len())) {
            ans = (ans + dp[mask]) % modd;
        }
        ans as i32
    }
}
