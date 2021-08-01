//! Driver to connect a Saitek Switch to Flightgear flight simulator
//! Copyright (C) 2021 Dave Attwood
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//! GNU General Public License for more details.
//! You should have received a copy of the GNU General Public License
//! along with this program. If not, see <https://www.gnu.org/licenses/>.
//!
//! The Saitek Switch Device consists of;
//!   13 labelled switches
//!      BATTERY, ALT, AVIONICS,FUELPUMP, DEICE, PITOTHEAT, COWLCLOSE, PANELLIGHT,
//!      BEACON, NAVLIGHTS, STROBE, TAXI, LANDING
//!   rotary Magneto switch
//!
//!      MAGOFF, MAGR, MAGL, MAGBOTH, MAGSTART
//!   a gear lever, up or down
//!   3 leds (R/O/G) for status indication
//!
//!  Any switch will trigger one or two outputs -
//!     for switches, one output (SET or RESET)
//!     for magneto, one output except for MAGBOTH, which outputs MAGSTART RESET as well
//!     
//!  The Nasal code in FGFS expects the bit values as the action i.e. SET 1, RESET 0
//!  Writing the device requires 3 bytes of data
//!
//!  For Windows compatibility, an extra bytes (value unimportant) on the end if required,
//!   without this affecting the Linux code.
//!

extern crate xml;

use crate::switch::switch_constants::*;
use crate::switch::switch_device;
use xml::reader::{EventReader, ParserConfig, XmlEvent};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::net::UdpSocket;
use std::process;
use std::{thread, time};

const SIMULATOR_INPUT_ADDRESS: &str = "127.0.0.1:60003"; // don't clash with Radio Panel
const SIMULATOR_OUTPUT_ADDRESS: &str = "127.0.0.1:60000";

pub struct Simulator {
    pub switch_mapper: HashMap<u32, String>,
    pub switch_status: HashMap<u32, u8>,
    pub magneto: String,
    pub mag_value: u32,
    pub mag_mapper: HashMap<u32, u8>,
    pub starter: String, // switch setting, value
    pub gear_retarget: String,
    pub gear_primer: String,
}

impl Simulator {
    /// Create and initialise the simulator mapping structure
    /// by loading the configuration file and forming suitable
    /// data structures for easy access
    pub fn new(config_file: &str) -> Simulator {
        let mut sim_map = Simulator {
            switch_mapper: HashMap::new(),
            switch_status: HashMap::new(),
            magneto: String::new(),
            mag_mapper: HashMap::new(),
            mag_value: 0,
            starter: String::new(),
            gear_retarget: String::new(),
            gear_primer: String::new(),
        };

        config_loader(config_file, switch_device::make_name_map(), &mut sim_map);

        sim_map
    }
    /// Set up initial values for the switches in the simulator
    /// using the current switch values (set up by the initial read)
    pub fn initialise_switches(&mut self, current_input: u32) {
        // Set initial state for all the switches
        // send the initial state for all the switches to the simulator
        // need to both store and iterate this list -  hashmap with
        // bit mask and current value -switch_status
        let delay = time::Duration::from_millis(50);
        // println!(
        //     "Enter initialise_switches - size of switch_mapper {}",
        //     &self.switch_mapper.len()
        // );
        for (key, value) in &self.switch_mapper {
            let status = if (current_input & key) == 0 { 0 } else { 1 };
            &self.switch_status.insert(*key, status);
            // println!("Key {:06x} Value {} destination {}", key, status, value);
            write_simulator(value, status);
            thread::sleep(delay);
        }
        // Magneto 0..5
        for (key, value) in &self.mag_mapper {
            if (current_input & key) != 0 {
                write_simulator(&self.magneto, *value);
                self.mag_value = *key;
                //    println!(
                //         "Initialise_switches: Magneto value preserved is 0x{:06x}",
                //         self.mag_value
                //     );
                break;
            }
        }
    }

    /// Send suitable command for the (change in) input data
    /// Current and previous values are incoming paramters
    pub fn process_input(&mut self, current_input: u32, previous_input: u32) {
        // Let's do SWITCHES
        let key = (current_input ^ previous_input) & SWITCHMASK;
        // println!("process_input key={:06x}", key);
        if key != 0 {
            // only one bit changes per read - OR NOT, bug from Bruce Maggs
            for lkey in self.switch_mapper.keys() {
                if (lkey & key) != 0 {
                    // single bit match
                    let command = self.switch_mapper.get(lkey).unwrap();
                    let value = self.switch_status.entry(*lkey).or_insert(0);
                    *value = if *value == 0 { 1u8 } else { 0u8 };
                    write_simulator(&command, *value);
                    if key == *lkey {
                        break; // quit if only one bit to match
                    }
                }
            }
        }

        // let's do MAGNETOS
        // println!("process_input: Magnetos? {:06x} nag_value {:06x}", device.get_current_input() & MAGMASK, self.mag_value);
        let key = current_input & MAGMASK;
        if (key != 0) && (key != self.mag_value) {
            // switch changed position
            if self.mag_value == MAGSTART {
                // is starter running? turn off starter now
                write_simulator(&self.starter, 0);
            }
            // println!("process_input: key {:06x} mag_value {:06x}", key, self.mag_value);
            self.mag_value = key;
            write_simulator(&self.magneto, *self.mag_mapper.get(&key).unwrap());
            if key == MAGSTART {
                write_simulator(&self.starter, 1); // extra action on the starter
            }
        }
    }
}

/// Send a command to the FGFS consisting of the simulator name for the switch to operate
///  and the action (which is one of 0, 1, 2, 3, 4)
fn write_simulator(control: &str, action: u8) {
    let data = format!("{},{}\n", control, action);
    // println!("Writing {}", data);
    let buf = data.into_bytes();
    // Following required to avoid getting 'address in use' error
    // Copied from https://illegalargumentexception.blogspot.com/2015/05/rust-send-and-receive-on-localhost-with.html
    let socket = UdpSocket::bind(SIMULATOR_INPUT_ADDRESS).expect("Socket create incoming error");
    // of course, the simulator READS from this address
    socket
        .send_to(&buf, SIMULATOR_OUTPUT_ADDRESS)
        .expect("Socket send error");
}

// ------------------------------------------------------------------------------------------------------------------------
// CONFIGURATION processing
// ------------------------------------------------------------------------------------------------------------------------

// Tag names in XML configuration file
enum StartType {
    Plane,
    Switch,
    Magnetos,
    Starter,
    GearRetarget,
    GearPrimer,
}

/// Processes the configuration file to build the mapping tables in the simulator
/// This is called from 'new', so there is no 'self' yet
fn config_loader(filename: &str, devmap: HashMap<String, u32>, config_data: &mut Simulator) {
    let file = File::open(filename).unwrap_or_else(|_e| {
        println!("Unable to access configuration file '{}'", filename);
        process::exit(4);
    });
    let file = BufReader::new(file);

    let parser = EventReader::new_with_config(file, ParserConfig::new().trim_whitespace(true));
    let mut mode = StartType::Plane;
    let mut switch: u32 = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let mut sname = String::new();
                match name.local_name.as_str() {
                    "switch" => {
                        for oa in attributes {
                            // println!("Attributes: {:?}", oa);
                            if oa.name.local_name == "name" {
                                sname = oa.value;
                            }
                        }
                        // println!("sname={:?}", sname);
                        match devmap.get(&sname) {
                            Some(x) => switch = *x,
                            None => panic!("Uknown SWITCH name {:?}", sname),
                        }
                        mode = StartType::Switch;
                    }
                    "magnetos" => mode = StartType::Magnetos,
                    "starter" => mode = StartType::Starter,
                    "gear-retarget" => mode = StartType::GearRetarget,
                    "gear-primer" => mode = StartType::GearPrimer,
                    "plane" => mode = StartType::Plane,
                    _ => panic!("Unexpected element <{}>", name.local_name),
                }
            }
            Ok(XmlEvent::Characters(data)) => match mode {
                StartType::Plane => {
                    println!("Configured for {}", data);
                }
                StartType::Switch => {
                    // println!("0x{:06x}={}", switch, data);
                    config_data.switch_mapper.insert(switch, data);
                    config_data.switch_status.insert(switch, 0u8);
                }
                StartType::Magnetos => {
                    // println!("Magneto=\"{}\"", data);
                    config_data.magneto = data;
                }
                StartType::Starter => {
                    // println!("Starter=\"{}\"", data);
                    config_data.starter = data;
                }
                StartType::GearRetarget => {
                    // println!("GearRetarget{:?}", data);
                    config_data.gear_retarget = data;
                }
                StartType::GearPrimer => {
                    // println!("GearPrimer {:?}", data);
                    config_data.gear_primer = data;
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    if config_data.switch_mapper.len() < 13 {
        panic!(
            "Error: you neeed 13 SWITCH elements, only {} unique ones provided",
            config_data.switch_mapper.len()
        );
    }
    config_data.mag_mapper.insert(MAGOFF, 0);
    config_data.mag_mapper.insert(MAGR, 1);
    config_data.mag_mapper.insert(MAGL, 2);
    config_data.mag_mapper.insert(MAGBOTH, 3);
    config_data.mag_mapper.insert(MAGSTART, 4);
}
