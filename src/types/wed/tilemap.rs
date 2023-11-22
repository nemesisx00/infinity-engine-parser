#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::types::Readable;

/**
The contents of WED Tilemap structures.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

For each tile cell in an overlay, there is one tilemap structure. These
structures tell us which tile(s) from the tileset resource are to be used for
the given tile cell. Each tile cell must have one tileset resource. Those which
are referenced by the door tile cells of a door must have 2: one for the open
state and one for the closed state. If a tile cell is animated it will use a
range of tile indices from the tile indices lookup table.

---

Offset | Size | Description
---|---|---
0x0000 | 2 | Start index in tile index lookup table of primary (default) tile
0x0002 | 2 | Count of tiles in tile index lookup table for primary (default) tile
0x0004 | 2 | Index from TIS file of secondary (alternate) tile (i.e. tile for closed state, if tile has an open/closed state) and also for overlays indication (tiles with marked overlay area, by "green" color)
0x0006 | 1 | Overlay mask
0x0007 | 3 | Unknown

### Overlay mask

- bit 0: Unused
- bit 1: Draw overlay 1
- bit 2: Draw overlay 2
- bit 3: Draw overlay 3
- bit 4: Draw overlay 4
- bit 5: Draw overlay 5
- bit 6: Draw overlay 6
- bit 7: Draw overlay 7
*/
#[derive(Clone, Copy, Debug, Default)]
pub struct Tilemap
{
	pub start: u16,
	pub count: u16,
	pub secondary: u16,
	pub mask: u8,
	pub unknown: [u8; 3],
}

impl Tilemap
{
	const UnknownSize: usize = 3;
}

impl Readable for Tilemap
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let start = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 start")?;
		let count = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 count")?;
		let secondary = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 secondary")?;
		let mask = cursor.read_u8()
			.context("Failed to read u8 mask")?;
		
		let mut unknown = [0; Self::UnknownSize];
		cursor.read_exact(&mut unknown)
			.context("Failed to read [u8; 3] unknown")?;
		
		return Ok(Self
		{
			start,
			count,
			secondary,
			mask,
			unknown,
		});
	}
}
