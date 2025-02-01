#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod keyboard_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("booted the h-3x kernel successfully");
    loop {
        let character = keyboard_buffer::read_char();
        if character != '\0' {
            print!("{}", character);
        }
    }
}