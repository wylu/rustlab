use std::fmt::Debug;

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn main() {
    report("hello");

    let mut text = String::from("hello");
    clear(&mut text);
    println!("text: {}", text);
}
