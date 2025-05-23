// The source code from this section originates from various sections in
// Philipp Oppermann's "Writing an OS in Rust" blog, which can be
// found at: https://os.phil-opp.com/
#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::mem::offset_of;
use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

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
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
    
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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable])
{
    serial_println!("Running {} tests", tests.len());
    for test in tests
    {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub trait Testable
{
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self)
    {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[test_case]
fn trivial_assertion()
{
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode
{
    Success = 0x10,
    Failed = 0x11,
}

/// isa-debug-exit uses Port-mapped I/O
pub fn exit_qemu(exit_code: QemuExitCode)
{
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
