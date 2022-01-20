impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = vec![0; 3];
        for x in stones.iter() {
            cnt[(x % 3) as usize] += 1;
        }
        if cnt[0] % 2 == 0 {
            return cnt[1] >= 1 && cnt[2] >= 1;
        }
        cnt[1] - cnt[2] > 2 || cnt[2] - cnt[1] > 2
    }
}
