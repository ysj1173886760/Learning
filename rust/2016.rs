impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut minn = i32::MAX;
        let mut ans = -1;
        for i in nums {
            if i > minn {
                ans = std::cmp::max(ans, i - minn);
            }
            minn = std::cmp::min(minn, i);
        }
        ans
    }
}
