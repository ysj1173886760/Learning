impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        // let mut s = String::new();
        // for x in address.chars() {
        //     if x == '.' {
        //         s.push_str("[.]")
        //     } else {
        //         s.push(x)
        //     }
        // }
        // s
        address.replace(".", "[.]")
    }
}
