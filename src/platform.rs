use core::fmt::Arguments;

use alloc::boxed::Box;
use spin::{Mutex, Once};

static PLATFORM: Once<Mutex<Box<dyn Platform>>> = Once::new();

pub fn init_platform(platform: impl Platform + 'static) {
    PLATFORM.call_once(|| Mutex::new(Box::new(platform)));
}

pub fn platform() -> &'static Mutex<Box<dyn Platform>> {
    try_platform().expect("os-test-framework platform not initialized")
}

pub fn try_platform() -> Option<&'static Mutex<Box<dyn Platform>>> {
    PLATFORM.get()
}

pub enum ExitState {
    Success,
    Failed,
}

pub trait Platform: Send + Sync {
    fn print(&mut self, args: Arguments);
    fn exit(&self, state: ExitState) -> !;
}
