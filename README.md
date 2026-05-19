# os-test-framework

Test framework for embedded systems and OS kernels.

`os-test-framework` requires `alloc`, so your kernel will need a `global-allocator`.

## Getting Started

First, enable `custom_test_frameworks`, set `test_runner` as the test runner from `os-test-framework`, and `reexport_test_harness_main`.

```rust
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(os_test_framework::run_tests)]
```

Implement `Platform`. The framework writes output through `Platform::print`
and finishes the run through `Platform::exit`:

```rust
use core::fmt::Arguments;
use os_test_framework::platform::{ExitState, Platform};

struct MyPlatform;

impl Platform for MyPlatform {
    fn print(&mut self, args: Arguments) {
        let _ = args;
        todo!()
    }

    fn exit(&self, state: ExitState) -> ! {
        match state {
            ExitState::Success => todo!(),
            ExitState::Failed => todo!(),
        }
    }
}
```

Call `init_platform` with your `Platform`, and `test_main` from your kernel entry point:

```rust
use os_test_framework::platform::init_platform;

fn kernel_entry() {
    init_platform(MyPlatform);
    test_main();
}
```

Forward panics from your OS to `os-test-framework`:

```rust
os_test_framework::forward_panic!();
```

## Adding A Test

You can declare tests like this:

```rust
use os_test_framework::test;

test! {
    "Hello" {
        assert!(true);
    }
}
```

You can add multiple tests in the same file:

```rust
use os_test_framework::test;

test! {
    "Hello" {
        assert!(true);
    }
}

test! {
    "Hello2" {
        assert!(true);
    }
}
```
