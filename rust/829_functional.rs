impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        // x + (x + 1) + (x + 2) + ... + y
        // equals (1 + 2 + ... + y - x) + (y - x + 1) * x
        // we want above formula equals to n
        // thus we have (1 + y - x) * (y - x) / 2 + (y - x + 1) * x == n
        // let = k (y - x + 1), then we have k * (k + 1) / 2 + k * x == n
        // so we can enumerate k, as long as (n - k * (k + 1) / 2) % k == 0
        // we can have k consecutive numbers which sum equals to n
        // since k is smaller than sqrt(n), so the overall complexity is O(sqrt(n))
        (1..=((n * 2) as f64).sqrt() as i32)
            .filter(|i| (n - i * (i + 1) / 2) % i == 0)
            .count() as i32
    }
}
