pub mod vga;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref SCREEN: Mutex<vga::Screen<'static>> = Mutex::new(vga::Screen::new(0x0a));
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        $crate::rw::_print(format_args!($($args)*))
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

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SCREEN.lock().write_fmt(args).unwrap();
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Colors {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LGray = 0x7,
    DGray = 0x8,
    LBlue = 0x9,
    LGreen = 0xa,
    LCyan = 0xb,
    LRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[allow(dead_code)]
impl Colors {
    pub fn theme(fg: Colors, bg: Colors) -> u8 {
        (bg as u8) << 4 | fg as u8
    }
}
