#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::{Readable, Point2D};

/**
The fully parsed contents of an Ambient in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | X coordinate
0x0022 | 2 | Y coordinate
0x0024 | 2 | Radius
0x0026 | 2 | Height
0x0028 | 4 | Pitch variance
0x002c | 2 | Volume variance
0x002e | 2 | Volume (%)
0x0030 | 8 | Resref of sound
0x0038 | 8 | Resref of sound
0x0040 | 8 | Resref of sound
0x0048 | 8 | Resref of sound
0x0050 | 8 | Resref of sound
0x0058 | 8 | Resref of sound
0x0060 | 8 | Resref of sound
0x0068 | 8 | Resref of sound
0x0070 | 8 | Resref of sound
0x0078 | 8 | Resref of sound
0x0080 | 2 | Count of sounds
0x0082 | 2 | Unused
0x0084 | 4 | Base time, in seconds, between sounds from this ambient list
0x0088 | 4 | Base time deviation
0x008c | 4 | Ambient appearance schedule
0x0090 | 4 | Flags
*/
#[derive(Clone, Debug, Default)]
pub struct AreAmbient
{
	pub name: String,
	pub coordinate: Point2D<u16>,
	pub radius: u16,
	pub height: u16,
	pub pitchVariance: u32,
	pub volumeVariance: u16,
	pub volume: u16,
	pub sounds: Vec<String>,
	pub soundCount: u16,
	pub soundInterval: u32,
	pub soundDeviation: u32,
	pub appearanceSchedule: u32,
	pub flags: u32,
}

impl AreAmbient
{
	const UnusedPadding: u64 = 64;
	const MaxSounds: usize = 10;
}

impl Readable for AreAmbient
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let name = readName(cursor)?;
		let coordinate = Point2D::<u16>::fromCursor(cursor)?;
		let radius = cursor.read_u16::<LittleEndian>()?;
		let height = cursor.read_u16::<LittleEndian>()?;
		let pitchVariance = cursor.read_u32::<LittleEndian>()?;
		let volumeVariance = cursor.read_u16::<LittleEndian>()?;
		let volume = cursor.read_u16::<LittleEndian>()?;
		
		let mut sounds = vec![];
		for _ in 0..Self::MaxSounds
		{
			let sound = readResRef(cursor)?;
			sounds.push(sound);
		}
		
		let soundCount = cursor.read_u16::<LittleEndian>()?;
		let _unused = cursor.read_u16::<LittleEndian>()?;
		let soundInterval = cursor.read_u32::<LittleEndian>()?;
		let soundDeviation = cursor.read_u32::<LittleEndian>()?;
		let appearanceSchedule = cursor.read_u32::<LittleEndian>()?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			coordinate,
			radius,
			height,
			pitchVariance,
			volumeVariance,
			volume,
			sounds,
			soundCount,
			soundInterval,
			soundDeviation,
			appearanceSchedule,
			flags,
		});
	}
}
