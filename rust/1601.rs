impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let m = requests.len();
        let mut ans = 0;
        for i in 1..(1 << m) {
            let mut deg = vec![0; n as usize];
            let mut cnt = 0;
            for j in 0..m {
                if ((1 << j) & i) != 0 {
                    cnt += 1;
                    deg[requests[j][0] as usize] += 1;
                    deg[requests[j][1] as usize] -= 1;
                }
            }
            let mut flag = true;
            for j in 0..n as usize {
                if deg[j] != 0 {
                    flag = false;
                }
            }
            if flag {
                ans = std::cmp::max(ans, cnt);
            }
        }
        ans
    }
}
