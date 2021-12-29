impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input != 0 {
            digits.push(input % 10);
            input /= 10;
        }

        let mut reverse_vec = digits.clone();
        reverse_vec.reverse();
        return digits == reverse_vec;
    }
}