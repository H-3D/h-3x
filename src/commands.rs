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
static mut BUFFER: &[u8] = b"";
static mut INPUT_COLOR: Color = Color::White;

pub fn architecture() {
    println!("x86_64");
}

pub fn bootloader() {
    println!("bootloader v0.9 crate");
}

pub fn calculator() {
    loop {
        let mut buffer = [0u8; 32];
        let mut pos = 0;

        loop {
            let c = keyboard_buffer::read_char();
            if c == '\0' { continue; }
            
            if c == '\\' {
                println!();
                return;
            }

            if c == '\n' {
                println!();
                break;
            }

            if (c.is_ascii_digit() || ['+', '-', '*', '/', '.', ' '].contains(&c)) && pos < buffer.len() {
                print!("{}", c);
                buffer[pos] = c as u8;
                pos += 1;
            }
        }

        let result = evaluate_expression(&buffer[..pos]);
        match result {
            Some(value) => println!("{:.15}", value),  // Changed from .6 to .15 for maximum reliable precision
            None => println!("Invalid expression"),
        }
    }
}

fn evaluate_expression(expr: &[u8]) -> Option<f64> {
    let mut num1 = 0.0f64;
    let mut num2 = 0.0f64;
    let mut op = 0u8;
    let mut parsing_second = false;
    let mut decimal_factor = 0.1f64;
    let mut is_decimal = false;

    for &byte in expr {
        match byte {
            b'0'..=b'9' if !parsing_second => {
                if is_decimal {
                    num1 += (byte - b'0') as f64 * decimal_factor;
                    decimal_factor *= 0.1;
                } else {
                    num1 = num1 * 10.0 + (byte - b'0') as f64;
                }
            }
            b'0'..=b'9' if parsing_second => {
                if is_decimal {
                    num2 += (byte - b'0') as f64 * decimal_factor;
                    decimal_factor *= 0.1;
                } else {
                    num2 = num2 * 10.0 + (byte - b'0') as f64;
                }
            }
            b'.' if !parsing_second && !is_decimal => {
                is_decimal = true;
                decimal_factor = 0.1;
            }
            b'.' if parsing_second && !is_decimal => {
                is_decimal = true;
                decimal_factor = 0.1;
            }
            b'+' | b'-' | b'*' | b'/' if !parsing_second => {
                op = byte;
                parsing_second = true;
                is_decimal = false;
                decimal_factor = 0.1;
            }
            b' ' => continue,
            _ => return None,
        }
    }

    match op {
        b'+' => Some(num1 + num2),
        b'-' => Some(num1 - num2),
        b'*' => Some(num1 * num2),
        b'/' => if num2 != 0.0 { Some(num1 / num2) } else { None },
        _ => None,
    }
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

pub fn cpu() {
    let mut brand_string = [0u8; 48];
    let mut regs: [u32; 4] = [0; 4];

    unsafe {
        asm!(
            "cpuid",
            in("eax") 0x80000000u32,
            lateout("eax") regs[0],
            lateout("edi") regs[1],
            lateout("ecx") regs[2],
            lateout("edx") regs[3],
        );
    }

    if regs[0] >= 0x80000004 {
        for i in 0..3 {
            unsafe {
                asm!(
                    "cpuid",
                    in("eax") 0x80000002u32 + i as u32,
                    lateout("eax") regs[0],
                    lateout("edi") regs[1],
                    lateout("ecx") regs[2],
                    lateout("edx") regs[3],
                );
            }

            let offset = i as usize * 16;
            for (j, reg) in regs.iter().enumerate() {
                brand_string[offset + j * 4..(offset + j * 4 + 4)]
                    .copy_from_slice(&reg.to_le_bytes());
            }
        }

        if let Ok(s) = core::str::from_utf8(&brand_string) {
            println!("{}", s.trim_end());
        }
    }
}

pub fn echo(input: &[u8]) {
    let input_str = core::str::from_utf8(input).unwrap_or("<invalid UTF-8>");
    println!("{}", input_str.trim());
}

pub fn flix() {
    clear();
    unsafe {
        print!("{}", core::str::from_utf8_unchecked(BUFFER));
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
        BUFFER = &VGA;
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
    println!("Commands:\narchitecture\nbootloader\ncalculator\nclear\ncolor [color]\ncpu\necho [message]\nflix\nflox\nhalt\nhelp\ninfo\nls\nmanual\npurge\nreboot\nsleep\ntime\ntouch [text]\nuptime\nvendor\nversion");
}

pub fn info() {
    print!("Architecture: ");
    architecture();
    print!("Bootloader: ");
    bootloader();
    print!("CPU: ");
    cpu();
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
calculator: Interactive calculator mode.
clear: Clears the screen.
color [color]: Changes the text color.
cpu: Displays the CPU brand string.
echo [message]: Echoes a message.
flix: Buffer Text Editor.
flox: Ephemeral Text Editor.
halt: Halts the CPU.
help: Lists all available commands.
info: Displays system information.
ls: Displays the contents of the variable.
manual: Displays the system manual.
purge: Deletes all the text in the variable.
reboot: Reboots the system.
sleep: Sleeps for a set duration (for testing purposes).
time: Displays the Real Time Clock.
touch [text]: Appends your text to a variable.
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