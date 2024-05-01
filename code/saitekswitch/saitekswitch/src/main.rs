
/*
Driver to connect a Saitek Switch Panel to Flightgear flight simulator

MIT License

Copyright (c) 2024 Dave Attwood

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

 */

//!
//!   This program links a Saitek Switch Panel to FlightGear Flight  Simulator.
//!   The code runs (without any changes) on Linux and Windows.
//!
//!   To aid testing without a switch panel, a Switch Panel Emulator (not as a HID device!)
//!   provides input in an identical manner to the real Switch Panel
//!   This requires compiling with:
//!       "cargo rustc -- --cfg piped"
//!    and there are occasional "#[cfg(piped)]" or "#[cfg(not(piped))]" for conditional compilation
//!    in the source code in the 'switch' modules.
//!

use simulator::Simulator;
use std::env;
use switch::Device;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    // Configuration file path
    let config = match args.next() {
        Some(arg) => arg.to_string(),
        None => "switchdefaultconfig.xml".to_string(),
    };

    println!(
        "{} Version {} Configuration file {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        config
    );
    run(config);
    Ok(())
}

/// Indefinite loop Driver for the switch panel to simulator interfaced
fn run(config: String) {
    let mut my_device = Device::new(); // access the device
    let mut my_simulator = Simulator::new(&config); // map device to simulator
    my_simulator.initialise_switches(my_device.get_current_input()); // initial switch settings provided from Device::new
    my_device.preserve_current_input();
    loop {
        my_device.read(); // blocking read
                          // println!("Read: 0x{:06x}", my_device.input_current);
        if my_device.has_input_changed() {
            my_simulator.process_input(
                my_device.get_current_input(),
                my_device.get_previous_input(),
            );
        }
        my_device.preserve_current_input();
    }
}
