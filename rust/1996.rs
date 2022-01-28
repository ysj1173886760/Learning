impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        let mut maxx = 0;
        let mut tmp_max = 0;
        let mut res = 0;
        for i in (0..properties.len()).rev() {
            if i < properties.len() - 1 &&
               properties[i][0] != properties[i + 1][0] {
                maxx = std::cmp::max(maxx, tmp_max);
            }
            if properties[i][1] < maxx {
                res += 1;
            }
            tmp_max = std::cmp::max(tmp_max, properties[i][1]);
            println!("{} {}", tmp_max, maxx);
        }
        res
    }
}
