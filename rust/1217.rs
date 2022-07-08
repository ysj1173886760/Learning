impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 2];
        for x in position.iter() {
            if x % 2 == 0 {
                cnt[0] += 1;
            } else {
                cnt[1] += 1;
            }
        }
        cnt[0].min(cnt[1])
    }
}
