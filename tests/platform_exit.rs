#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use core::fmt::Arguments;
use core::panic;
use std::process::exit;

use os_test_framework::{
    test, {ExitState, Platform, init_platform, platform},
};

struct TestPlatform;

impl Platform for TestPlatform {
    fn print(&mut self, args: Arguments) {
        print!("{args}");
    }

    fn exit(&self, state: os_test_framework::ExitState) -> ! {
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

test!(platform_exit);
fn platform_exit() {
    println!();
    platform().lock().exit(ExitState::Failed)
}
