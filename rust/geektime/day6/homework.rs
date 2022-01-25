use std::rc::Rc;

fn main() {
  let arr = vec![1];

  std::thread::spawn(|| {
    println!("{:?}", Rc::new(arr));
  });
}