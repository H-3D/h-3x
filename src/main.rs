#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod vga_buffer;
mod keyboard_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted the h-3x kernel successfully");
    shell();
    loop {}
}

const BUFFER_SIZE: usize = 79;

struct Buffer {
    buffer: [u8; BUFFER_SIZE],
    index: usize,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            buffer: [0; BUFFER_SIZE],
            index: 0,
        }
    }

    fn add_char(&mut self, c: u8) {
        if self.index < BUFFER_SIZE - 1 {
            self.buffer[self.index] = c;
            self.index += 1;
        }
    }

    fn reset(&mut self) {
        self.index = 0;
        self.buffer = [0; BUFFER_SIZE];
    }

    fn get_input(&self) -> &[u8] {
        &self.buffer[..self.index]
    }
}

pub fn shell() {
    println!("Welcome to the h-3x shell");
    println!("Enter 'help' to list all the commands");
    print!("> ");
    let mut buffer = Buffer::new();
    loop {
        let character = keyboard_buffer::read_char();
        if character != '\0' {
            print!("{}", character);
            buffer.add_char(character as u8);
            if buffer.index == BUFFER_SIZE - 1{
                if character != '\n' {
                    buffer.reset();
                    print!("> ");
                }
            }
        }
        if character == '\n' {
            let input = buffer.get_input();
            let input_str = core::str::from_utf8(input).unwrap_or("<invalid UTF-8>");
            if input_str == "architecture\n" {
                architecture();
            }
            if input_str == "clear\n" {
                clear();
            }
            if input_str.starts_with("echo ") {
                let input = &input_str[5..];
                echo(input.as_bytes());
            }
            if input_str == "halt\n" {
                halt();
            }
            if input_str == "help\n" {
                help();
            }
            if input_str == "info\n" {
                info();
            }
            if input_str == "reboot\n" {
                reboot();
            }
            if input_str == "sleep\n" {
                sleep();
            }
            if input_str == "version\n" {
                version();
            }
            buffer.reset();
            print!("> ");
        }
    }
}

pub fn architecture() {
    println!("x86_64");
}

pub fn clear() {
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
}

pub fn echo(input: &[u8]) {
    let input_str = core::str::from_utf8(input).unwrap_or("<invalid UTF-8>");
    print!("{}", input_str);
}

pub fn halt() {
    clear();
    print!("CPU Halted");
    unsafe {
        asm!("hlt");
    }
    loop {}
}

pub fn help() {
    println!("Commands:\narchitecture\nclear\necho [arg ...]\nhalt\nhelp\ninfo\nreboot\nsleep\nversion");
}

pub fn info() {
    print!("Architecture: ");
    architecture();
    print!("Version: ");
    version();
}

pub fn reboot() {
    unsafe {
        asm!("int 0x19");
    }
}

pub fn sleep() {
    for _ in 0..10_000_000 {}
}

pub fn version() {
    println!("h-3x Kernel v1.0.0-alpha");
}