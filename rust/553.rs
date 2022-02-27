impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 1 {
            return nums[0].to_string();
        }
        let mut s = String::new();
        for i in nums.iter().skip(1) {
            s.push_str(&(i.to_string() + "/"));
        }
        let s = &s[0..s.len() - 1];
        if nums.len() == 2 {
            format!("{}/{}", nums[0].to_string(), s)
        } else {
            format!("{}/({})", nums[0].to_string(), s)
        }
    }
}
