fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    // error[E0382]: borrow of moved value: `s`
    // println!("在move进函数后继续使用s: {}", s);

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
}

fn takes_ownership(some_thing: String) {
    // some_thing 进入作用域
    println!("{}", some_thing);
} // 这里，some_thing 移除作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
