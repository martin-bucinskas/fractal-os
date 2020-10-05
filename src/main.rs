#![no_std] // Disable stdlib linking
#![no_main] // Disable all rust-level entry points

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
    println!("My name is {}", "Martin");
    print!("My favourite colour is {} and favourite word is {}", 42, 3.1417);

    loop {}
}
