impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: i32 = m - 1;
        let mut j: i32 = n - 1;
        let mut ptr: i32 = m + n - 1;
        while ptr >= 0 {
            if i >= 0 && (j < 0 || nums1[i as usize] >= nums2[j as usize]) {
                nums1[ptr as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[ptr as usize] = nums2[j as usize];
                j -= 1;
            }
            ptr -= 1;
        }
    }
}