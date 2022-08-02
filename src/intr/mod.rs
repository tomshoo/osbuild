pub mod gdt;
pub mod hw;
pub mod page;

use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(doublefault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_IDX);

            idt.page_fault.set_handler_fn(page::pagefault_handler);

            idt[hw::IntrIndex::Timer.into()].set_handler_fn(hw::timer_intr_handler);
            idt[hw::IntrIndex::Keyboard.into()].set_handler_fn(hw::keyboar_intr_handler);
        }
        idt
    };
}

pub fn idt_init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: bREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn doublefault_handler(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: dOUBLEFAULT\n{:#?}", stack_frame);
}

#[test_case]
fn breakpoint_test() {
    x86_64::instructions::interrupts::int3();
}
