#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::serial::Serial;
use avr_device::atmega328p::USART0;
use core::fmt::Write;
use nb::block;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // Set up serial communication
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    
    // Print a startup message
    writeln!(&mut serial, "Arduino Nano serial communication started").unwrap();
    
    // Buffer to store incoming data
    let mut buffer = [0u8; 32];
    let mut index = 0;
    
    // Main program loop
    loop {
        // Check for incoming data
        match serial.read() {
            Ok(byte) => {
                // Handle the received byte
                buffer[index] = byte;
                index = (index + 1) % buffer.len();
                
                // Print the data we received
                writeln!(&mut serial, "Received byte: {}", byte).unwrap();
                
                // Optional: If a specific command is received, respond
                if byte == 11 {
                    writeln!(&mut serial, "Special command received!").unwrap();
                    // Perform some action here
                }
            }
            Err(nb::Error::WouldBlock) => {
                // No data available, continue with other tasks
                arduino_hal::delay_ms(10);
            }
            Err(nb::Error::Other(_)) => {
                // Handle error
                writeln!(&mut serial, "Error reading from serial").unwrap();
            }
        }
        
        // Your main program logic can go here
    }
}