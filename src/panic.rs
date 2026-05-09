use core::panic::PanicInfo;

use owo_colors::OwoColorize;

use crate::platform::{ExitState, try_platform};

pub fn handle_panic(panic_info: &PanicInfo) -> ! {
    if let Some(platform) = try_platform() {
        let mut platform = platform.lock();
        platform.print(format_args!(" {}\n\n{}\n", "Failed".bright_red().bold(), panic_info.message().red()));
        platform.exit(ExitState::Failed)
    }

    loop {
        core::hint::spin_loop();
    }
}
