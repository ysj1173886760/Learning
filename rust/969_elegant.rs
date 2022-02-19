impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let n = arr.len();

        for i in (0..n - 1) {
            let (idx, _) = arr[0..n - i].iter().enumerate().max_by_key(|x| x.1).unwrap();
            if idx != n - i - 1 {
                ans.push((idx + 1) as i32);
                ans.push((n - i) as i32);
                arr[0..idx + 1].reverse();
                arr[0..n - i].reverse();
            }
        }

        ans
    }
}
