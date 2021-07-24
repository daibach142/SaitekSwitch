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
// returned 3 bytes from Saitek Switch Panel
// bit positions of flags, per packed integer
// packed as:
// 1st byte 0x??0000
// 2nd byte 0x00??00
// 3rd byte 0x0000??

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
		| LANDING | GEARMASK;

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
