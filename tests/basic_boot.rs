#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fractal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fractal_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

// fn test_runner(tests: &[&dyn Fn()]) {
//     unimplemented!();
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fractal_os::test_panic_handler(info);
}

#[test_case]
fn test_println_single_line() {
    println!("test_println_single_line");
}

#[test_case]
fn test_println_multiple_lines() {
    for _ in 0..350 {
        println!("test_println_multiple_lines");
    }
}

// #[test_case]
// fn test_println_output() {
//     let string = "test_println_output";
//     println!("{}", string);
//
//     for (i, c) in string.chars().enumerate() {
//         let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
//         assert_eq!(char::from(screen_char.ascii_character), c);
//     }
// }

