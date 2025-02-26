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

static mut FILE: [u8; 1920] = [0; 1920];
static mut LENGTH: usize = 0;

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

pub fn system_call(function: i32, input: &[u8]) {
    if function == 0 {
        halt();
    }
    if function == 1 {
        reboot();
    }
    if function == 2 {
        file(input);
    }
    if function == 3 {
        remove();
    }
    if function == 4 {
        list();
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

fn reboot() {
    unsafe {
        asm!("int 0x19");
    }
}

fn file(text: &[u8]) {
    unsafe {
        if !text.is_empty() {
            for &byte in text.iter() {
                if LENGTH < FILE.len() {
                    FILE[LENGTH] = byte;
                    LENGTH += 1;
                }
                else {
                    println!("ERROR: Memory attempted to exceed 1920 bytes");
                    break;
                }
            }
            if LENGTH < FILE.len() {
                FILE[LENGTH] = b' ';
                LENGTH += 1;
            }
        }
    }
}

fn remove() {
    unsafe {
        FILE = [0; 1920];
        LENGTH = 0;
    }
}

fn list() {
    unsafe {
        for i in 0..LENGTH {
            print!("{}", FILE[i] as char);
        }
    }
    println!();
}