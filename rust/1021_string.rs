impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = String::new();
        let mut stack = vec![];
        for (i, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(i);
            } else {
                let begin = stack.pop().unwrap();
                if stack.is_empty() {
                    ans.push_str(&s[begin + 1..i]);
                }
            }
        }
        ans
    }
}
