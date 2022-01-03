use core::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, process, thread};

static INITED: AtomicBool = AtomicBool::new(false);
static EXITING: AtomicBool = AtomicBool::new(false);

pub fn exit() {
    let result = EXITING.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
    if result.is_ok() {
        thread::Builder::new()
            .name("will_exit".to_string())
            .spawn(|| {
                thread::sleep(Duration::from_secs(2));
                process::exit(0);
            })
            .unwrap();
    }
}
pub fn will_exit() -> bool {
    EXITING.load(Ordering::Acquire)
}

pub fn init() -> Result<(), io::Error> {
    let result = INITED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
    if result.is_ok() {
        unsafe {
            signal_hook_registry::register(libc::SIGINT, exit)?;
            signal_hook_registry::register(libc::SIGTERM, exit)?;
            signal_hook_registry::register(libc::SIGABRT, exit)?;
            #[cfg(not(windows))]
            signal_hook_registry::register(libc::SIGQUIT, exit)?;
        }
    }
    Ok(())
}
