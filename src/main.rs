#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod keyboard_buffer;
mod shell;
mod commands;
mod configuration;

pub static mut ERROR: bool = false;
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

pub fn system_call(function: i32, input: &[u8]) {
    if function == 0 {
        ls();
    }
    if function == 1 {
        purge();
    }
    if function == 2 {
        rm(input);
    }
    if function == 3 {
        touch(input);
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

fn purge() {
    unsafe {
        FILE = [0; 1920];
        LENGTH = 0;
    }
}

fn rm(text: &[u8]) {
    unsafe {
        if let Ok(current) = core::str::from_utf8(&FILE[..LENGTH]) {
            let mut new_content: [u8; 1920] = [0; 1920];
            let mut new_index = 0;
            let mut found = false;
            let name = core::str::from_utf8(text).unwrap_or("");
            for token in current.split_whitespace() {
                if !found && token == name {
                    found = true;
                    continue;
                }
                if token.len() > 0 {
                    if new_index != 0 {
                        if new_index < new_content.len() {
                            new_content[new_index] = b' ';
                            new_index += 1;
                        }
                    }
                    for &b in token.as_bytes() {
                        if new_index < new_content.len() {
                            new_content[new_index] = b;
                            new_index += 1;
                        }
                    }
                }
            }
            if new_index > 0 && new_content[new_index - 1] != b' ' {
                if new_index < new_content.len() {
                    new_content[new_index] = b' ';
                    new_index += 1;
                }
            }
            if !found {
                ERROR = true;
                println!("ERROR: Text not found");
            } else {
                ERROR = false;
                for i in 0..new_index {
                    FILE[i] = new_content[i];
                }
                LENGTH = new_index;
            }
        }
    }
}

fn touch(text: &[u8]) {
    unsafe {
        if !text.is_empty() {
            for &byte in text.iter() {
                if LENGTH < FILE.len() {
                    ERROR = false;
                    FILE[LENGTH] = byte;
                    LENGTH += 1;
                }
                else {
                    ERROR = true;
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