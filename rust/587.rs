impl Solution {
    fn cross(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
        (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0])
    }
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let n = trees.len();
        let mut vis = vec![false; n];

        if (n < 4) {
            return trees;
        }

        let leftMost = trees.iter()
                            .enumerate()
                            .fold(0, |minn, (i, val)| {
                                if val[0] < trees[minn][0] { i } else { minn }
                            });
        // println!("leftMost {}", leftMost);

        let mut p = leftMost;
        loop {
            let mut q = (p + 1) % n;

            // find right most point
            for (i, pos) in trees.iter().enumerate() {
                if Solution::cross(&trees[p], &trees[q], pos) < 0 {
                    q = i;
                }
            }

            // check the point at the same line
            for (i, pos) in trees.iter().enumerate() {
                if vis[i] || i == p || i == q {
                    continue;
                }
                if Solution::cross(&trees[p], &trees[q], pos) == 0 {
                    ans.push(pos.clone());
                    vis[i] = true;
                }
            }

            // append the right most one
            if !vis[q] {
                ans.push(trees[q].clone());
                vis[q] = true;
            }

            if q == leftMost {
                break;
            }

            p = q;
        }
        ans
    }
}
