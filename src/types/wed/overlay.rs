#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::getManager;
use crate::platform::Games;
use crate::types::{Readable, Tis};

/**
The contents of a single WED Overlay entry.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

Each overlay is mapped to a tileset. The sections of the area each overlay will
cover is controlled by the tilemap section.

---

Offset | Size | Description
---|---|---
0x0000 | 2 | Width (in tiles)
0x0002 | 2 | Height (in tiles)
0x0004 | 8 | Name of tileset
0x000c | 2 | Unique tile count
0x000e | 2 | Movement type
0x0010 | 4 | Offset to tilemap for this overlay
0x0014 | 4 | Offset to tile index lookup for this overlay
*/
#[derive(Clone, Debug)]
pub struct Overlay
{
	pub width: u16,
	pub height: u16,
	pub name: String,
	pub uniqueTileCount: u16,
	pub movementType: u16,
	pub tilemapOffset: u32,
	pub lookupOffset: u32,
	pub tis: Option<Tis>,
}

impl Readable for Overlay
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let width = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 width")?;
		let height = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 height")?;
		let name = readResRef(cursor)
			.context("Failed to read RESREF name")?;
		let uniqueTileCount = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 uniqueTileCount")?;
		let movementType = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 movementType")?;
		let tilemapOffset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read u32 tilemapOffset")?;
		let lookupOffset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read u32 lookupOffset")?;
		
		let mut tis = None;
		if let Ok(resourceManager) = getManager().lock()
		{
			tis = resourceManager.loadTileset(Games::BaldursGate1, name.to_owned());
		}
		
		return Ok(Self
		{
			width,
			height,
			name,
			uniqueTileCount,
			lookupOffset,
			movementType,
			tilemapOffset,
			tis,
		});
	}
}
