impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by(|lhs, rhs| {
            let (ident1, content1) = lhs.split_once(' ').unwrap();
            let (ident2, content2) = rhs.split_once(' ').unwrap();
            let is_digit1 = content1.find(|c: char| c.is_lowercase()).is_none();
            let is_digit2 = content2.find(|c: char| c.is_lowercase()).is_none();

            if is_digit1 && is_digit2 {
                return std::cmp::Ordering::Equal;
            } else if !is_digit1 && is_digit2 {
                return std::cmp::Ordering::Less;
            } else if is_digit1 && !is_digit2 {
                return std::cmp::Ordering::Greater;
            } else {
                return content1.cmp(content2).then(ident1.cmp(ident2));
            }
        });
        logs
    }
}
