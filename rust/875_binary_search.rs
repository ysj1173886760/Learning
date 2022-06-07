impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut lb = 0;
        let mut ub = 1e9 as i32;
        while ub - lb > 1 {
            let mid = (ub + lb) / 2;
            if piles.iter().map(|&x| (x + mid - 1) / mid).sum::<i32>() <= h {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        ub
    }
}
