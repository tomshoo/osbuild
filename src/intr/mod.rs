use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

#[allow(unused_mut, unused_variables)]
pub fn idt_init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: bREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn breakpoint_handler() {
    x86_64::instructions::interrupts::int3();
}
