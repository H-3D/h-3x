# H-3X

H-3X is a Rust-based operating system kernel featuring a basic shell, system calls, error handling and commands.

## Features

- **Basic Shell**
- **System Calls**
- **Error Handling**
- **Commands**:
  - `architecture`: Displays the system architecture (x86_64).
  - `bootloader`: Information about the bootloader (bootloader v0.9 crate).
  - `calculator`: Interactive calculator mode for basic arithmetic operations.
  - `clear`: Clears the screen.
  - `color [color]`: Changes the text color.
  - `cpu`: Displays the CPU brand string.
  - `delay`: Sleeps for a set duration (for testing purposes).
  - `echo [message]`: Echoes a message.
  - `flix`: Buffer Text Editor.
  - `flox`: Ephemeral Text Editor.
  - `halt`: Halts the CPU.
  - `help`: Lists all available commands.
  - `info`: Displays system information.
  - `ls`: Displays the contents of the variable.
  - `manual`: Displays the system manual.
  - `purge`: Deletes all the text in the variable.
  - `reboot`: Reboots the system.
  - `time`: Displays the Real Time Clock.
  - `touch [text]`: Appends your text to a variable.
  - `uptime`: Displays the system uptime.
  - `vendor`: Displays CPU vendor string.
  - `version`: Displays the kernel version.

## Credits

This project was heavily inspired by the blog series **"Writing an OS in Rust"** by **Phillip Oppermann**. The blog series provided the foundation and step-by-step guide to building a kernel in Rust, which guided the development of this OS. If you're interested in learning more about operating system development in Rust, I highly recommend checking it out:

- [Writing an OS in Rust by Phillip Oppermann](https://os.phil-opp.com)

Thank you to Phillip Oppermann for the incredible resource!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.