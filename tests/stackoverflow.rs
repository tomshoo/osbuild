#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use osbuild::serialout;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// Stack overflow
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serialout!("Stackoverflow...\t");
    osbuild::intr::gdt::gdt_init();
    init_test_idt();
    stack_overflow();
    panic!("[ok]");
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    osbuild::_testpanic(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_doublefault_handler)
                .set_stack_index(osbuild::intr::gdt::DOUBLE_FAULT_IST_IDX);
        }
        idt
    };
}

fn init_test_idt() {
    IDT.load();
}

extern "x86-interrupt" fn test_doublefault_handler(
    _stackframe: InterruptStackFrame,
    _code: u64,
) -> ! {
    serialout!("[ok]\n");
    osbuild::exti_qemu(osbuild::QemuExitStatus::Success);
    loop {}
}
