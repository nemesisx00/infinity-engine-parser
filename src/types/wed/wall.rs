#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::types::Readable;

/**
A polygon identifying when a creature is "behind" a wall.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

Outlines of objects, specifically walls and doors, need to be stored in order
to determine when to dither creature animations, to represent moving "behind" a
wall. Outlines are stored as a series of ordered vertices, creating a polygon.
Doors can be represented by more than one polygon in either their open or closed
state. This allows for double doors. These wall groups are sets of indices in
the polygon indices lookup table, which in turn points into the polygon table.

---

Offset | Size | Description
---|---|---
0x0000 | 2 | Starting polygon index
0x0002 | 2 | Count of polygon indices
*/
#[derive(Clone, Copy, Debug, Default)]
pub struct WallGroup
{
	pub start: u16,
	pub count: u16,
}

impl Readable for WallGroup
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let start = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 start")?;
		let count = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 count")?;
		
		return Ok(Self
		{
			start,
			count,
		});
	}
}
