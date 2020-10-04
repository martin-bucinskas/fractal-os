/*
    main.rs

    Entry point of the kernel.
 */

// Disable stdlib linking
#![no_std]
// Disable all rust-level entry points
#![no_main]

use core::panic::PanicInfo;

static HELLO_WORLD: &[u8] = b"Hello, world!";

// Calls on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Don't mangle the entry point function name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO_WORLD.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
