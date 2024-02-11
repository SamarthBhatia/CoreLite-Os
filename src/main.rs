#![no_std] //We need a freestanding binary and by default Rust crates link the std libarary thus we use no_std
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]

fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HEY: &[u8] = b"Hello Everyone I am BareMetal-OS Good to see Everyone";

#[no_mangle]

pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for(buff, &byte) in HEY.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(buff as isize * 2) = byte;
            *vga_buffer.offset(buff as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

 
