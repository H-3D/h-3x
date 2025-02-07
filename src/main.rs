#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod keyboard_buffer;
mod shell;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted the h-3x kernel successfully");
    shell::shell();
    loop {}
}