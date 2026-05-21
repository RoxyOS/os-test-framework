#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![allow(clippy::assertions_on_constants)]

use core::fmt::Arguments;
use std::process::exit;

use os_test_framework::{ExitState, Platform, init_platform, test};

struct TestPlatform;

impl Platform for TestPlatform {
    fn print(&mut self, args: Arguments) {
        print!("{args}");
    }

    fn exit(&self, state: os_test_framework::ExitState) -> ! {
        match state {
            ExitState::Success => exit(0),
            ExitState::Failed => panic!("Test shouldn't fail"),
        }
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    init_platform(TestPlatform);
    os_test_framework::run_tests(tests);
}

test!(test1);
fn test1() {
    assert!(true);
}

test!(test2);
fn test2() {
    assert!(true);
}
