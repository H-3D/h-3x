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
        "halt" => system_call(0),
        "help" => commands::help(),
        "info" => commands::info(),
        "manual" => commands::manual(),
        "reboot" => system_call(1),
        "sleep" => commands::sleep(),
        "time" => commands::time(),
        "uptime" => commands::uptime(),
        "vendor" => commands::vendor(),
        "version" => commands::version(),
        "" => (),
        _ if input_str.starts_with("color ") => commands::color(&input_str[6..], Color::Black),
        _ if input_str.starts_with("echo ") => commands::echo(&input_str[5..].as_bytes()),
        _ => println!("Invalid Command"),
    }
}