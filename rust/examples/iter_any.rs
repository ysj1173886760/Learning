fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    // println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));
    // println!("{:?}", vec2);
    let mut iter = vec2.into_iter();
    {
        let iter2 = iter.clone();
        println!("{:?}", iter2.collect::<Vec<i32>>());
    }

    // 捕获变量和参数是两回事，这里的any传入的参数是值类型的，而捕获的则是可变借用
    println!("2 in vec2: {}", iter.any(|x| x == 2));
    println!("{:?}", iter.collect::<Vec<i32>>());


    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
