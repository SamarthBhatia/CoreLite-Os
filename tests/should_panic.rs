#![no_std]
#![no_main]

use bareMetal_os::{exit_qemu, serial_println, QemuExitCode};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail(); //For single test we do not require test runner we can just use this as excutable
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

use bareMetal_os::serial_print;
// #[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
