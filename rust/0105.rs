impl Solution {
    pub fn one_edit_away(mut first: String, mut second: String) -> bool {
        if first.len() < second.len() {
            let tmp = first;
            first = second;
            second = tmp;
        }

        if first.len() - second.len() > 1 {
            return false;
        }

        if first.len() == second.len() {
            let mut cnt = 0;
            for i in 0..first.len() {
                if first.as_bytes()[i] != second.as_bytes()[i] {
                    cnt += 1;
                }
            }
            return cnt <= 1;
        }

        let mut ptr = 0;
        let mut cnt = 0;
        for i in 0..first.len() {
            if ptr < second.len() && first.as_bytes()[i] == second.as_bytes()[ptr] {
                ptr += 1;
            } else {
                cnt += 1;
            }
        }
        return cnt <= 1;
    }
}
