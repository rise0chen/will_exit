use core::time::Duration;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::{io, process, thread};

static INITED: AtomicBool = AtomicBool::new(false);
static EXITING: AtomicBool = AtomicBool::new(false);
static EXIT_TIME: AtomicU16 = AtomicU16::new(0);
#[cfg(feature = "async")]
static EVENT: event_listener::Event = event_listener::Event::new();

pub fn exit(code: i32) -> ! {
    EXITING.store(true, Ordering::SeqCst);
    #[cfg(feature = "async")]
    EVENT.notify(usize::MAX);

    let exit_time = EXIT_TIME.load(Ordering::SeqCst);
    thread::sleep(Duration::from_millis(exit_time as u64));
    process::exit(code);
}
pub fn send_exit_signal(code: i32) {
    thread::Builder::new()
        .name("will_exit".to_string())
        .spawn(move || exit(code))
        .unwrap();
}

pub fn will_exit() -> bool {
    EXITING.load(Ordering::Acquire)
}
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub async fn wait_will_exit() {
    while EXITING.load(Ordering::Acquire) == false {
        EVENT.listen().await
    }
}

/// 设置调用exit()后多少毫秒后退出程序
pub fn init(exit_time: u16) -> Result<(), io::Error> {
    EXIT_TIME.store(exit_time, Ordering::SeqCst);
    let result = INITED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
    if result.is_ok() {
        unsafe {
            signal_hook_registry::register(libc::SIGINT, || exit(libc::SIGINT))?;
            signal_hook_registry::register(libc::SIGTERM, || exit(libc::SIGTERM))?;
            signal_hook_registry::register(libc::SIGABRT, || exit(libc::SIGABRT))?;
            #[cfg(not(windows))]
            signal_hook_registry::register(libc::SIGQUIT, || exit(libc::SIGQUIT))?;
        }
    }
    Ok(())
}
