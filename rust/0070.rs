impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let (mut prev, mut curr) = (1, 2);
        for i in 2..n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        return curr;
    }
}