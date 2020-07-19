#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(lexi::test_runner)]
#![reexport_test_harness_main = "test_main"]
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use lexi::println;
entry_point!(kernel_main);
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
     
    lexi::init(); 
    use lexi::memory;
    use lexi::memory::BootInfoFrameAllocator;;
    use x86_64::{structures::paging::Page, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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
