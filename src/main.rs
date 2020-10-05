/*
    main.rs

    Entry point of the kernel.
 */

// Disable stdlib linking
#![no_std]
// Disable all rust-level entry points
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// Calls on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[KERNEL_PANIC]: {}", info);
    loop {}
}

// Don't mangle the entry point function name
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello, world!");

    loop {}
}
