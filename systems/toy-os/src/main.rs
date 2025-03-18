#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use toy_os::{allocator, memory, println, task::Task, task::executor::Executor, task::keyboard};
use x86_64::VirtAddr;
//use x86_64::structures::paging::Page;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("かっこいでしょう！");

    // initialize the InterruptDescriptionTable
    toy_os::init();

    // setup the heap (dynamically allocated memory)
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
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
    //// map an unused page
    //let page = Page::containing_address(VirtAddr::new(0));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    //// write the string `New!` to the screen through the new mapping
    //let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    //let addresses = [
    //    // the identity-mapped vga buffer page
    //    0xb8000,
    //    // some code page
    //    0x201008,
    //    // some stack page
    //    0x0100_0020_1a10,
    //    // virtual address mapped to physical address 0
    //    boot_info.physical_memory_offset,
    //];

    //for &address in &addresses {
    //    let virt = VirtAddr::new(address);
    //    let phys = mapper.translate_addr(virt);
    //    println!("{:?} -> {:?}", virt, phys);
    //}

    // trigger a page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u8) = 42;
    //};

    #[cfg(test)]
    test_main();

    // setup multi-tasking
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toy_os::test_panic_handler(info)
}
