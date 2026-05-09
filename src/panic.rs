use core::panic::PanicInfo;

use owo_colors::OwoColorize;
use x86_64::instructions::interrupts;

use crate::{
    platform::{ExitState, platform},
    println,
};

#[panic_handler]
fn handle_panic(panic_info: &PanicInfo) -> ! {
    interrupts::disable();

    println!(" {}", "Failed".bright_red().bold());
    println!();
    println!("{}", panic_info.message().red());

    platform().lock().exit(ExitState::Failed)
}
