# os-test-framework

Test framework for embedded systems and OS kernels.

## Getting Started

First, enable `custom_test_frameworks`, set `test_runner` as the test runner from `os-test-framework`, and `reexport_test_harness_main`.

```rust
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(os_test_framework::run_tests)]
```

Implement `Platform`. The framework writes output through `core::fmt::Write` and finishes the run through `Platform::exit`:

```rust
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
fn kernel_entry() {
    init_platform(MyPlatform);
    test_main();
}
```

## Adding A Test

You can declare tests like this:

```rust
test! {
    "Hello" {
        assert!(true);
    }
}
```

You can add multiple tests in the same file:

```rust
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
