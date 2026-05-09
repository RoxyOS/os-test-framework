use core::fmt::Arguments;

use crate::platform::platform;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::printing::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    platform().lock().write_fmt(args).unwrap();
}
