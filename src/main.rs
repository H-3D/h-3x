#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use crate::configuration::process;

mod vga_buffer;
mod keyboard_buffer;
mod shell;
mod commands;
mod configuration;

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

pub fn execute(input: &str) {
    process(input);
}

pub fn system_call(function: i32) {
    if function == 0 {
        halt();
    }
    if function == 1 {
        reboot();
    }
}

fn halt() {
    unsafe {
        asm!(
            "mov rdi, 0xB8000",
            "mov rax, 0x20",
            "mov rbx, 0x0F",
            "mov rcx, 2000",
            "2:",
            "mov [rdi], ax",
            "add rdi, 2",
            "loop 2b",
            options(nostack)
        );
    }
    print!("CPU Halted");
    unsafe {
        asm!("hlt");
    }
    loop {}
}

pub fn reboot() {
    unsafe {
        asm!("int 0x19");
    }
}