impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let atoi = |s: &[u8]| ((s[0] - '0' as u8) * 10 + (s[1] - '0' as u8)) as i32;
        let mut minutes: Vec<_> = time_points
            .iter()
            .map(|time| {
                let time = time.as_bytes();
                atoi(&time[..2]) * 60 + atoi(&time[3..])
            })
            .collect();
        minutes.sort();
        minutes.push(minutes[0] + 1440);
        (1..minutes.len()).map(|i| minutes[i] - minutes[i - 1]).min().unwrap()
    }
}
