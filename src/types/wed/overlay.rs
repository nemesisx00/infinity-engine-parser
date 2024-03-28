use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::getManager;
use crate::platform::Games;
use crate::types::{Readable, Tis};
use super::Tilemap;

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
	pub tilesetName: String,
	pub uniqueTileCount: u16,
	pub movementType: u16,
	pub tilemapOffset: u32,
	pub tileIndexLookupOffset: u32,
	pub tileIndexLookup: Vec<u16>,
	pub tilemaps: Vec<Tilemap>,
	pub tis: Option<Tis>,
}

impl Overlay
{
	/**
	Calculate the total size, in bytes, of this overlay's tile data.
	*/
	pub fn size(&self) -> u32
	{
		return (self.width * self.height) as u32
			* Tis::TileLength
			* Tis::ColorLength;
	}
	
	/**
	Collect the tile data together into a single array of bytes.
	
	---
	
	An area is a grid, with each 64*64 cell within the grid (called a tile cell)
	being a location for a tile. Tile cells are numbered, starting at 0, and run
	from top left to bottom right (i.e. a tile cell number can be calculated by
	y*width+x). As well the tiles for the main area graphics, an area can use
	overlays. Overlays are usually used for rivers and lakes. Each overlay layer
	is placed in a separate grid, which are stacked on top of the base grid.
	Areas also contain another grid, split into 16*16 squares, for the
	exploration map.
	
	The process of drawing an area is outlined below:
	
	- The cell number acts as an index into a tilemap structure.
	- This give a "tile lookup index" which is an index into the tile indices lookup table.
	- The tile indices lookup table gives the index into the actual tileset, at which point, the tile is drawn.
	- The process is repeated for each required overlay (using the associated overlay tilemap / tile indices).
	*/
	pub fn getTileBytes(&self) -> Vec<u8>
	{
		let mut tiles = vec![];
		for y in 0..self.height
		{
			for x in 0..self.width
			{
				let cellId = ((y * (self.width - 1)) + x) as usize;
				if let Some(tis) = &self.tis
				{
					if let Some(tileIndex) = self.tileIndexLookup.get(cellId.clone())
					{
						if let Some(tile) = tis.tiles.get(*tileIndex as usize)
						{
							let tileBytes = tile.toBytes();
							tiles.push(tileBytes);
						}
					}
				}
			}
		}
		
		let bytes = tiles.concat();
		return bytes;
	}
}

impl Readable for Overlay
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let width = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 width")?;
		let height = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 height")?;
		let tilesetName = readResRef(cursor)
			.context("Failed to read RESREF name")?;
		let uniqueTileCount = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 uniqueTileCount")?;
		let movementType = cursor.read_u16::<LittleEndian>()
			.context("Failed to read u16 movementType")?;
		let tilemapOffset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read u32 tilemapOffset")?;
		let tileIndexLookupOffset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read u32 lookupOffset")?;
		
		let mut tis = None;
		if let Ok(resourceManager) = getManager().lock()
		{
			tis = resourceManager.loadTileset(Games::BaldursGate1, tilesetName.to_owned());
		}
		
		let mut tilemaps = vec![];
		let mut tileIndexLookup = vec![];
		
		if let Some(tis) = &tis
		{
			let position = cursor.position();
			
			cursor.set_position(tilemapOffset as u64);
			let mut tilesRead = 0;
			let mut instances = vec![];
			while tilesRead < tis.tileCount
			{
				let tilemap = Tilemap::fromCursor(cursor)
					.context(format!("Failed to read Tilemap after reading {} tiles", tilesRead))?;
				tilesRead += tilemap.count as u32;
				instances.push(tilemap);
			}
			
			if !instances.is_empty()
			{
				tilemaps = instances;
			}
			
			cursor.set_position(tileIndexLookupOffset as u64);
			for i in 0..tilemaps.len()
			{
				let index = cursor.read_u16::<LittleEndian>()
					.context(format!("Failed to read u16 tileIndexLookup index {}", i))?;
				tileIndexLookup.push(index);
			}
			
			cursor.set_position(position);
		}
		
		return Ok(Self
		{
			width,
			height,
			tilesetName,
			uniqueTileCount,
			movementType,
			tilemapOffset,
			tileIndexLookupOffset,
			tileIndexLookup,
			tilemaps,
			tis,
		});
	}
}
