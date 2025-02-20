#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

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
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate()
    {
        // Below we are writing each character to the VGA buffer, starting from 0xb8000 and writing
        // two bytes at a time, the first one for the character and the second for its colour.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize *2 + 1) = 0xb;  // Colour byte (light cyan)
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}