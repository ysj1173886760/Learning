impl Solution {
    fn dfs(cur: usize, nums: &Vec<i32>, vis: &mut Vec<bool>) -> i32 {
        vis[cur] = true;
        if !vis[nums[cur] as usize] {
            return Self::dfs(nums[cur] as usize, nums, vis) + 1;
        }
        1
    }
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut vis = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            if !vis[i] {
                ans = ans.max(Self::dfs(i, &nums, &mut vis));
            }
        }
        ans
    }
}
