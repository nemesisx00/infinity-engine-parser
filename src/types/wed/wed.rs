#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::types::{InfinityEngineType, Readable};
use super::{Door, SecondaryHeader, Overlay, Polygon, Tilemap, WallGroup, WedHeader};

/**
The fully parsed contents of a WED file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

This file format maps the layout of terrain to the tiles in the tileset, and
adds structure to an area by listing its doors and walls.

---

### Header Data

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('WED ')
0x0004 | 4 | Version ('V1.3')
0x0008 | 4 | Number of overlays (including the base layer)
0x000c | 4 | Number of doors
0x0010 | 4 | Offset (from start of file) to overlays
0x0014 | 4 | Offset (from start of file) to secodary header
0x0018 | 4 | Offset (from start of file) to doors
0x001c | 4 | Offset (from start of file) to door tile cell indices
*/
#[derive(Clone, Debug, Default)]
pub struct Wed
{
	pub header: WedHeader,
	pub overlays: Vec<Overlay>,
	pub secondaryHeader: SecondaryHeader,
	pub doors: Vec<Door>,
	pub tilemaps: HashMap<String, Vec<Tilemap>>,
	pub doorTileCellIndices: Vec<u32>,
	pub tileIndexLookup: Vec<u16>,
	pub wallGroups: Vec<WallGroup>,
	pub polygons: Vec<Polygon>,
	pub polygonIndexLookup: Vec<u16>,
}

impl Wed
{
	const Signature: &str = "WED ";
	const Version: &str = "V1.3";
	
	/**
	One thing worth remembering is that one wall group has the following
	dimensions: `10 tiles * 7.5 tiles`
	
	Thus the number of wall groups contained within an area can be calculated
	based upon the area's dimensions.
	
	For example, an area with dimensions 80x60 tiles should have 64 wall groups.
	*/
	const WallGroupSize: u32 = 75;
}

impl InfinityEngineType for Wed {}

impl Readable for Wed
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let header = WedHeader::fromCursor(cursor)
			.context("Failed to read WedHeader header")?;
		
		let mut overlays = vec![];
		for i in 0..header.overlayCount
		{
			let overlay = Overlay::fromCursor(cursor)
				.context(format!("Failed to read Overlay index {}", i))?;
			overlays.push(overlay);
		}
		
		let secondaryHeader = SecondaryHeader::fromCursor(cursor)?;
		
		let mut doors = vec![];
		for i in 0..header.doorCount
		{
			let door = Door::fromCursor(cursor)
				.context(format!("Failed to read Door index {}", i))?;
			doors.push(door);
		}
		
		let mut tilemaps = HashMap::<String, Vec<Tilemap>>::default();
		for entry in &overlays
		{
			let tileCount = match &entry.tis
			{
				Some(tis) => tis.tileCount,
				None => 0,
			};
			
			let mut tilesRead = 0;
			let mut instances = vec![];
			while tilesRead < tileCount
			{
				let tilemap = Tilemap::fromCursor(cursor)
					.context(format!("Failed to read Tilemap after reading {} tiles", tilesRead))?;
				tilesRead += tilemap.count as u32;
				instances.push(tilemap);
			}
			
			if !instances.is_empty()
			{
				tilemaps.insert(entry.name.to_owned(), instances);
			}
		}
		
		let mut doorTileCellIndices = vec![];
		cursor.set_position(header.doorTileOffset as u64);
		for i in 0..header.doorCount
		{
			let index = cursor.read_u32::<LittleEndian>()
				.context(format!("Failed to read u32 doorTileOffset index {}", i))?;
			doorTileCellIndices.push(index);
		}
		
		let lookupTableSize = tilemaps.iter().fold(0, |acc, (_, list)| acc + list.len());
		let mut tileIndexLookup = vec![];
		for i in 0..lookupTableSize
		{
			let index = cursor.read_u16::<LittleEndian>()
				.context(format!("Failed to read u16 tileIndexLookup index {}", i))?;
			tileIndexLookup.push(index);
		}
		
		let wallGroupsSize = tilemaps[&overlays[0].name].len() as u32 / Self::WallGroupSize;
		let mut wallGroups = vec![];
		cursor.set_position(secondaryHeader.wallGroupsOffset as u64);
		for i in 0..wallGroupsSize
		{
			let wallGroup = WallGroup::fromCursor(cursor)
				.context(format!("Failed to read WallGroup index {}", i))?;
			wallGroups.push(wallGroup);
		}
		
		let mut polygons = vec![];
		cursor.set_position(secondaryHeader.polygonOffset as u64);
		for i in 0..secondaryHeader.polygonCount
		{
			let polygon = Polygon::fromCursor(cursor)
				.context(format!("Failed to read Polygon index {}", i))?;
			polygons.push(polygon);
		}
		
		let mut polygonIndexLookup = vec![];
		for i in 0..secondaryHeader.polygonCount
		{
			let idx = cursor.read_u16::<LittleEndian>()
				.context(format!("Failed to read u16 idx index {}", i))?;
			polygonIndexLookup.push(idx);
		}
		
		return Ok(Self
		{
			header,
			overlays,
			secondaryHeader,
			doors,
			tilemaps,
			doorTileCellIndices,
			tileIndexLookup,
			wallGroups,
			polygons,
			polygonIndexLookup,
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
	//use image::ImageOutputFormat;
	#[allow(unused_imports)]
	use super::*;
	use crate::platform::Games;
	use crate::resource::ResourceManager;
	use crate::types::ResourceType_WED;
use crate::types::util::BoundingBox; //{ResourceType_WEB, Bmp};
	
    #[test]
    fn TestWed()
	{
		let game = Games::BaldursGate1;
		let name = "AR2600";
		
		let expectedDoors = vec![
			("DOOR2616", 1, 0),
			("DOOR2618", 1, 5),
			("DOOR2607", 1, 9),
			("DOOR2608", 1, 15),
			("DOOR2602", 1, 19),
			("DOOR2606", 1, 24),
		];
		
		let expectedOverlays = vec![
			("AR2600", 80, 60, true, true, 4803, 576),
			("WTWAVE", 1, 1, true, true, 1, 2984),
			("WTPOOL", 1, 1, true, true, 1, 3311),
			("", 0, 0, false, false, 0, 0),
			("", 0, 0, false, false, 0, 0),
		];
		
		let expectedWallGroups = vec![
			WallGroup { start: 0, count: 27 },
			WallGroup { start: 1247, count: 1 },
		];
		
		let expectedPolygonCount = 957;
		
		let expectedPolygons = vec![
			// First Read
			Polygon
			{
				start: 0,
				count: 16,
				mask: 1,
				height: 255,
				boundingBox: BoundingBox
				{
					left: 1116,
					right: 1272,
					top: 336,
					bottom: 411,
				},
			},
			
			// Last Read
			Polygon
			{
				start: 11212,
				count: 4,
				mask: 1,
				height: 255,
				boundingBox: BoundingBox
				{
					left: 4523,
					right: 4620,
					top: 2046,
					bottom: 2452,
				}
			},
		];
		
		let resourceManager = ResourceManager::default();
		let result = resourceManager.loadResource::<Wed>(game, ResourceType_WED, name.to_owned()).unwrap();
		
		assert_eq!(Wed::Signature, result.header.identity.signature);
		assert_eq!(Wed::Version, result.header.identity.version);
		assert_eq!(expectedDoors.len(), result.header.doorCount as usize);
		assert_eq!(result.header.doorCount as usize, result.doors.len());
		assert_eq!(expectedOverlays.len(), result.header.overlayCount as usize);
		assert_eq!(result.header.overlayCount as usize, result.overlays.len());
		assert_eq!(expectedPolygonCount, result.secondaryHeader.polygonCount);
		assert_eq!(result.secondaryHeader.polygonCount as usize, result.polygons.len());
		
		for i in 0..expectedDoors.len()
		{
			let (name, openClosed, index) = expectedDoors[i];
			assert_eq!(name, result.doors[i].name);
			assert_eq!(openClosed, result.doors[i].openClosed);
			assert_eq!(index, result.doors[i].firstDoorIndex);
		}
		
		for i in 0..expectedOverlays.len()
		{
			let (name, width, height, isSome, hasTilemap, tilemapLength, lastStartIndex) = expectedOverlays[i];
			assert_eq!(name, result.overlays[i].name);
			assert_eq!(width, result.overlays[i].width);
			assert_eq!(height, result.overlays[i].height);
			assert_eq!(isSome, result.overlays[i].tis.is_some());
			assert_eq!(hasTilemap, result.tilemaps.contains_key(&result.overlays[i].name));
			
			if hasTilemap
			{
				assert!(!result.tilemaps[&result.overlays[i].name].is_empty());
				assert_eq!(tilemapLength, result.tilemaps[&result.overlays[i].name].len());
				assert_eq!(lastStartIndex, result.tilemaps[&result.overlays[i].name].last().unwrap().start);
			}
		}
		
		assert_eq!(expectedDoors.len(), result.doorTileCellIndices.len());
		assert_eq!(expectedOverlays.iter().fold(0, |acc, (_, _, _, _, _, count, _)| acc + count), result.tileIndexLookup.len());
		
		assert_eq!(expectedWallGroups.first(), result.wallGroups.first());
		assert_eq!(expectedWallGroups.last(), result.wallGroups.last());
		
		assert_eq!(expectedPolygons.first(), result.polygons.first());
		assert_eq!(expectedPolygons.last(), result.polygons.last());
		
		//Verify with eyes
		/*
		let tis = result.overlays[0].clone().tis.unwrap();
		let firstTilemap = result.tilemaps[&result.overlays[0].name][0];
		let tileLookup = firstTilemap.start;
		let tileIndex = result.tileIndexLookupTable[tileLookup as usize];
		let tile = tis.tiles[tileIndex as usize].clone();
		
		let bytes = tile.toBytes();
		let adhocBmp = Bmp::adhoc(64, 64, bytes, None);
		let imageBytes = adhocBmp.toImageBytes(Some(ImageOutputFormat::Png)).expect("Failed to generate image bytes");
		
		let outPath = Path::new("target").join(format!("testoutput_{}.png", name));
		let mut file = File::create(outPath.as_path())
			.expect("Output file couldn't be created");
		let result = file.write_all(&imageBytes);
		assert!(result.is_ok());
		// */
	}
}
