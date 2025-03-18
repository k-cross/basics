#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(toy_os::test_runner)]

use core::panic::PanicInfo;

use toy_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    toy_os::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    toy_os::hlt_loop();
}

#[test_case]
fn test_println() {
    println!("something");
}
