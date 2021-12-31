impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut sum = 0;
        let mut i = 1;
        while i * i <= num {
            if num % i == 0 {
                sum += i;
                if (i * i != num) {
                    sum += num / i;
                }
            }
            i += 1;
        }
        println!("{}", sum);
        return sum == num * 2;
    }
}