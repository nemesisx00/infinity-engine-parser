use std::io::{Cursor, Read};
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use super::{Identity, Readable, ReadIntoSelf, util::Color};

/**
The fully parsed contents of a TIS file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tis_v1.htm

This file format describes a **Parlette-based*** tileset.

TIS files are generally comprised of a large number of tiles, each of which
consists of a palette and a rectangular block of pixels. Each pixel is an index
into the asociated palette. Each tile has its own palette and a block of pixels.
The pixel data is not compressed.

The TIS file contains only the graphics for an area. The location information is
stored in a WED file.

---

### Header Data

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('TIS ')
0x0004 | 4 | Version ('V1  ')
0x0008 | 4 | Count of tiles within this tileset
0x000c | 4 | Length of a tile data block
0x0010 | 4 | Size of the header (offset to tiles)
0x0014 | 4 | Dimension of 1 tile in pixels (64x64)
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tis
{
	pub identity: Identity,
	pub tileCount: u32,
	pub tileLength: u32,
	pub headerSize: u32,
	pub tileSize: u32,
	pub tiles: Vec<TisTileData>,
}

impl Tis
{
	const Signature: &'static str = "TIS ";
	const Version: &'static str = "V1  ";
	
	/**
	A palette-based TIS tile palette always has 256 32-bit colors.
	
	256 * 4 = 1024 bytes
	*/
	pub const PaletteSize: usize = 256;
	
	/**
	A palette-based TIS tile is always sized 64x64.
	
	64 * 64 = 4096 bytes
	*/
	pub const TileLength: u32 = 4096;
	
	/**
	Each pixel is an index mapping to a u32 color value.
	*/
	pub const ColorLength: u32 = 4;
	
	const HeaderSize: u32 = 24;
	pub const TileSize: u32 = 64;
	
	pub fn new(count: u32) -> Self
	{
		return Self
		{
			tileCount: count,
			..Default::default()	
		};
	}
}

impl ReadIntoSelf for Tis
{
	fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<()>
	{
		let mut tiles = vec![];
		
		for i in 0..self.tileCount
		{
			let tile = TisTileData::fromCursor(cursor)
				.context(format!("Error reading TisTileData for tile index {}", i))?;
			tiles.push(tile);
		}
		
		self.tiles = tiles;
		
		return Ok(());
	}
}

impl Default for Tis
{
	fn default() -> Self
	{
		return Self
		{
			identity: Identity { signature: Self::Signature.to_owned(), version: Self::Version.to_owned() },
			tileCount: 0,
			tileLength: Self::TileLength,
			headerSize: Self::HeaderSize,
			tileSize: Self::TileSize,
			tiles: vec![],
		};
	}
}

// --------------------------------------------------

/**
The palette and pixel data of a single palette-based tile.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tis_v1.htm

---

Each tile data block is 5120 bytes long.
	- Palette (256 * 4) + data (64 * 64)

## Color Notes

Each entry in the color palette is a 32-bit value in BGRA order. The alpha
value is unused.

The pixel data are 8-bit indices from the color palette.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TisTileData
{
	pub colors: Vec<Color>,
	pub palette: Vec<u32>,
	pub pixels: Vec<u8>,
}

impl TisTileData
{
	pub fn toBytes(&self) -> Vec<u8>
	{
		let chromaKey = self.colors[0];
		let mut colors = vec![];
		for pixel in self.pixels.iter()
		{
			let b = match self.colors.get(*pixel as usize)
			{
				None => chromaKey.bytes(),
				Some(color) => color.bytes(),
			};
			colors.push(b);
		}
		
		let bytes = colors.concat();
		return bytes;
	}
}

impl Readable for TisTileData
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let mut colors = vec![];
		let mut palette = vec![];
		
		for i in 0..Tis::PaletteSize
		{
			let value = cursor.read_u32::<LittleEndian>()
				.context(format!("Failed reading Tis tile palette value index {}", i))?;
			palette.push(value);
			//The palette contains colors in BGRA order
			colors.push(Color::fromBGRA(value));
		}
		
		let mut pixels = [0; Tis::TileLength as usize];
		cursor.read_exact(&mut pixels)
			.context(format!("Failed reading Tis tile data at position {}", cursor.position()))?;
		
		return Ok(Self
		{
			colors,
			palette,
			pixels: pixels.into(),
		});
	}
}

#[cfg(test)]
mod tests
{
	#[allow(unused_imports)]
	use std::fs::File;
	#[allow(unused_imports)]
	use std::io::Write;
	#[allow(unused_imports)]
	use std::path::Path;
	#[allow(unused_imports)]
	use image::io::Reader as ImageReader;
	#[allow(unused_imports)]
	use super::*;
	use crate::platform::Games;
	use crate::resource::ResourceManager;
	use crate::test::updateResourceManager;
	
    #[test]
    fn TestTis()
	{
		let game = Games::BaldursGate1;
		let name = "AR2600".to_string();
		
		let resourceManager = ResourceManager::default();
		let _ = updateResourceManager(&resourceManager);
		
		let result = resourceManager.loadTileset(game, name.to_owned()).unwrap();
		
		assert_ne!(0, result.tileCount);
		assert_eq!(result.tileCount as usize, result.tiles.len());
		
		for tile in &result.tiles
		{
			assert!(!tile.colors.is_empty());
			assert!(!tile.palette.is_empty());
			assert!(!tile.pixels.is_empty());
			
			assert_eq!(tile.palette.len(), tile.colors.len());
			for i in 0..tile.palette.len()
			{
				assert_eq!(tile.palette[i], tile.colors[i].intoBGRA());
			}
		}
	}
}
