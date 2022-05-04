impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        fn process(n: i32, k: i32) -> i32 {
            if n == 1 { 0 } else { (process(n - 1, k) + k) % n }
        }
        process(n, k) + 1
    }
}
