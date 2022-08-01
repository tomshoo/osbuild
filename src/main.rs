#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osbuild::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osbuild::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    osbuild::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Hello world");
    loop {}
}

#[cfg(not(test))]
#[doc(hidden)]
#[panic_handler]
pub fn _panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[doc(hidden)]
#[panic_handler]
pub fn _panic(info: &PanicInfo) -> ! {
    osbuild::_testpanic(info)
}
