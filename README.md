# H-3X: Operating System Kernel in Rust

H-3X is a Rust-based operating system kernel featuring a basic shell with commands for system info, screen clearing, and rebooting.

## Features

- **Basic Shell**: A simple command-line interface to interact with the kernel.
- **Commands**:
  - `architecture`: Displays the system architecture (x86_64).
  - `bootloader`: Information about the bootloader (rust bootimage-generated).
  - `clear`: Clears the screen.
  - `echo [message]`: Echoes a message.
  - `halt`: Halts the CPU.
  - `help`: Lists all available commands.
  - `info`: Displays system information (architecture, bootloader, vendor, version).
  - `reboot`: Reboots the system.
  - `sleep`: Sleeps for a set duration (for testing purposes).
  - `vendor`: Displays CPU vendor string.
  - `version`: Displays the kernel version.

## Credits

This project was heavily inspired by the blog series **"Writing an OS in Rust"** by **Phillip Oppermann**. The blog series provided the foundation and step-by-step guide to building a kernel in Rust, which guided the development of this OS. If you're interested in learning more about operating system development in Rust, I highly recommend checking it out:

- [Writing an OS in Rust by Phillip Oppermann](https://os.phil-opp.com)

Thank you to Phillip Oppermann for the incredible resource!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.