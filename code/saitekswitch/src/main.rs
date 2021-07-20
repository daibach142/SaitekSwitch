/*
Driver to connect a Saitek Switch Panel to Flightgear flight simulator
Copyright (C) 2021 Dave Attwood

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

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
//!    in the source code.
//! 
mod simulator;
mod switch;

use simulator::simulator::Simulator;
use std::env;
use switch::switch_device::Device;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    // Configuration file path
    let config = match args.next() {
        Some(arg) => arg.to_string(),
        None => "data/cessna.xml".to_string(),
    };

    println!("Configuration file {}", config);
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
