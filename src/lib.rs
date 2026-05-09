#![no_std]

use alloc::boxed::Box;
use owo_colors::OwoColorize;
use spin::{Mutex, Once};

use crate::platform::{ExitState, Platform, platform};

extern crate alloc;

pub mod make_test;
pub mod panic;
pub mod platform;
pub mod printing;

pub fn run_tests(tests: &[&dyn Fn()]) -> ! {
    println!("\nRunning {} tests", tests.len().bold());

    for test in tests {
        test()
    }

    platform().lock().exit(ExitState::Success);
}

pub fn _run_test(name: &str, func: impl FnOnce()) {
    print!("{}", name);
    func();
    println!(" {}", "OK".green().bold());
}
