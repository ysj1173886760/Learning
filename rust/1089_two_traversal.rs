impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let zero_cnt = arr.iter().filter(|x| **x == 0).count();
        let mut lst = arr.len() + zero_cnt - 1;
        let old_len = arr.len();
        arr.resize(lst + 1, 0);
        for i in (0..old_len).rev() {
            if arr[i] == 0 {
                arr[lst] = arr[i];
                lst -= 1;
            }
            arr[lst] = arr[i];
            lst -= 1;
        }
        arr.resize(old_len, 0);
    }
}
