#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osbuild::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osbuild::println;
use x86_64::registers::control::Cr3;

#[allow(unconditional_recursion)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting...");
    osbuild::init();

    let (l4, some) = Cr3::read();
    println!("{:?}, {:?}", l4.start_address(), some);

    #[cfg(test)]
    test_main();

    println!("Done...");
    osbuild::hlt();
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
