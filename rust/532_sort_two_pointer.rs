impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut dup_cnt = 0;
        {
            let mut i = 0;
            while i < nums.len() {
                let mut nxt = i;
                while nxt < nums.len() && nums[nxt] == nums[i] {
                    nxt += 1;
                }
                if nxt != i + 1 {
                    dup_cnt += 1;
                }
                i = nxt;
            }
        }
        if k == 0 {
            return dup_cnt;
        }
        nums.dedup();
        let mut left = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            while nums[i] - nums[left] > k {
                left += 1
            }
            if nums[i] - nums[left] == k {
                ans += 1;
            }
        }
        ans
    }
}
