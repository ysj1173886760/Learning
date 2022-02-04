impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cnt = 0;
        for rect in rectangles.iter() {
            let l = std::cmp::min(rect[0], rect[1]);
            if l == ans {
                cnt += 1;
            } else if l > ans {
                ans = l;
                cnt = 1;
            }
        }
        cnt
    }
}
