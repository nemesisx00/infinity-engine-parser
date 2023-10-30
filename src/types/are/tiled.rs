#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::Readable;

/**
The fully parsed contents of a Tiled Object in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 8 | Tile Id
0x0028 | 4 | Flags
0x0030 | 4 | Offset to open search squares
0x0034 | 4 | Count of open search squares
0x0038 | 4 | Count of closed search squares
0x003c | 4 | Offset to closed search squares
*/
#[derive(Clone, Debug, Default)]
pub struct AreTiledObject
{
	pub name: String,
	pub tileId: String,
	pub flags: u32,
	pub openOffset: u32,
	pub openCount: u32,
	pub closedCount: u32,
	pub closedOffset: u32,
}

impl AreTiledObject
{
	const UnusedPadding: u64 = 48;
}

impl Readable for AreTiledObject
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readName(cursor)?;
		let tileId = readResRef(cursor)?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		let openOffset = cursor.read_u32::<LittleEndian>()?;
		let openCount = cursor.read_u32::<LittleEndian>()?;
		let closedCount = cursor.read_u32::<LittleEndian>()?;
		let closedOffset = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			tileId,
			flags,
			openOffset,
			openCount,
			closedCount,
			closedOffset,
		});
	}
}
