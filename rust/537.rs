impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        fn process(s: String) -> (i32, i32) {
            let s: Vec<char> = s.chars().collect();
            let mut cur = 0;
            let (mut x, mut y) = (0, 0);
            let (mut sign_x, mut sign_y) = (1, 1);
            if s[cur] == '-' {
                cur += 1;
                sign_x = -1;
            }
            while s[cur] != '+' {
                x = x * 10 + s[cur] as i32 - '0' as i32;
                cur += 1;
            }
            cur += 1;
            if s[cur] == '-' {
                cur += 1;
                sign_y = -1;
            }
            while s[cur] != 'i' {
                y = y * 10 + s[cur] as i32 - '0' as i32;
                cur += 1;
            }
            return (x * sign_x, y * sign_y);
        }
        let (x1, y1) = process(num1);
        let (x2, y2) = process(num2);
        let mut x = x1 * x2 - y1 * y2;
        let mut y = x1 * y2 + x2 * y1;
        x.to_string() + "+" + &y.to_string() + "i"
    }
}
