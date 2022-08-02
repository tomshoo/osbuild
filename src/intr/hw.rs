use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::{self, Mutex};
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::print;

lazy_static! {
    static ref KBD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        layouts::Us104Key,
        ScancodeSet1,
        HandleControl::MapLettersToUnicode
    ));
}

pub const PIC1_OFFSET: u8 = 0x20;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum IntrIndex {
    Timer = PIC1_OFFSET,
    Keyboard,
}

impl From<IntrIndex> for usize {
    fn from(intr: IntrIndex) -> Self {
        usize::from(intr as u8)
    }
}

pub extern "x86-interrupt" fn timer_intr_handler(_stackframe: InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(IntrIndex::Timer as u8);
    }
}

pub extern "x86-interrupt" fn keyboar_intr_handler(_stackframe: InterruptStackFrame) {
    let mut kbd = KBD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(event)) = kbd.add_byte(scancode) {
        if let Some(key) = kbd.process_keyevent(event) {
            match key {
                DecodedKey::Unicode(ch) => print!("{ch}"),
                DecodedKey::RawKey(key) => print!("{key:?}"),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(IntrIndex::Keyboard as u8);
    }
}
