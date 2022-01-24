fn main() {
  let mut arr = vec![1, 2, 3];
  // cache the last item
  let last = arr.last();
  // consume previously stored last item
  println!("last: {:?}", last);
  arr.push(4);
}