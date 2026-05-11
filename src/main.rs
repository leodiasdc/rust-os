#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::Page, VirtAddr};
    use rust_os::memory::BootInfoFrameAllocator;
    use rust_os::memory;

    println!("Hello World{}", "!");
    rust_os::init();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     println!("Hello World{}", "!");
//     rust_os::init();

//     fn stack_overflow() {
//         stack_overflow(); // for each recursion, the return address is pushed
//     }

//     // trigger a stack overflow
//     // stack_overflow();

//     // trigger a page fault
//     // unsafe {
//     //     *(0xdeadbeef as *mut u8) = 42;
//     // }; 

//     // new page fault exception
//     // let ptr = 0xdeadbeaf as *mut u8;
//     // unsafe { *ptr = 42; }

//     // Note: The actual address might be different for you. Use the address that
//     // your page fault handler reports.
//     // let ptr = 0x204f96 as *mut u8;

//     // read from a code page
//     // unsafe { let x = *ptr; }
//     // println!("read worked");

//     // // write to a code page
//     // unsafe { *ptr = 42; }
//     // println!("write worked");
    
//     use x86_64::registers::control::Cr3;

//     let (level_4_page_table, _) = Cr3::read();
//     println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    
//     #[cfg(test)]
//     test_main();
    
//     // deadlock exception provoke
//     // loop {
//     //     use rust_os::print;
//     //     print!("-");        // new
//     // }
    
//     rust_os::hlt_loop(); 
// }
