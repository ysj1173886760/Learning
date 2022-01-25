pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let r = &mut s1;
    let hello;
    {
        hello = strtok(r, ' ');
    }
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}

// s1的可变引用的生命周期应该只在strtok中
// 如果将可变引用的生命周期与返回值的生命周期绑定，那么就相当于让s1的可变引用的生命周期与hello相同
// 那么后一句中s1就会被认定冲突，因为还存在着对s1的可变引用