impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut ans = 0;
        let mut cur_path: Vec<i32> = vec![];
        let mut cur_length = 0;
        let input = input.as_bytes();

        let mut ptr = 0;
        while ptr < input.len() {
            let mut level = 0;
            while ptr < input.len() && input[ptr] == b'\t' {
                level += 1;
                ptr += 1;
            }

            while cur_path.len() > level {
                cur_length -= cur_path.pop().unwrap();
            }

            let mut is_file = false;
            let mut cnt = 0;
            while (ptr < input.len() && input[ptr] != b'\n') {
                if (input[ptr] == b'.') {
                    is_file = true;
                }
                cnt += 1;
                ptr += 1;
            }
            // skip \n
            ptr += 1;

            // println!("cnt {} cur_length {} cur_path {:?}", cnt, cur_length, cur_path);
            if is_file {
                ans = std::cmp::max(ans, cur_length + cnt + level as i32);
            } else {
                cur_path.push(cnt);
                cur_length += cnt;
            }
        }

        ans
    }
}
