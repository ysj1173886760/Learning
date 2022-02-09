impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let mut ans = vec![];
        for i in 2..n + 1 {
            for j in 1..i {
                if gcd(i, j) == 1 {
                    ans.push(format!("{}/{}", j, i));
                }
            }
        }
        ans
    }
}
