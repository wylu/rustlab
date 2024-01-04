fn main() {
    // 基本类型都是通过自动拷贝的方式来赋值的，不涉及所有权转移
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // String 不是基本类型，而且是存储在堆上的，因此不能自动拷贝
    let s1 = String::from("hello");
    // 当 s1 赋予 s2 后，Rust 认为 s1 不再有效，
    // 因此也无需在 s1 离开作用域后 drop 任何东西，
    // 这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。
    let s2 = s1;
    // error[E0382]: borrow of moved value: `s1`
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // x 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权。
    let x: &str = "hello, world";
    // 因此 let y = x 中，仅仅是对该引用进行了拷贝，此时 y 和 x 都引用了同一个字符串。
    let y = x;
    println!("{}, {}", x, y);
}
