impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut first, mut second) = (0, nums.len() - 1);
        while first < second {
            while first < second && (nums[first] % 2 == 0) {
                first += 1;
            }
            while first < second && (nums[second] % 2 == 1) {
                second -= 1;
            }
            if first != second {
                nums.swap(first, second);
                first += 1;
                second -= 1;
            }
        }

        nums
    }
}
