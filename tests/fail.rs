#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![allow(clippy::assertions_on_constants)]

use core::fmt::Arguments;
use std::{panic, process::exit};

use os_test_framework::{
    platform::{Platform, init_platform},
    test,
};

struct TestPlatform;

impl Platform for TestPlatform {
    fn print(&mut self, args: Arguments) {
        print!("{args}");
    }

    fn exit(&self, _state: os_test_framework::platform::ExitState) -> ! {
        unreachable!()
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    panic::set_hook(Box::new(|_| {
        println!();
        println!("Test failed, exactly as we wanted.");
        exit(0)
    }));

    init_platform(TestPlatform);
    os_test_framework::run_tests(tests);
}

test! {
    "Should fail" {
        assert!(false);
    }
}
