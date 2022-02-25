impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (mut sn1, mut sn2) = (num1.split("+"), num2.split("+"));
        let (x1, y1) = (sn1.next().unwrap().parse::<i32>().unwrap(), 
                        sn1.next().unwrap().trim_end_matches(|x| x == 'i').parse::<i32>().unwrap());
        let (x2, y2) = (sn2.next().unwrap().parse::<i32>().unwrap(), 
                        sn2.next().unwrap().trim_end_matches(|x| x == 'i').parse::<i32>().unwrap());
        format!("{}+{}i", x1 * x2 - y1 * y2, x1 * y2 + x2 * y1)
    }
}
