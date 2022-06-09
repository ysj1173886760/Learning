use rand;
struct Solution {
    sum: Vec<i32>,
    rects: Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(rects: Vec<Vec<i32>>) -> Self {
        let get_area = |rect: &Vec<i32>| { (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1) };
        let mut sum = vec![0; rects.len()];
        sum[0] = get_area(&rects[0]);
        for i in 1..sum.len() {
            sum[i] = sum[i - 1] + get_area(&rects[i]);
        }
        Self {
            sum: sum,
            rects: rects
        }
    }
    
    fn pick(&self) -> Vec<i32> {
        let mut ub: i32 = self.sum.len() as i32 - 1;
        let mut lb: i32 = -1;
        let target = (rand::random::<u32>() % self.sum[self.sum.len() - 1] as u32) as i32;
        while ub - lb > 1 {
            let mid = (ub + lb) / 2;
            if target < self.sum[mid as usize] {
                ub = mid
            } else {
                lb = mid;
            }
        }

        let ub = ub as usize;
        let pre = if ub == 0 { 0 } else { self.sum[ub - 1] };
        let dx = (target - pre) % (self.rects[ub][2] - self.rects[ub][0] + 1);
        let dy = (target - pre) / (self.rects[ub][2] - self.rects[ub][0] + 1);
        if target > self.sum[ub] || target < pre {
            panic!("fuck")
        }
        vec![self.rects[ub][0] + dx, self.rects[ub][1] + dy]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
