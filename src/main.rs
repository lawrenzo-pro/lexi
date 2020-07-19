#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(lexi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lexi::println;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    lexi::init(); // new
    println!("Welcome {}", "!");
    #[cfg(test)]
    test_main();
    println!("It did not crash{}","!");
    lexi::hlt_loop();  
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    lexi::hlt_loop();  
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lexi::test_panic_handler(info)
}
