#![no_std] // Disable stdlib linking
#![no_main] // Disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(fractal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fractal_os::{print, println};

// Don't mangle the entry point function name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    fractal_os::init();

    println!("Hello, world!");
    println!("My name is {}", "Martin");
    print!(
        "My favourite colour is {} and favourite word is {}\n",
        42, 3.1417
    );

    // Causes a breakpoint exception
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

// Panic handler.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[KERNEL_PANIC]: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fractal_os::test_panic_handler(info);
}

#[test_case]
fn smoke_test() {
    assert_eq!(1, 1);
}
