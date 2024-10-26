use std::{thread, time::Duration};

fn main() {
    println!("Hello from the daemon");

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("still here!")
    }
}
