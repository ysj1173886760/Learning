impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if (s.is_empty()) {
            return 0;
        }
        return if s.chars().rev().collect::<String>() == s { 1 } else { 2 }
    }
}
