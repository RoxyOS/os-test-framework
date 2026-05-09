//! # os-test-framework
//!
//! Test framework for embedded systems and OS kernels.
//!
//! ## Getting Started
//!
//! First, enable `custom_test_frameworks`, set `test_runner` as the test runner
//! from `os-test-framework`, and `reexport_test_harness_main`.
//!
//! ```rust,ignore
//! #![feature(custom_test_frameworks)]
//! #![reexport_test_harness_main = "test_main"]
//! #![test_runner(os_test_framework::run_tests)]
//! ```
//!
//! Implement `Platform`. The framework writes output through
//! `core::fmt::Write` and finishes the run through `Platform::exit`:
//!
//! ```rust,ignore
//! use core::fmt::Write;
//!
//! use os_test_framework::platform::{ExitState, Platform};
//!
//! struct MyPlatform;
//!
//! impl Platform for MyPlatform {
//!     fn exit(&self, state: ExitState) -> ! {
//!         match state {
//!             ExitState::Success => todo!(),
//!             ExitState::Failed => todo!(),
//!         }
//!     }
//! }
//!
//! impl Write for MyPlatform {
//!     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//!         // Write to serial, frame buffer, UART, etc.
//!         let _ = s;
//!         todo!()
//!     }
//! }
//! ```
//!
//! Call `init_platform` with your `Platform`, and `test_main` from your kernel
//! entry point:
//!
//! ```rust,ignore
//! use os_test_framework::platform::init_platform;
//!
//! fn kernel_entry() {
//!     init_platform(MyPlatform);
//!     test_main();
//! }
//! ```
//!
//! ## Adding A Test
//!
//! You can declare tests like this:
//!
//! ```rust,ignore
//! use os_test_framework::test;
//!
//! test! {
//!     "Hello" {
//!         assert!(true);
//!     }
//! }
//! ```
//!
//! You can add multiple tests in the same file:
//!
//! ```rust,ignore
//! use os_test_framework::test;
//!
//! test! {
//!     "Hello" {
//!         assert!(true);
//!     }
//! }
//!
//! test! {
//!     "Hello2" {
//!         assert!(true);
//!     }
//! }
//! ```
#![no_std]

use owo_colors::OwoColorize;

use crate::platform::{ExitState, platform};

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
