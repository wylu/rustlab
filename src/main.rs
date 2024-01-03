use std::thread;
use std::time;

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        let delay = time::Duration::from_secs(1);
        let now = time::Instant::now();
        println!("{:?}", now);
        thread::sleep(delay);
    }
}

fn main() {
    _ = dead_end;
    forever();
}
