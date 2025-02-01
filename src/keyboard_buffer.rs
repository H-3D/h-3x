use x86_64::instructions::port::Port;

static mut SHIFT_PRESSED: bool = false;

pub fn read_char() -> char {
    let mut status_port: Port<u8> = Port::new(0x64);
    let mut data_port: Port<u8> = Port::new(0x60);

    while unsafe { status_port.read() } & 1 == 0 {}

    let scancode = unsafe { data_port.read() };

    // Check for key release
    if scancode & 0x80 != 0 {
        let key_released = scancode & 0x7F;
        if key_released == 0x2A || key_released == 0x36 {
            unsafe { SHIFT_PRESSED = false; }
        }
        return '\0';
    }

    // Check for shift key press
    if scancode == 0x2A || scancode == 0x36 {
        unsafe { SHIFT_PRESSED = true; }
        return '\0';
    }

    scancode_to_char(scancode)
}

fn scancode_to_char(scancode: u8) -> char {
    match scancode {
        0x02 => if unsafe { SHIFT_PRESSED } { '!' } else { '1' },
        0x03 => if unsafe { SHIFT_PRESSED } { '@' } else { '2' },
        0x04 => if unsafe { SHIFT_PRESSED } { '#' } else { '3' },
        0x05 => if unsafe { SHIFT_PRESSED } { '$' } else { '4' },
        0x06 => if unsafe { SHIFT_PRESSED } { '%' } else { '5' },
        0x07 => if unsafe { SHIFT_PRESSED } { '^' } else { '6' },
        0x08 => if unsafe { SHIFT_PRESSED } { '&' } else { '7' },
        0x09 => if unsafe { SHIFT_PRESSED } { '*' } else { '8' },
        0x0A => if unsafe { SHIFT_PRESSED } { '(' } else { '9' },
        0x0B => if unsafe { SHIFT_PRESSED } { ')' } else { '0' },
        0x10 => if unsafe { SHIFT_PRESSED } { 'Q' } else { 'q' },
        0x11 => if unsafe { SHIFT_PRESSED } { 'W' } else { 'w' },
        0x12 => if unsafe { SHIFT_PRESSED } { 'E' } else { 'e' },
        0x13 => if unsafe { SHIFT_PRESSED } { 'R' } else { 'r' },
        0x14 => if unsafe { SHIFT_PRESSED } { 'T' } else { 't' },
        0x15 => if unsafe { SHIFT_PRESSED } { 'Y' } else { 'y' },
        0x16 => if unsafe { SHIFT_PRESSED } { 'U' } else { 'u' },
        0x17 => if unsafe { SHIFT_PRESSED } { 'I' } else { 'i' },
        0x18 => if unsafe { SHIFT_PRESSED } { 'O' } else { 'o' },
        0x19 => if unsafe { SHIFT_PRESSED } { 'P' } else { 'p' },
        0x1E => if unsafe { SHIFT_PRESSED } { 'A' } else { 'a' },
        0x1F => if unsafe { SHIFT_PRESSED } { 'S' } else { 's' },
        0x20 => if unsafe { SHIFT_PRESSED } { 'D' } else { 'd' },
        0x21 => if unsafe { SHIFT_PRESSED } { 'F' } else { 'f' },
        0x22 => if unsafe { SHIFT_PRESSED } { 'G' } else { 'g' },
        0x23 => if unsafe { SHIFT_PRESSED } { 'H' } else { 'h' },
        0x24 => if unsafe { SHIFT_PRESSED } { 'J' } else { 'j' },
        0x25 => if unsafe { SHIFT_PRESSED } { 'K' } else { 'k' },
        0x26 => if unsafe { SHIFT_PRESSED } { 'L' } else { 'l' },
        0x2C => if unsafe { SHIFT_PRESSED } { 'Z' } else { 'z' },
        0x2D => if unsafe { SHIFT_PRESSED } { 'X' } else { 'x' },
        0x2E => if unsafe { SHIFT_PRESSED } { 'C' } else { 'c' },
        0x2F => if unsafe { SHIFT_PRESSED } { 'V' } else { 'v' },
        0x30 => if unsafe { SHIFT_PRESSED } { 'B' } else { 'b' },
        0x31 => if unsafe { SHIFT_PRESSED } { 'N' } else { 'n' },
        0x32 => if unsafe { SHIFT_PRESSED } { 'M' } else { 'm' },
        0x1C => '\n', // Enter
        0x39 => ' ',  // Space
        0x0C => if unsafe { SHIFT_PRESSED } { '_' } else { '-' },  // Dash
        0x0D => if unsafe { SHIFT_PRESSED } { '+' } else { '=' },  // Equals
        0x1A => if unsafe { SHIFT_PRESSED } { '{' } else { '[' },  // Left bracket
        0x1B => if unsafe { SHIFT_PRESSED } { '}' } else { ']' },  // Right bracket
        0x27 => if unsafe { SHIFT_PRESSED } { ':' } else { ';' },  // Semicolon
        0x28 => if unsafe { SHIFT_PRESSED } { '"' } else { '\'' }, // Apostrophe
        0x29 => if unsafe { SHIFT_PRESSED } { '~' } else { '`' },  // Backtick
        0x2B => if unsafe { SHIFT_PRESSED } { '|' } else { '\\' }, // Backslash
        0x33 => if unsafe { SHIFT_PRESSED } { '<' } else { ',' },  // Comma
        0x34 => if unsafe { SHIFT_PRESSED } { '>' } else { '.' },  // Period
        0x35 => if unsafe { SHIFT_PRESSED } { '?' } else { '/' },  // Forward slash
        _ => '?',     // Unknown character
    }
}