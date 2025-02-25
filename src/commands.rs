#[warn(static_mut_refs)]

use core::arch::asm;
use core::fmt;
use crate::{print, println};
use crate::vga_buffer::{WRITER, Color};
use crate::keyboard_buffer;

const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const RTC_PORT_INDEX: u16 = 0x70;
const RTC_PORT_DATA: u16 = 0x71;
static mut FILE: &[u8] = b"";
static mut INPUT_COLOR: Color = Color::White;

pub fn architecture() {
    println!("x86_64");
}

pub fn bootloader() {
    println!("bootloader v0.9 crate");
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

pub fn color(foreground: &str, background: Color) {
    unsafe {
        INPUT_COLOR = match foreground {
            "black\n" => Color::Black,
            "blue\n" => Color::Blue,
            "green\n" => Color::Green,
            "cyan\n" => Color::Cyan,
            "red\n" => Color::Red,
            "magenta\n" => Color::Magenta,
            "brown\n" => Color::Brown,
            "lightgray\n" => Color::LightGray,
            "darkgray\n" => Color::DarkGray,
            "lightblue\n" => Color::LightBlue,
            "lightgreen\n" => Color::LightGreen,
            "lightcyan\n" => Color::LightCyan,
            "lightred\n" => Color::LightRed,
            "pink\n" => Color::Pink,
            "yellow\n" => Color::Yellow,
            "white\n" => Color::White,
            _ => INPUT_COLOR,
        };
        let mut writer = WRITER.lock();
        writer.color(INPUT_COLOR, background);
    }
}

pub fn echo(input: &[u8]) {
    let input_str = core::str::from_utf8(input).unwrap_or("<invalid UTF-8>");
    println!("{}", input_str.trim());
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

pub fn flox() {
    clear();
    loop {
        let character = keyboard_buffer::read_char();
        if character == '\\' {
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

pub fn help() {
    println!("Commands:\narchitecture\nbootloader\nclear\ncolor [color]\necho [message]\nflix\nflox\nhalt\nhelp\ninfo\nmanual\nreboot\nsleep\ntime\nuptime\nvendor\nversion");
}

pub fn info() {
    print!("Architecture: ");
    architecture();
    print!("Bootloader: ");
    bootloader();
    print!("Time: ");
    time();
    print!("Uptime: ");
    uptime();
    print!("Vendor: ");
    vendor();
    print!("Version: ");
    version();
}

pub fn manual() {
    println!("Commands:
architecture: Displays the system architecture (x86_64).
bootloader: Information about the bootloader (bootloader v0.9 crate).
clear: Clears the screen.
color [color]: Changes the text color.
echo [message]: Echoes a message.
flix: Buffer Text Editor.
flox: Ephemeral Text Editor.
halt: Halts the CPU.
help: Lists all available commands.
info: Displays system information.
manual: Displays the system manual.
reboot: Reboots the system.
sleep: Sleeps for a set duration (for testing purposes).
time: Displays the Real Time Clock.
uptime: Displays the system uptime.
vendor: Displays CPU vendor string.
version: Displays the kernel version.");
}

pub fn sleep() {
    for _ in 0..10_000_000 {}
}

struct Time {
    seconds: u8,
    minutes: u8,
    hours: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

fn bcd_to_decimal(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

pub fn time() {
    unsafe {
        outb(RTC_PORT_INDEX, 0x00);
        let bcd_seconds = inb(RTC_PORT_DATA);

        outb(RTC_PORT_INDEX, 0x02);
        let bcd_minutes = inb(RTC_PORT_DATA);

        outb(RTC_PORT_INDEX, 0x04);
        let bcd_hours = inb(RTC_PORT_DATA);

        let time = Time {
            seconds: bcd_to_decimal(bcd_seconds),
            minutes: bcd_to_decimal(bcd_minutes),
            hours: bcd_to_decimal(bcd_hours),
        };
        
        println!("UTC: {}", time);
    }
}

unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value);
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", in("dx") port, out("al") value);
    value
}

pub fn uptime() {
    let mut tsc: u64;
    unsafe {
        asm!(
            "rdtsc",
            out("eax") _,
            out("edx") tsc,
            options(nostack, preserves_flags)
        );
    }
    println!("{} cycles", tsc);
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