use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for email in emails.iter() {
            let mail = email.split('@').collect::<Vec<_>>();
            let mut cur = String::new();
            for &ch in mail[0].as_bytes() {
                match ch {
                    b'.' => {},
                    b'+' => break,
                    _ => cur.push(ch as char),
                }
            }
            cur.push('@');
            cur.push_str(mail[1]);
            set.insert(cur);
        }
        set.len() as i32
    }
}
