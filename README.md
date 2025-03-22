# H-3X:

H-3X is a minimalist operating system built in Rust, designed for educational exploration into OS development and low-level programming.

## Getting Started:

> **Note:**  
> Windows users should use the Windows Subsystem for Linux (WSL) for optimal compatibility. For detailed instructions, please refer to the [official Microsoft guide](https://docs.microsoft.com/en-us/windows/wsl/install).

- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Source the corresponding env file under $HOME/.cargo:
    - sh/bash/zsh/ash/dash/pdksh: `. "$HOME/.cargo/env"`
    - fish: `source "$HOME/.cargo/env.fish"`
    - nushell: `source "$HOME/.cargo/env.nu"`
- Install QEMU:
    - Linux:
        - Arch Linux: `sudo pacman -S qemu`
        - Debian/Ubuntu: `sudo apt install qemu-system`
        - Fedora: `sudo dnf install @virtualization`
        - Gentoo: `sudo emerge --ask app-emulation/qemu`
        - RHEL/CentOS: `sudo yum install qemu-kvm`
        - openSUSE: `sudo zypper install qemu`
    - macOS:
        - Homebrew: `brew install qemu`
        - MacPorts: `sudo port install qemu`
- Clone the repository: `git clone https://github.com/H-3D/h-3x`
- Enter the 'h-3x' directory: `cd h-3x`
- Set the project to use the Rust Nightly Compiler: `rustup override set nightly`
- Add the Rust standard library source to the nightly toolchain for a 64-bit Linux system: `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`
- Install the LLVM tools preview component for your current Rust toolchain: `rustup component add llvm-tools-preview`
- Install the bootimage tool: `cargo install bootimage`
- Compile the project to create the **x86_64-bootloader.json** file: `cargo build`
- Insert the **"rustc-abi": "x86-softfloat",** line into the **x86_64-bootloader.json** file after the **"features": "-mmx,-sse,+soft-float",** line: <code style="white-space: pre-wrap;">sed -i '/"features": "-mmx,-sse,+soft-float",/a \    "rustc-abi": "x86-softfloat",' ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bootloader-0.9.29/x86_64-bootloader.json</code>
- Build and run the project using Cargo: `cargo run`

## Commands:

- `architecture`: Displays the system architecture (x86_64).
- `bootloader`: Information about the bootloader (bootloader v0.9 crate).
- `buffer`: Buffer Text Editor.
- `calculator`: Interactive calculator mode for basic arithmetic operations.
- `clear`: Clears the screen.
- `color [color]`: Changes the text color.
- `cpu`: Displays the CPU brand string.
- `delay [cycles]`: Sleeps for the specified number of cycles.
- `echo [message]`: Echoes a message.
- `ephemeral`: Ephemeral Text Editor.
- `halt`: Halts the CPU.
- `help`: Lists all available commands.
- `info`: Displays system information.
- `ls`: Displays the contents of the variable.
- `manual`: Displays the system manual.
- `mv [previous text] [updated text]`: Replaces the previous text with the updated text.
- `purge`: Deletes all the text in the variable.
- `reboot`: Reboots the system.
- `rm [text]`: Removes the specified text from the variable.
- `time`: Displays the Real Time Clock.
- `touch [text]`: Appends your text to a variable.
- `uptime`: Displays the system uptime.
- `vendor`: Displays CPU vendor string.
- `version`: Displays the kernel version.

## Credits:

H-3X was inspired by Phillip Oppermann's **"Writing an OS in Rust"** series, which provided a fantastic foundation for building a kernel in Rust. This project is an educational tool aimed at demystifying the inner workings of operating systems.

- [Writing an OS in Rust by Phillip Oppermann](https://os.phil-opp.com)

Thank you, Phillip Oppermann, for the incredible resource!

## License:

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.