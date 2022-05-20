impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut table = intervals.iter()
                                 .enumerate()
                                 .map(|(i, vec)| (vec[0], i as i32))
                                 .collect::<Vec<(i32, i32)>>();
        table.sort_by(|a, b| a.0.cmp(&b.0));

        intervals.iter()
                 .map(|vec| Self::binary_search(&table, vec[1]))
                 .collect::<Vec<i32>>()
    }
    fn binary_search(array: &[(i32, i32)], elem: i32) -> i32 {
        let mut lb = -1;
        let mut ub = array.len() as i32;
        while ub - lb > 1 {
            let mid = (ub + lb) / 2;
            if array[mid as usize].0 >= elem {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        match ub {
            x if x == array.len() as i32 => -1,
            x => array[x as usize].1,
        }
    }
}
