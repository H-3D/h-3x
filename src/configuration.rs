use crate::vga_buffer::Color;
use crate::{println, system_call};
use crate::commands;

pub fn process(input_str: &str) {
    match input_str.trim() {
        "architecture" => commands::architecture(),
        "bootloader" => commands::bootloader(),
        "clear" => commands::clear(),
        "flix" => commands::flix(),
        "flox" => commands::flox(),
        "halt" => system_call(0, b""),
        "help" => commands::help(),
        "info" => commands::info(),
        "ls" => system_call(4, b""),
        "manual" => commands::manual(),
        "purge" => system_call(3, b""),
        "reboot" => system_call(1, b""),
        "sleep" => commands::sleep(),
        "time" => commands::time(),
        "uptime" => commands::uptime(),
        "vendor" => commands::vendor(),
        "version" => commands::version(),
        "" => (),
        _ if input_str.starts_with("color ") => commands::color(&input_str[6..], Color::Black),
        _ if input_str.starts_with("echo ") => commands::echo(&input_str[5..].as_bytes()),
        _ if input_str.starts_with("touch ") => system_call(2, &input_str[6..].trim().as_bytes()),
        _ => println!("ERROR: Invalid Command"),
    }
}