/*
    main.rs

    Entry point of the kernel.
 */

// Disable stdlib linking
#![no_std]
// Disable all rust-level entry points
#![no_main]

use core::panic::PanicInfo;

// Calls on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Don't mangle the entry point function name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
