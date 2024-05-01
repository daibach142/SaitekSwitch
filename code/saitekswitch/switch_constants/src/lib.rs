/*
Driver to connect a Saitek Switch to Flightgear flight simulator

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
//!  For the Gear lever, moving the lever causes TWO actions, one for the GEARUP and the
//!  converse action for the GEARDOWN. For use as a simple switch, only configure for
//!  one of the actions e.g. GEARUP
//!    
//!  The Nasal code in FGFS expects the bit values as the action i.e. SET 1, RESET 0
//!  Writing the device requires 3 bytes of data
//!
//!
// returned 3 bytes from Saitek Switch Panel
// bit positions of flags, per packed integer
// packed as:
// 1st byte 0xff0000
// 2nd byte 0x00ff00
// 3rd byte 0x0000ff

pub const BATTERY: u32 = 0x010000;
pub const ALT: u32 = 0x020000;
pub const AVIONICS: u32 = 0x040000;
pub const FUELPUMP: u32 = 0x080000;
pub const DEICE: u32 = 0x100000;
pub const PITOTHEAT: u32 = 0x200000;
pub const COWLCLOSE: u32 = 0x400000;
pub const PANELLIGHT: u32 = 0x800000;
pub const BEACON: u32 = 0x000100;
pub const NAVLIGHTS: u32 = 0x000200;
pub const STROBE: u32 = 0x000400;
pub const TAXI: u32 = 0x000800;
pub const LANDING: u32 = 0x001000;
pub const SWITCHMASK: u32 =
	BATTERY
		| ALT | AVIONICS
		| FUELPUMP
		| DEICE | PITOTHEAT
		| COWLCLOSE
		| PANELLIGHT
		| BEACON | NAVLIGHTS
		| STROBE | TAXI
		| LANDING
		| GEARMASK;

pub const GEARUP: u32 = 0x000004;
pub const GEARDOWN: u32 = 0x000008;
pub const GEARMASK: u32 = GEARUP | GEARDOWN;

pub const MAGOFF: u32 = 0x002000;
pub const MAGR: u32 = 0x004000;
pub const MAGL: u32 = 0x008000;
pub const MAGBOTH: u32 = 0x000001;
pub const MAGSTART: u32 = 0x000002;
pub const MAGMASK: u32 = MAGOFF | MAGR | MAGL | MAGBOTH | MAGSTART;

// Write data for Gear LEDS
// LED may be yellow if RED and GREEN asserted
#[cfg(not(piped))]
pub const NOSERED: u8 = 0x08;
// pub const NOSEGREEN: u8 = 0x01;
// pub const NOSEOFF: u8 = !(NOSERED | NOSEGREEN);
// pub const LEFTRED: u8 = 0x10;

// pub const LEFTGREEN: u8 = 0x02;
// pub const LEFTOFF: u8 = !(LEFTRED | LEFTGREEN);
// pub const RIGHTRED: u8 = 0x20;
// pub const RIGHTGREEN: u8 = 0x04;
// pub const RIGHTOFF: u8 = !(RIGHTRED | RIGHTGREEN);
#[cfg(not(piped))]
pub const ALLOFF: u8 = 0x00;
// pub const LEDFIELD: u8 = 0x3f;
