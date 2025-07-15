// The source code from this file originates from various sections in
// Philipp Oppermann's "Writing an OS in Rust" blog, which can be
// found at: https://os.phil-opp.com/
#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(stacsos_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use stacsos_rs::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !
{
    use stacsos_rs::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    stacsos_rs::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write the string 'New!' to the screen through the mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    stacsos_rs::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    println!("{}", info);
    stacsos_rs::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    stacsos_rs::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion()
{
    assert_eq!(1, 1);
}
