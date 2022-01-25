use std::sync::Arc;

fn main() {
    let s1 = Arc::new("Hello World");
    let s2 = s1.clone();

    std::thread::spawn(move || {
        println!("{}", s1);
    });

    println!("{}", s2);
}