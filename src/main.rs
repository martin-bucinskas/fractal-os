#![no_std] // Disable stdlib linking
#![no_main] // Disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

// Don't mangle the entry point function name
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello, world!");
    println!("My name is {}", "Martin");
    print!("My favourite colour is {} and favourite word is {}\n", 42, 3.1417);

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

// -------- Testing Functions --------

// Test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failure]");
    serial_println!();
    serial_println!("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-");
    serial_println!("Tests Failed.");
    serial_println!();
    serial_println!("[KERNEL_PANIC]: {}", info);
    serial_println!("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!();
    serial_println!("================================================");
    serial_println!("Running {} tests", tests.len());
    serial_println!("================================================");
    serial_println!();

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn smoke_test() {
    assert_eq!(1, 1);
}
