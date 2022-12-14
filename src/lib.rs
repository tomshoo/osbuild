#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod intr;
pub mod macros;
pub mod rw;

use core::panic::PanicInfo;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum QemuExitStatus {
    Success = 0x10,
    Failure = 0x11,
}

pub trait Tests {
    fn call(&self);
}

impl<T: Fn()> Tests for T {
    fn call(&self) {
        serialout!("{}...\t", core::any::type_name::<T>());
        self();
        serialout!("[ok]\n");
    }
}

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_runner(tests: &[&dyn Tests]) {
    serialout!("Found {} test(s)...\n", tests.len());
    for test in tests {
        test.call();
    }

    exti_qemu(QemuExitStatus::Success);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt();
}

pub fn _testpanic(info: &PanicInfo) -> ! {
    serialout!("Error: {}... [fail]\n", info);
    exti_qemu(QemuExitStatus::Failure);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    _testpanic(info)
}

pub fn exti_qemu(code: QemuExitStatus) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32)
    }
}

// Interrupt Descriptor Table
pub fn init() {
    intr::gdt::gdt_init();
    intr::idt_init();
    unsafe {
        intr::hw::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}
