impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let (mut month, mut year) = (month, year);
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let c = year / 100;
        year %= 100;
        let week = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let res = c / 4 - 2 * c + year + year / 4 + (13 * (month + 1)) / 5 + day - 1;
        week[((res % 7 + 7) % 7) as usize].to_string()
    }
}
