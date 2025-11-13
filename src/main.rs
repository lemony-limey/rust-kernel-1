// The source code from this file originates from various sections in
// Philipp Oppermann's "Writing an OS in Rust" blog, which can be
// found at: https://os.phil-opp.com/
#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(stacsos_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{
    boxed::Box,
    rc::Rc,
    vec,
    vec::Vec,
};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use stacsos_rs::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !
{
    use stacsos_rs::{
        allocator,
        memory::{self, BootInfoFrameAllocator},
    };
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    stacsos_rs::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialisation failed");

    // Allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // Create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500
    {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference counted vector. It will be freed when the reference
    // count reaches 0.
    let reference_counted_vec = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted_vec.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted_vec);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // This is ok because of "reexport_test_harness_main"
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
