// The source code from this section originates from various sections in
// Philipp Oppermann's "Writing an OS in Rust" blog, which can be
// found at: https://os.phil-opp.com/
#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(stacsos_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use stacsos_rs::println;

/// Using "extern C" tells the compiler to use the C calling convention.
///
/// This function is the entry point, as the linker looks for "_start" by default.
///
/// We need to return the never type as opposed to void, because void still returns,
/// but we have no caller, so even though void returns nothing, it would still attempt
/// to hand control back to the previous caller, even though there isn't one as this
/// is the entry point to the OS.
#[unsafe(no_mangle)] // Keep the name of this function as "_start" in the object code
pub extern "C" fn _start() -> !
{
    println!("Hello World{}", "!");
    
    // Initialise IDT, then cause a breakpoint exception.
    stacsos_rs::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    println!("{}", info);
    loop {}
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
