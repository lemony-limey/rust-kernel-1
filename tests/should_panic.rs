#![no_std]
#![no_main]

use core::panic::PanicInfo;
use stacsos_rs::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !
{
    should_fail();
    // This code should not be reached if the test case above
    // panics, as is to be expected.
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail()
{
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

/// To simulate the 'should_panic' attribute from the standard library,
/// we can instead define a panic handler that returns success when called.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}