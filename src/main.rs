#![no_std]
#![no_main]

mod rw;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..82 {
        println!("Printing... [{}]", i);
    }
    panic!("Casual panic at {}", 83);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
