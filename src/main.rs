#![no_std]
#![no_main]
use core::{panic::PanicInfo, prelude::v1};
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Olá Rust, este é o desenvolvimeto de um sistema operacional! Seja muito bem-vindo!{}", "!");

    loop {}
}