#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osbuild::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    osbuild::_testpanic(info)
}

#[test_case]
fn trivial_test() {
    use osbuild::println;
    println!("Some random  icon");
}
