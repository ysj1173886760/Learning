impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if query_ip.find('.').is_none() {
            Self::process_ipv6(query_ip)
        } else {
            Self::process_ipv4(query_ip)
        }
    }
    fn process_ipv4(query_ip: String) -> String {
        let mut item_cnt = 0;
        for item in query_ip.split('.') {
            let val = item.parse::<i32>();
            if val.is_err() {
                return String::from("Neither");
            }
            let mut val = val.unwrap();
            if val < 0 || val > 255 {
                return String::from("Neither");
            }
            if val == 0 {
                if item.len() != 1 {
                    return String::from("Neither");
                }
            } else {
                let mut cnt = 0;
                while val > 0 {
                    val /= 10;
                    cnt += 1;
                }
                if item.len() != cnt {
                    return String::from("Neither");
                }
            }
            item_cnt += 1;
        }
        if item_cnt != 4 {
            return String::from("Neither");
        }
        return String::from("IPv4");
    }
    fn process_ipv6(query_ip: String) -> String {
        let mut item_cnt = 0;
        for item in query_ip.split(':') {
            item_cnt += 1;
            if item.len() < 1 || item.len() > 4 {
                return String::from("Neither");
            }
            for &ch in item.as_bytes() {
                if (ch >= b'0' && ch <= b'9') || 
                   (ch >= b'a' && ch <= b'f') || 
                   (ch >= b'A' && ch <= b'F') {
                    continue;
                }
                return String::from("Neither");
            }
        }
        if item_cnt != 8 {
            return String::from("Neither");
        }
        return String::from("IPv6");
    }
}
