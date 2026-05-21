use core::panic::PanicInfo;

use owo_colors::OwoColorize;

use crate::platform::{self, ExitState, try_platform};

pub fn handle_panic(panic_info: &PanicInfo) -> ! {
    if let Some(platform) = try_platform() {
        let mut platform = platform.lock();
        platform.print(format_args!(
            " {}\n\n{}\n",
            "FAILED".bright_red(),
            panic_info.message()
        ));

        platform.print(format_args!("test result: {}.", "FAILED".bright_red()));
        platform.exit(ExitState::Failed)
    }

    loop {
        core::hint::spin_loop();
    }
}

#[macro_export]
macro_rules! forward_panic {
    () => {
        #[cfg(test)]
        #[panic_handler]
        fn _os_test_framework_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
            $crate::handle_panic(panic_info);
        }
    };
}
