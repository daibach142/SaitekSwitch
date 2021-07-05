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
		| LANDING;

pub const GEARUP: u32 = 0x000004;
pub const GEARDOWN: u32 = 0x000008;
pub const _GEARMASK: u32 = GEARUP | GEARDOWN;

pub const MAGOFF: u32 = 0x002000;
pub const MAGR: u32 = 0x004000;
pub const MAGL: u32 = 0x008000;
pub const MAGBOTH: u32 = 0x000001;
pub const MAGSTART: u32 = 0x000002;
pub const MAGMASK: u32 = MAGOFF | MAGR | MAGL | MAGBOTH | MAGSTART;

// Write data for Gear LEDS
// LED may be yellow if RED and GREEN asserted
pub const NOSERED: u8 = 0x08;
// pub const NOSEGREEN: u8 = 0x01;
// pub const NOSEOFF: u8 = !(NOSERED | NOSEGREEN);
// pub const LEFTRED: u8 = 0x10;

// pub const LEFTGREEN: u8 = 0x02;
// pub const LEFTOFF: u8 = !(LEFTRED | LEFTGREEN);
// pub const RIGHTRED: u8 = 0x20;
// pub const RIGHTGREEN: u8 = 0x04;
// pub const RIGHTOFF: u8 = !(RIGHTRED | RIGHTGREEN);
pub const ALLOFF: u8 = 0x00;
// pub const LEDFIELD: u8 = 0x3f;
