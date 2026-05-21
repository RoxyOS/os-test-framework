//! # os-test-framework
//!
//! Test framework for embedded systems and OS kernels.
//!
//! `os-test-framework` requires `alloc`, so your kernel will need a `global-allocator`.
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
//! `Platform::print` and finishes the run through `Platform::exit`:
//!
//! ```rust,ignore
//! use core::fmt::Arguments;
//! use os_test_framework::{ExitState, Platform};
//!
//! struct MyPlatform;
//!
//! impl Platform for MyPlatform {
//!     fn print(&mut self, args: Arguments) {
//!         let _ = args;
//!         todo!()
//!     }
//!
//!     fn exit(&self, state: ExitState) -> ! {
//!         match state {
//!             ExitState::Success => todo!(),
//!             ExitState::Failed => todo!(),
//!         }
//!     }
//! }
//! ```
//!
//! Call `init_platform` with your `Platform`, and `test_main` from your kernel
//! entry point:
//!
//! ```rust,ignore
//! use os_test_framework::init_platform;
//!
//! fn kernel_entry() {
//!     init_platform(MyPlatform);
//!     test_main();
//! }
//! ```
//!
//! Forward panics from your OS to `os-test-framework`:
//!
//! ```rust,ignore
//! os_test_framework::forward_panic!();
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

extern crate alloc;

mod panic;
mod platform;
#[doc(hidden)]
pub mod __private {
    pub use crate::_run_test;
    pub use crate::printing::_print;
}

pub use crate::{panic::*, platform::*};

mod make_test;
mod printing;

pub fn run_tests(tests: &[&dyn Fn()]) -> ! {
    println!("\nRunning {} tests", tests.len().bold());

    for test in tests {
        test()
    }

    platform().lock().exit(ExitState::Success);
}

#[doc(hidden)]
pub fn _run_test(name: &str, func: fn()) {
    print!("{}", name);
    func();
    println!(" {}", "OK".green().bold());
}
