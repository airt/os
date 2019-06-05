#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(micox::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use micox::allocator;
use micox::memory::{self, BootInfoFrameAllocator};
use micox::println;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  println!("Hello World");
  micox::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
  allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

  let heap_value = Box::new(41);
  println!("heap_value at {:p}", heap_value);

  let mut vec = Vec::new();
  for i in 0..500 {
    vec.push(i);
  }
  println!("vec at {:p}", vec.as_slice());

  let reference_counted = Rc::new(vec![1, 2, 3]);
  let cloned_reference = reference_counted.clone();
  println!(
    "current reference count is {}",
    Rc::strong_count(&cloned_reference)
  );
  core::mem::drop(reference_counted);
  println!(
    "reference count is {} now",
    Rc::strong_count(&cloned_reference)
  );

  #[cfg(test)]
  test_main();
  println!("It did not crash!");
  micox::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
  println!("{}", info);
  micox::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
  micox::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
  assert_eq!(1, 1);
}
