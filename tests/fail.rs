#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![allow(clippy::assertions_on_constants)]

use std::{fmt::Write, panic, process::exit};

use os_test_framework::{
    platform::{Platform, init_platform},
    test,
};

struct TestPlatform;

impl Platform for TestPlatform {
    fn exit(&self, _state: os_test_framework::platform::ExitState) -> ! {
        unreachable!()
    }
}

impl Write for TestPlatform {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        print!("{s}");
        Ok(())
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
