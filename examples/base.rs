use core::time::Duration;
use std::thread;

fn main() {
    will_exit::init(2000).unwrap();
    std::panic::set_hook(Box::new(|info| {
        println!("{}", info);
        will_exit::exit(-1);
    }));

    thread::spawn(|| {
        thread::sleep(Duration::from_secs(6));
        will_exit::exit(0);
    });
    loop {
        if will_exit::will_exit() {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("exit");
}
