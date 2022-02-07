impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut ans : Vec<char> = vec![];
        let mut cnt = vec![(a, 'a'), (b, 'b'), (c, 'c')];
        loop {
            let mut flag = false;
            cnt.sort();
            // println!("{:?}", cnt);
            for i in (0..3).rev() {
                if cnt[i].0 <= 0 {
                    break;
                }
                if ans.len() >= 2 && 
                    ans[ans.len() - 1] == cnt[i].1 &&
                    ans[ans.len() - 2] == cnt[i].1 {
                    continue;
                }
                cnt[i].0 -= 1;
                ans.push(cnt[i].1);
                flag = true;
                break;
            }
            if !flag {
                break;
            }
        }
        ans.into_iter().collect()
    }
}
