fn main() {
    let names = vec!["Bob", "Frank", "Ferris", "sheep"];

    // moved as set level, not element level
    for name in names.into_iter() {
        match name {
            "Ferris" => {
                println!("There is a rustacean among us!");
                break;
            }
            _ => println!("Hello {}", name),
        }
    }
    println!("{}", names[3]);
}
