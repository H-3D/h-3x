use crate::{print, println, system_call};
use crate::vga_buffer::Color;
use crate::keyboard_buffer;
use crate::commands;

const BUFFER_SIZE: usize = 79;
static mut INPUT_COLOR: Color = Color::White;

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
    println!("Enter 'manual' to display the system manual");
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
                commands::architecture();
            }
            if input_str == "bootloader\n" {
                commands::bootloader();
            }
            if input_str == "clear\n" {
                commands::clear();
            }
            if input_str.starts_with("color ") {
                unsafe {
                    INPUT_COLOR = match &input_str[6..] {
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
                    commands::color(INPUT_COLOR, Color::Black);
                }
            }
            if input_str.starts_with("echo ") {
                commands::echo(&input_str[5..].as_bytes());
            }
            if input_str == "flix\n" {
                commands::flix();
            }
            if input_str == "flox\n" {
                commands::flox();
            }
            if input_str == "halt\n" {
                system_call(0);
            }
            if input_str == "help\n" {
                commands::help();
            }
            if input_str == "info\n" {
                commands::info();
            }
            if input_str == "manual\n" {
                commands::manual();
            }
            if input_str == "reboot\n" {
                system_call(1);
            }
            if input_str == "sleep\n" {
                commands::sleep();
            }
            if input_str == "time\n" {
                commands::time();
            }
            if input_str == "uptime\n" {
                commands::uptime();
            }
            if input_str == "vendor\n" {
                commands::vendor();
            }
            if input_str == "version\n" {
                commands::version();
            }
            buffer.reset();
            print!("> ");
        }
    }
}