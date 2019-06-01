#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(micox::test_runner)]
#![reexport_test_harness_main = "test_main"]

use micox::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World");
  #[cfg(test)]
  test_main();
  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
  micox::test_panic_handler(info)
}
