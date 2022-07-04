impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut ans = vec![];
        let mut minn = i32::MAX;
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] < minn {
                ans = vec![];
                minn = arr[i] - arr[i - 1];
            }
            if arr[i] - arr[i - 1] == minn {
                ans.push(vec![arr[i - 1], arr[i]])
            }
        }
        ans
    }
}
