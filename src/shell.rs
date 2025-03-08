use crate::{print, println};
use crate::configuration::execute;
use crate::keyboard_buffer;

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
    println!("Enter 'manual' to display the system manual");
    print!("> ");
    let mut buffer = Buffer::new();
    loop {
        let character = keyboard_buffer::read_char();
        if character != '\0' {
            print!("{}", character);
            buffer.add_char(character as u8);
            if buffer.index == BUFFER_SIZE - 1 && character != '\n' {
                let input = buffer.get_input();
                let input_str = core::str::from_utf8(input).unwrap_or("<invalid UTF-8>");
                loop {
                    let character = keyboard_buffer::read_char();
                    if character == '\n' {
                        break;
                    }
                }
                execute(input_str);
                buffer.reset();
                print!("> ");
            }
        }
        if character == '\n' {
            let input = buffer.get_input();
            let input_str = core::str::from_utf8(input).unwrap_or("<invalid UTF-8>");
            execute(input_str);
            buffer.reset();
            print!("> ");
        }
    }
}