/*
Driver to connect a Saitek Radio to Flightgear flight simulator
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
//!   This program links a Saitek Switch Panel to FlightGuse crate::switch::switch_device::Device;ear Flight  Simulator.
//!   The code runs (without any changes) on Linux and Windows.
mod switch;
mod simulator;

use std::env;
use switch::switch_device::Device; 
use simulator::simulator::Simulator;


fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    // default addresses

    let config = match args.next() {
        Some(arg) => arg.to_string(),
        None => "data/cessna.xml".to_string(),
    };

    println!("Configuration file {}", config);
    run(config);
    Ok(())
}

fn run(config: String) {
    let mut my_device = Device::new();
    let mut my_simulator = Simulator::new(&config);
    my_simulator.initialise_switches(my_device.get_current_input());
    my_device.preserve_current_input();
    loop {
        my_device.read(); // blocking read
                          // println!("Read: 0x{:06x}", my_device.input_current);
        if my_device.has_input_changed() {
            my_simulator.process_input(my_device.get_current_input(), my_device.get_previous_input());
        }
        my_device.preserve_current_input();
    }
}
