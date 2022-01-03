use core::time::Duration;
use std::thread;
use will_exit;

fn main() {
    will_exit::init().unwrap();
    loop {
        if will_exit::will_exit() {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("exit");
}
