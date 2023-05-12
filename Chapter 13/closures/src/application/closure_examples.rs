use std::{thread, time::Duration};

pub fn expensive_closure() -> impl Fn(u64) -> u64 {
    |num: u64| -> u64 {
        println!("calculating slowly... {num}");
        thread::sleep(Duration::from_secs(num));
        num
    }
}
