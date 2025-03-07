#![no_std]
#![no_main]

use core::panic::PanicInfo;
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
        touch(input);
    }
    if function == 1 {
        purge();
    }
    if function == 2 {
        ls();
    }
}

fn touch(text: &[u8]) {
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

fn purge() {
    unsafe {
        FILE = [0; 1920];
        LENGTH = 0;
    }
}

fn ls() {
    unsafe {
        for i in 0..LENGTH {
            print!("{}", FILE[i] as char);
        }
    }
    println!();
}