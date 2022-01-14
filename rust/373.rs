use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut k = k;
        let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::from(
            (0..nums1.len())
            .map(|i| (-nums1[i] - nums2[0], i, 0))
            .collect::<Vec<(i32, usize, usize)>>()
        );
        while let Some((_, i, j)) = pq.pop() {
            if k == 0 {
                break;
            }
            k -= 1;
            res.push(vec![nums1[i], nums2[j]]);
            if (j + 1 < nums2.len()) {
                pq.push((-nums1[i] - nums2[j + 1], i, j + 1));
            }
        }
        res
    }
}
