use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::{hlt, println};

pub extern "x86-interrupt" fn pagefault_handler(
    _stackframe: InterruptStackFrame,
    _code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: pAGEFAULT");
    println!("Tried to access: {:?}", Cr2::read());
    println!("Err: {:?}", _code);
    println!("{:#?}", _stackframe);
    hlt();
}
