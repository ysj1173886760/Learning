impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ptr = 0;
        let mut cur = 1;
        let mut ans = 0;
        for (i, &x) in nums.iter().enumerate() {
            cur *= x;
            while cur >= k && ptr <= i {
                cur /= nums[ptr];
                ptr += 1;
            }
            ans += i - ptr + 1;
        }
        ans as i32
    }
}
