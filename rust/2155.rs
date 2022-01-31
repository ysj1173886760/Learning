impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let (mut num0, mut num1) = (0, nums.iter().fold(0, |sum, x| x + sum));
        let mut res = vec![];
        let mut maxx = num1;
        res.push(0);

        for i in 0..nums.len() {
            if nums[i] == 0 {
                num0 += 1;
            } else {
                num1 -= 1;
            }
            if num0 + num1 > maxx {
                res.clear();
                res.push((i + 1) as i32);
                maxx = num0 + num1;
            } else if num0 + num1 == maxx {
                res.push((i + 1) as i32);
            }
        }
        res
    }
}
