#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bareMetal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bareMetal_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bareMetal_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
