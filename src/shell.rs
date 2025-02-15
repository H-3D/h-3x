#[warn(static_mut_refs)]

use core::arch::asm;
use crate::{print, println, SYSTEM_CALL, FILE};
use crate::keyboard_buffer;

const BUFFER_SIZE: usize = 79;
const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

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
            if input_str == "bootloader\n" {
                bootloader();
            }
            if input_str == "clear\n" {
                clear();
            }
            if input_str.starts_with("echo ") {
                let input = &input_str[5..];
                echo(input.as_bytes());
            }
            if input_str == "flix\n" {
                flix();
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
            if input_str == "manual\n" {
                manual();
            }
            if input_str == "reboot\n" {
                reboot();
            }
            if input_str == "sleep\n" {
                sleep();
            }
            if input_str == "time\n" {
                unsafe {
                    SYSTEM_CALL = 0;
                }
                return;
            }
            if input_str == "uptime\n" {
                unsafe {
                    SYSTEM_CALL = 1;
                }
                return;
            }
            if input_str == "vendor\n" {
                vendor();
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

pub fn bootloader() {
    println!("rust bootimage-generated bootloader");
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

pub fn flix() {
    clear();
    unsafe {
        print!("{}", core::str::from_utf8_unchecked(FILE));
    }
    loop {
        let character = keyboard_buffer::read_char();
        if character == '\\' {
            screen();
            break;
        }
        if character == '/' {
            clear();
            println!();
        }
        if character != '\0' && character != '/' {
            print!("{}", character);
        }
    }
    clear();
    println!();
}

pub fn screen() {
    static mut VGA: [u8; VGA_WIDTH * VGA_HEIGHT] = [0; VGA_WIDTH * VGA_HEIGHT];
    unsafe {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let index = row * VGA_WIDTH + col;
                let char_cell = *VGA_BUFFER.offset(index as isize);
                VGA[index] = (char_cell & 0xFF) as u8;
            }
        }
        FILE = &VGA;
    }
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
    println!("Commands:\narchitecture\nbootloader\nclear\necho [message]\nflix\nhalt\nhelp\ninfo\nmanual\nreboot\nsleep\ntime\nuptime\nvendor\nversion");
}

pub fn info() {
    print!("Architecture: ");
    architecture();
    print!("Bootloader: ");
    bootloader();
    print!("Vendor: ");
    vendor();
    print!("Version: ");
    version();
}

pub fn manual() {
    println!("Commands:
architecture: Displays the system architecture (x86_64).
bootloader: Information about the bootloader (rust bootimage-generated).
clear: Clears the screen.
echo [message]: Echoes a message.
flix: Buffer Text Editor
halt: Halts the CPU.
help: Lists all available commands.
info: Displays system information (architecture, bootloader, vendor, version).
manual: Displays the system manual.
reboot: Reboots the system.
sleep: Sleeps for a set duration (for testing purposes).
time: Sends a system call to the kernel to display the Real Time Clock.
uptime: Sends a system call to the kernel to display the system uptime.
vendor: Displays CPU vendor string.
version: Displays the kernel version.");
}

pub fn reboot() {
    unsafe {
        asm!("int 0x19");
    }
}

pub fn sleep() {
    for _ in 0..10_000_000 {}
}

pub fn vendor() {
    let mut regs: [u32; 4] = [0; 4];

    unsafe {
        asm!(
            "cpuid",
            in("eax") 0,
            lateout("eax") regs[0],
            lateout("edi") regs[1],
            lateout("ecx") regs[2],
            lateout("edx") regs[3],
        );
    }

    let vendor = [
        (regs[1] & 0xFF) as u8,
        ((regs[1] >> 8) & 0xFF) as u8,
        ((regs[1] >> 16) & 0xFF) as u8,
        ((regs[1] >> 24) & 0xFF) as u8,
        (regs[3] & 0xFF) as u8,
        ((regs[3] >> 8) & 0xFF) as u8,
        ((regs[3] >> 16) & 0xFF) as u8,
        ((regs[3] >> 24) & 0xFF) as u8,
        (regs[2] & 0xFF) as u8,
        ((regs[2] >> 8) & 0xFF) as u8,
        ((regs[2] >> 16) & 0xFF) as u8,
        ((regs[2] >> 24) & 0xFF) as u8,
    ];

    for &byte in &vendor {
        print!("{}", byte as char);
    }
    println!();
}

pub fn version() {
    println!("h-3x Kernel v1.0.0-beta");
}