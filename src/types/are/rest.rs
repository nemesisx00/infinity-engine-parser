#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::parseString;
use crate::bytes::{readName, readResRef};
use crate::types::util::Readable;

/**
The fully parsed contents of the Rest Interruptions in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 10 | Interruption explanation text
0x002a | 10 | Interruption explanation text
0x0034 | 10 | Interruption explanation text
0x003e | 10 | Interruption explanation text
0x0048 | 8 | Resref of creature to spawn
0x0050 | 8 | Resref of creature to spawn
0x0058 | 8 | Resref of creature to spawn
0x0060 | 8 | Resref of creature to spawn
0x0068 | 8 | Resref of creature to spawn
0x0070 | 8 | Resref of creature to spawn
0x0078 | 8 | Resref of creature to spawn
0x0080 | 8 | Resref of creature to spawn
0x0088 | 8 | Resref of creature to spawn
0x0090 | 8 | Resref of creature to spawn
0x0098 | 2 | Count of creatures in spawn table
0x009a | 2 | Difficulty. Spawn if Party Level * Difficulty > Total Creature Power
0x009c | 4 | Removal time
0x00a0 | 2 | Movement restriction distance
0x00a2 | 2 | Movement restriction distance (move to object)
0x00a4 | 2 | Maximum number of creatures to spawn
0x00a6 | 2 | Is interruption point enabled? 0: No / 1: Yes
0x00a8 | 2 | Probability per hour (day)
0x00aa | 2 | Probability per hour (night)
*/
#[derive(Clone, Debug, Default)]
pub struct AreRestInterruptions
{
	pub name: String,
	pub text: Vec<String>,
	pub creatures: Vec<String>,
	pub creatureCount: u16,
	pub difficulty: u16,
	pub removalTime: u32,
	pub movementRestriction: u16,
	pub movementRestrictionObject: u16,
	pub creatureMax: u16,
	pub enabled: u16,
	pub probabilityDay: u16,
	pub probabilityNight: u16,
}

impl AreRestInterruptions
{
	const CreatureRefMax: usize = 10;
	const LineLength: usize = 10;
	const TextLines: usize = 4;
	const UnusedPadding: u64 = 56;
}

impl Readable for AreRestInterruptions
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readName(cursor)?;
		
		let mut text = vec![];
		for _ in 0..Self::TextLines
		{
			let mut lineBytes: [u8; Self::LineLength] = [0; Self::LineLength];
			cursor.read_exact(&mut lineBytes)?;
			let line = parseString!(lineBytes);
			text.push(line);
		}
		
		let mut creatures = vec![];
		for _ in 0..Self::CreatureRefMax
		{
			let resref = readResRef(cursor)?;
			creatures.push(resref);
		}
		
		let creatureCount = cursor.read_u16::<LittleEndian>()?;
		let difficulty = cursor.read_u16::<LittleEndian>()?;
		let removalTime = cursor.read_u32::<LittleEndian>()?;
		let movementRestriction = cursor.read_u16::<LittleEndian>()?;
		let movementRestrictionObject = cursor.read_u16::<LittleEndian>()?;
		let creatureMax = cursor.read_u16::<LittleEndian>()?;
		let enabled = cursor.read_u16::<LittleEndian>()?;
		let probabilityDay = cursor.read_u16::<LittleEndian>()?;
		let probabilityNight = cursor.read_u16::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			text,
			creatures,
			creatureCount,
			difficulty,
			removalTime,
			movementRestriction,
			movementRestrictionObject,
			creatureMax,
			enabled,
			probabilityDay,
			probabilityNight,
		});
	}
}
