impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let n = arr.len();

        fn flip(arr: &mut Vec<i32>, k: usize) {
            for i in (0..k / 2) {
                let tmp = arr[i];
                arr[i] = arr[k - i - 1];
                arr[k - i - 1] = tmp;
            }
        }

        for i in (0..n - 1) {
            let mut idx = 0;
            for j in (0..n - i) {
                if arr[j] > arr[idx] {
                    idx = j;
                }
            }
            if idx != n - i - 1 {
                ans.push((idx + 1) as i32);
                ans.push((n - i) as i32);
                flip(&mut arr, idx + 1);
                flip(&mut arr, n - i);
            }
        }

        ans
    }
}
