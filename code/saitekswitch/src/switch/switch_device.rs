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

// use crate::simulator::simulator::Simulator;
use crate::switch::switch_constants::*;

use std::collections::HashMap;

#[cfg(piped)]
use std::io::{self, Read};

#[cfg(not(piped))]
use hidapi::{HidApi, HidDevice};

#[cfg(not(piped))]
use std::process;

// Saitek is 6a3, switch is d67, radio is d08
#[cfg(not(piped))]
const VENDOR_ID: u16 = 0x06a3;
#[cfg(not(piped))]
const SWITCH_ID: u16 = 0x0d67;

const RIGHT_SIZE: usize = 4; // 1 byte at end unused, required on Windows hidapi

//-------------------------------------------------------------------------------
#[cfg(not(piped))]
pub struct Device {
    device: HidDevice,  // for device reads and writes
    input_current: u32, // data from device
    input_old: u32,     // previous data
}

// if data is piped to this driver, input is via STDIN, so there is no need to
// hold any device information
#[cfg(piped)]
pub struct Device {
    input_current: u32, // data from device
    input_old: u32,     // previous data
}

impl Device {
    /// Create an instance of the Saitek Switch device.
    /// The (first) device is located by vendor and device ID.
    /// The device is initialised and set for blocking reads
    #[cfg(not(piped))]
    pub fn new() -> Device {
        let ctxt = HidApi::new().unwrap();

        let mut r = Device {
            // ctxt: HidApi::new().unwrap(),
            device: ctxt.open(VENDOR_ID, SWITCH_ID).unwrap_or_else(|_err| {
                println!("Saitek Switch not found");
                process::exit(1);
            }),
            input_current: 0, // adjusted during initialise_device
            input_old: 0,
        };

        // set up display & read selections, device is always a blocking read
        r.input_current = initialise_device(&r.device);

        r
    }

    /// Create an instance of the Saitek Switch device as piped data
    /// from the switch panel emulator
    /// The device is initialised, and set for blocking reads
    #[cfg(piped)]
    pub fn new() -> Device {
        let mut r = Device {
            input_current: 0, // adjusted during initialise_device
            input_old: 0,
        };

        // set up display & read selections, device is always a blocking read
        r.input_current = initialise_device();

        r
    }

    /// Blocking read of the device into the 'input_current' field in the Device
    /// struct. If there is no data, does not disturb the 'input_current' field.
    /// Three data bytes are provided by the switch panel and are packed into a u32 such that
    /// the bit positions and other masks in 'switch_constants.rs' coincide.
    #[cfg(not(piped))]
    pub fn read(&mut self) {
        // Non-blocking read the radio panel switches and selectors
        // return 0 if no data, else pack 3 bytes into ls part of u32
        let mut buf = [0u8; RIGHT_SIZE];
        let read_length = match self.device.read(&mut buf) {
            Ok(l) => l,  // good read, should be 4
            Err(_) => 0, // probably non-blocking - no data
        };
        // device sends RIGHT_SIZE bytes, 0 on error
        if read_length > 2 {
            self.input_current = pack(&buf);
            // println!("Read {:6x}", self.input_current);
        }
    }

    /// Blocking read of the device into the 'input_current' field in the Device
    /// struct.
    /// Three data bytes are provided by the switch panel and are packed into a u32 such that
    /// the bit positions and other masks in 'switch_constants.rs' coincide.
    #[cfg(piped)]
    pub fn read(&mut self) {
        // println!("Read - enter");
        // blocking read the radio panel switches and selectors
        // return 0 if no data, else pack 3 bytes into ls part of u32
        let mut buf = [0u8; RIGHT_SIZE];
        let read_length = match io::stdin().read(&mut buf) {
            Ok(l) => l,  // good read, should be 4
            Err(_) => 0, // probably non-blocking - no data
        };
        // println!("Read complete - {} bytes read", read_length);
        // device sends RIGHT_SIZE bytes, 0 on error
        if read_length > 2 {
            self.input_current = pack(&buf);
            // println!("Read 0x{:06x} previous 0x{:06x}", self.input_current, self.input_old);
        }
    }

    //-----------------------------------------------------------------------------------------

    /// Returns the current input value
    pub fn get_current_input(&self) -> u32 {
        self.input_current
    }

    /// Returns the old input value
    pub fn get_previous_input(&self) -> u32 {
        self.input_old
    }

    /// Saves the current input as the old input
    pub fn preserve_current_input(&mut self) {
        self.input_old = self.input_current;
    }

    /// Compares the current input  with the old input.
    /// Returns true if they differ.
    pub fn has_input_changed(&self) -> bool {
        self.input_current != self.input_old
    }
}

//----------------------------------------------------------------------------------------

/// Sends a RED led to the nosewheel, waiting for a key to be pressed.
/// When key is received, clears the led, and returns the
/// value read from the device, which gives the current switch settings.
#[cfg(not(piped))]
fn initialise_device(device: &HidDevice) -> u32 {
    let reply: u32;
    let mut buf = [0u8; RIGHT_SIZE];
    let mut obuf: [u8; 2] = [0, NOSERED];
    device.send_feature_report(&obuf).unwrap(); // nose light set red
    println!("Operate a key on the Saitek Switch");
    let rsize = device.read(&mut buf).unwrap_or_else(|_| {
        println!("Saitek Switch read error");
        process::exit(3);
    });
    if rsize > 2 {
        reply = pack(&buf);
    } else {
        reply = 0;
    }

    obuf[1] = ALLOFF;
    device.send_feature_report(&obuf).unwrap(); // all leds off
    println!("Saitek Switch ready");

    reply
}

/// Waits for a key to be pressed.
/// When key is received, returns the
/// value read from the device, which gives the current switch settings.
#[cfg(piped)]
fn initialise_device() -> u32 {
    let reply: u32;
    let mut buf = [0u8; RIGHT_SIZE];
    println!("Operate a key on the Saitek Switch");
    let rsize = io::stdin().read(&mut buf).unwrap();
    if rsize > 2 {
        reply = pack(&buf);
    } else {
        reply = 0;
    }
    println!("Saitek Switch ready 0x{:06x}", reply);

    reply
}

//-------------------------------------------------------------------------------------------------------

/// Pack first 3 bytes of the incoming bufffer into a u32 value (ls 3 bytes)
fn pack(buf: &[u8]) -> u32 {
    // println!("pack: {:?}", buf);
    (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32)
}

/// Create a hashmap keyed by the (text) switch name, with the switch bit-value as the data.
/// This data is all static information.
///
pub fn make_name_map() -> HashMap<String, u32> {
    let mut devmap: HashMap<String, u32> = HashMap::new();
    devmap.insert("BATTERY".to_string(), BATTERY);
    devmap.insert("ALT".to_string(), ALT);
    devmap.insert("AVIONICS".to_string(), AVIONICS);
    devmap.insert("FUELPUMP".to_string(), FUELPUMP);
    devmap.insert("DEICE".to_string(), DEICE);
    devmap.insert("PITOTHEAT".to_string(), PITOTHEAT);
    devmap.insert("COWLCLOSE".to_string(), COWLCLOSE);
    devmap.insert("PANELLIGHT".to_string(), PANELLIGHT);
    devmap.insert("BEACON".to_string(), BEACON);
    devmap.insert("NAVLIGHTS".to_string(), NAVLIGHTS);
    devmap.insert("STROBE".to_string(), STROBE);
    devmap.insert("TAXI".to_string(), TAXI);
    devmap.insert("LANDING".to_string(), LANDING);
    devmap.insert("GEARUP".to_string(), GEARUP);
    devmap.insert("GEARDOWN".to_string(), GEARDOWN);
    devmap
}
