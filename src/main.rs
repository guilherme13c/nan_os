#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nan_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use nan_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    nan_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    nan_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    nan_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nan_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
