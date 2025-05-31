// The source code from this file originates from the "Testing" section
// of Philipp Oppermann's "Writing an OS in Rust" blog, which can be
// found at: https://os.phil-opp.com/testing/
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !
{
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    kernel::test_panic_handler(info)
}

#[test_case]
fn test_println()
{
    println!("test_println output");
}
