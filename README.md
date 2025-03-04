# arduino-flash-controller

## Setting up the Rust environment for Arduino Nano

To compile and flash this code to an Arduino Nano, you'll need to set up your development environment:


1. **Install Rust and required tools**:
#### Installing Rust with rustup in user directory

*Install rustup* (accept the warning about existing installation):
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

When prompted about the existing installation, type ```y``` to continue

Choose option 1 (default installation) when prompted


*Configure your shell* to use the rustup-installed version:
```source $HOME/.cargo/env```

This adds rustup's binaries to your PATH for the current session


*Make the change permanent* by adding this to your shell profile:
```echo 'source $HOME/.cargo/env' >> ~/.bashrc  # for bash
# OR
echo 'source $HOME/.cargo/env' >> ~/.zshrc   # for zsh```

*Verify rustup* is being used:
```which rustc  # Should show ~/.cargo/bin/rustc
rustc --version```

*Now add the AVR target:*
```rustup target add avr-atmega328p
```

2. **Install additional dependencies**:
   ```bash
   sudo apt install avr-libc gcc-avr pkg-config avrdude
   ```
   (Use the appropriate package manager for your OS)

3. **Create a new Rust project**:
   ```bash
   cargo new --bin arduino_flash_controller
   cd arduino_flash_controller
   ```

4. **Configure your project**:
   Create a `.cargo/config.toml` file with:
   ```toml
   [build]
   target = "avr-atmega328p"

   [unstable]
   build-std = ["core"]
   ```

5. **Add required dependencies to Cargo.toml**:
   ```toml
   [dependencies]
   arduino-hal = "0.1.0"
   panic-halt = "0.2.0"
   nb = "1.0.0"
   avr-device = "0.3.2"
   ```

6. **Compile and flash**:
   ```bash
   cargo build --release
   avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 57600 -U flash:w:target/avr-atmega328p/release/arduino_flash_controller.elf
   ```
   (Replace `/dev/ttyUSB0` with your actual port)

The key differences from the JavaScript code:

1. Instead of callback-based event handling, we use a polling approach in a loop
2. The code runs directly on the Arduino, not on a computer communicating with it
3. We're working with minimal resources, so we use a fixed-size buffer
4. We handle specific commands (like the byte value 11) directly in our code

This Rust implementation should be more efficient and reliable since it runs directly on the hardware without the overhead of Node.js.