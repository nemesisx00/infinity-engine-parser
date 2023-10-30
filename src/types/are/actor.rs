#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::{SectionAddress, Readable, Point2D};

/**
The fully parsed contents of an Actor in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | Current X coordinate
0x0022 | 2 | Current Y coordinate
0x0024 | 2 | Destination X coordinate
0x0026 | 2 | Destination Y coordinate
0x0028 | 4 | Flags
0x002c | 2 | Is random monster? 0: No / 1: Yes
0x002e | 1 | First letter of CRE resref (changed to *)
0x002f | 1 | Unused
0x0030 | 4 | Actor animation
0x0034 | 2 | Actor orientation
0x0036 | 2 | Unused
0x0038 | 4 | Actor removal timer (seconds) (usually -1 to avoid removal)
0x003c | 2 | Movement restriction distance
0x003e | 2 | Movement restriction distance (move to object)
0x0040 | 4 | Actor appearance schedule. Bits 0-23 represent an hour of game time. Setting a bit means the actor will appear in the area for the corresponding hour.
0x0044 | 4 | NumTimesTalkedTo (in SAV files)
0x0048 | 8 | Dialog
0x0050 | 8 | Script (Override)
0x0058 | 8 | Script (General)
0x0060 | 8 | Script (Class)
0x0068 | 8 | Script (Race)
0x0070 | 8 | Script (Default)
0x0078 | 8 | Script (Specific)
0x0080 | 8 | CRE file
0x0088 | 4 | Offset to CRE structure (for embedded CRE files)
0x008c | 4 | Size of stored CRE structure
*/
#[derive(Clone, Debug, Default)]
pub struct AreActor
{
	pub name: String,
	pub current: Point2D<u16>,
	pub destination: Point2D<u16>,
	pub flags: u32,
	pub randomMonster: u16,
	pub creFirstLetter: u8,
	pub animation: u32,
	pub orientation: u16,
	pub removalTimer: u32,
	pub movementRestrictionDistance: u16,
	pub movementRestrictionDistance2: u16,
	pub appearanceSchedule: u32,
	pub conversedCount: u32,
	pub dialog: String,
	pub scriptOverride: String,
	pub scriptGeneral: String,
	pub scriptClass: String,
	pub scriptRace: String,
	pub scriptDefault: String,
	pub scriptSpecific: String,
	pub cre: String,
	pub creAddress: SectionAddress<u32, u32>,
}

impl AreActor
{
	pub const UnusedPadding: u64 = 128;
}

impl Readable for AreActor
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readName(cursor)?;
		let current = Point2D::<u16>::fromCursor(cursor)?;
		let destination = Point2D::<u16>::fromCursor(cursor)?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		let randomMonster = cursor.read_u16::<LittleEndian>()?;
		let creFirstLetter = cursor.read_u8()?;
		let _unknown = cursor.read_u8()?;
		let animation = cursor.read_u32::<LittleEndian>()?;
		let orientation = cursor.read_u16::<LittleEndian>()?;
		let _unknown = cursor.read_u16::<LittleEndian>()?;
		let removalTimer = cursor.read_u32::<LittleEndian>()?;
		let movementRestrictionDistance = cursor.read_u16::<LittleEndian>()?;
		let movementRestrictionDistance2 = cursor.read_u16::<LittleEndian>()?;
		let appearanceSchedule = cursor.read_u32::<LittleEndian>()?;
		let conversedCount = cursor.read_u32::<LittleEndian>()?;
		let dialog = readResRef(cursor)?;
		let scriptOverride = readResRef(cursor)?;
		let scriptGeneral = readResRef(cursor)?;
		let scriptClass = readResRef(cursor)?;
		let scriptRace = readResRef(cursor)?;
		let scriptDefault = readResRef(cursor)?;
		let scriptSpecific = readResRef(cursor)?;
		let cre = readResRef(cursor)?;
		let creAddress = SectionAddress::<u32, u32>::fromCursor(cursor)?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			current,
			destination,
			flags,
			randomMonster,
			creFirstLetter,
			animation,
			orientation,
			removalTimer,
			movementRestrictionDistance,
			movementRestrictionDistance2,
			appearanceSchedule,
			conversedCount,
			dialog,
			scriptOverride,
			scriptGeneral,
			scriptClass,
			scriptRace,
			scriptDefault,
			scriptSpecific,
			cre,
			creAddress,
		});
	}
}
