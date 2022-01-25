fn longest<'a>(x: &str, y: &str) -> &'a str {
    x
}

fn main() {
    let str1 = String::from("str1");
    let str2 = String::from("str2");
    let result = longest(str1.as_str(), str2.as_str());
    println!("{}", result);
}
