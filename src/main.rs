#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

/// Using "extern C" tells the compiler to use the C calling convention.
///
/// This function is the entry point, as the linker looks for "_start" by default.
///
/// We need to return the never type as opposed to void, because void still returns,
/// but we have no caller, so even though void returns nothing, it would still attempt
/// to hand control back to the previous caller, even though there isn't one as this
/// is the entry point to the OS.
#[no_mangle] // Keep the name of this function as "_start" in the object code
pub extern "C" fn _start() -> !
{
    vga_buffer::print_hello_world();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}