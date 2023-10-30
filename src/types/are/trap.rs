#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::util::{SectionAddress, Readable, Point3D};

/**
The fully parsed contents of a Projectile Trap in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 8 | Projectile resref
0x0008 | 4 | Effect block offset
0x000c | 2 | Effect block size
0x000e | 2 | Missile.ids reference (projectl.ids - 1)
0x0010 | 2 | Ticks until next trigger check
0x0012 | 2 | Triggers remaining (i.e. explosion count)
0x0014 | 2 | X coordinate
0x0016 | 2 | Y coordinate
0x0018 | 2 | Z coordinate
0x001a | 1 | Enemy-ally targeting
0x001b | 1 | Party member index which created this projectile (0-5)
*/
#[derive(Clone, Debug, Default)]
pub struct AreProjectileTrap
{
	pub projectile: String,
	pub effectBlock: SectionAddress<u32, u16>,
	pub missileRef: u16,
	pub ticks: u16,
	pub triggersRemaining: u16,
	pub coordinate: Point3D<u16>,
	pub friendlyFire: u8,
	pub creator: u8,
}

impl Readable for AreProjectileTrap
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let projectile = readResRef(cursor)?;
		let effectBlock = SectionAddress::<u32, u16>::fromCursor(cursor)?;
		let missileRef = cursor.read_u16::<LittleEndian>()?;
		let ticks = cursor.read_u16::<LittleEndian>()?;
		let triggersRemaining = cursor.read_u16::<LittleEndian>()?;
		let coordinate = Point3D::<u16>::fromCursor(cursor)?;
		let friendlyFire = cursor.read_u8()?;
		let creator = cursor.read_u8()?;
		
		return Ok(Self
		{
			projectile,
			effectBlock,
			missileRef,
			ticks,
			triggersRemaining,
			coordinate,
			friendlyFire,
			creator,
		});
	}
}
