#![no_std] //We need a freestanding binary and by default Rust crates link the std libarary thus we use no_std
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bareMetal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bareMetal_os::allocator;
use bareMetal_os::memory::BootInfoFrameAllocator;
use bareMetal_os::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

pub trait Testable {
    fn run(&self) -> ();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    // loop {}
    bareMetal_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    bareMetal_os::test_panic_handler(_info);
}


entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello ,Guys{}", "!");
   
    bareMetal_os::init();

    use bareMetal_os::memory;
    use x86_64::{
        structures::paging::Page,
        VirtAddr,
    };
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    let mut ptr = vec.as_slice().as_ptr();
    println!("vec is at {ptr:p}");

    for i in 0..500 {
        vec.push(i);
        let new_ptr = vec.as_ptr();
        if ptr == new_ptr {
            ptr = new_ptr;
            println!(" vec moved to {ptr:p}");
        }
    }

    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);

    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    bareMetal_os::hlt_loop();
}

