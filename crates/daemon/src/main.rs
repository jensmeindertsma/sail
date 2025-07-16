use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}
