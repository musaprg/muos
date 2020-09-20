
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Override default entrypoint which is known as "crt0"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
