#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt;

mod vga_buffer;
mod keyboard_buffer;
mod shell;

pub static mut SYSTEM_CALL: u64 = 0;
pub static mut FILE: &[u8] = b"";

const RTC_PORT_INDEX: u16 = 0x70;
const RTC_PORT_DATA: u16 = 0x71;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted the h-3x kernel successfully");
    println!("Welcome to the h-3x shell");
    println!("Enter 'help' to list all the commands");
    println!("Enter 'manual' to display the system manual");
    loop {
        shell::shell();
        unsafe {
            if SYSTEM_CALL == 0 {
                time();
            }
            if SYSTEM_CALL == 1 {
                uptime();
            }
        }
    }
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

fn time() {
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