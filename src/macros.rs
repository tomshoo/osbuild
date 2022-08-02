use core::fmt::Write;

use x86_64::instructions::interrupts;

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        $crate::macros::_print(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($args:tt)*) => {
        $crate::print!("{}\n", format_args!($($args)*))
    }
}

#[macro_export]
macro_rules! serialout {
    ($($args:tt)*) => {
        $crate::macros::_serial_print(format_args!($($args)*))
    };
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use crate::rw::SCREEN;
    interrupts::without_interrupts(|| {
        SCREEN.lock().write_fmt(args).unwrap();
    })
}

#[doc(hidden)]
pub fn _serial_print(args: core::fmt::Arguments) {
    use crate::rw::serial::SERIAL1;
    SERIAL1.lock().write_fmt(args).unwrap()
}
