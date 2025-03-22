use crate::vga_buffer::Color;
use crate::{println, system_call, ERROR};
use crate::commands;

pub fn execute(input_str: &str) {
    unsafe {
        ERROR = false;
    }
    match input_str.trim() {
        "architecture" => commands::architecture(),
        "bootloader" => commands::bootloader(),
        "buffer" => commands::buffer(),
        "calculator" => commands::calculator(),
        "clear" => commands::clear(),
        "cpu" => commands::cpu(),
        "ephemeral" => commands::ephemeral(),
        "halt" => commands::halt(),
        "help" => commands::help(),
        "info" => commands::info(),
        "ls" => system_call(0, b""),
        "manual" => commands::manual(),
        "purge" => system_call(1, b""),
        "reboot" => commands::reboot(),
        "time" => commands::time(),
        "uptime" => commands::uptime(),
        "vendor" => commands::vendor(),
        "version" => commands::version(),
        "" => (),
        _ if input_str.starts_with("color ") => commands::color(&input_str[6..].trim(), Color::Black),
        _ if input_str.starts_with("delay ") => commands::delay(input_str[6..].trim()),
        _ if input_str.starts_with("echo ") => commands::echo(&input_str[5..].trim()),
        _ if input_str.starts_with("rm ") => system_call(2, &input_str[3..].trim().as_bytes()),
        _ if input_str.starts_with("touch ") => system_call(3, &input_str[6..].trim().as_bytes()),
        _ if input_str.starts_with("mv ") => commands::mv(input_str[3..].trim()),
        _ => {
            unsafe {
                ERROR = true;
            }
            println!("ERROR: Invalid Command")
        },
    }
}