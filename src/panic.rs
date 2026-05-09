use core::panic::PanicInfo;

use owo_colors::OwoColorize;

use crate::{
    platform::{ExitState, platform},
    println,
};

pub fn handle_panic(panic_info: &PanicInfo) -> ! {
    println!(" {}", "Failed".bright_red().bold());
    println!();
    println!("{}", panic_info.message().red());

    platform().lock().exit(ExitState::Failed)
}
