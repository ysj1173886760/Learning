fn main() {
    let mut arr = vec![1];
    let ir = &arr;
    arr.push(2);
    println!("{:?}", arr);
}