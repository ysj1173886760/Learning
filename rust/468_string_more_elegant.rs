impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if query_ip.find('.').is_none() {
            if Self::process_ipv6(query_ip) {
                return "IPv6".to_string();
            }
        } else {
            if Self::process_ipv4(query_ip) {
                return "IPv4".to_string();
            }
        }
        return "Neither".to_string();
    }
    fn process_ipv4(query_ip: String) -> bool {
        if query_ip.split('.').count() != 4 {
            return false;
        }
        for item in query_ip.split('.') {
            match item.parse::<u8>() {
                Ok(val) if item == &val.to_string() => (),
                _ => return false,
            }
        }
        true
    }
    fn process_ipv6(query_ip: String) -> bool {
        if query_ip.split(':').count() != 8 {
            return false;
        }
        for item in query_ip.split(':') {
            if !(1..=4).contains(&item.len()) {
                return false;
            }
            if item.chars().any(|c| !"0123456789ABCDEFabcdef".contains(c)) {
                return false;
            }
        }
        true
    }
}
