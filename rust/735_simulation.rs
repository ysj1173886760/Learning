impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut cur: Vec<i32> = vec![];
        for aster in asteroids {
            let mut alive = true;
            while cur.len() > 0 && cur[cur.len() - 1] > 0 && aster < 0 {
                if cur[cur.len() - 1].abs() > aster.abs() {
                    alive = false;
                    break;
                } else if cur[cur.len() - 1].abs() == aster.abs() {
                    alive = false;
                    cur.pop();
                    break;
                } else {
                    cur.pop();
                }
            }
            if alive {
                cur.push(aster);
            }
        }
        cur
    }
}
