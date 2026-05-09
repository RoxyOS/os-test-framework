#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use core::panic;
use core::fmt::Arguments;
use std::process::exit;

use os_test_framework::{
    platform::{ExitState, Platform, init_platform, platform},
    test,
};

struct TestPlatform;

impl Platform for TestPlatform {
    fn print(&mut self, args: Arguments) {
        print!("{args}");
    }

    fn exit(&self, state: os_test_framework::platform::ExitState) -> ! {
        match state {
            ExitState::Success => panic!("Test shouldent succeed"),
            ExitState::Failed => exit(0),
        }
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    init_platform(TestPlatform);
    os_test_framework::run_tests(tests);
}

test! {
    "Platform exit" {
        println!();
        platform().lock().exit(ExitState::Failed)
    }
}
