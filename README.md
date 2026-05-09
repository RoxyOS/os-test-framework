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

Implement `Platform`. The framework writes output through `core::fmt::Write` and finishes the run through `Platform::exit`:

```rust
use core::fmt::Write;

use os_test_framework::platform::{ExitState, Platform};

struct MyPlatform;

impl Platform for MyPlatform {
    fn exit(&self, state: ExitState) -> ! {
        match state {
            ExitState::Success => todo!(),
            ExitState::Failed => todo!(),
        }
    }
}

impl Write for MyPlatform {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Write to serial, frame buffer, UART, etc.
        let _ = s;
        todo!()
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
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_test_framework::panic::handle_panic(info)
}
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
