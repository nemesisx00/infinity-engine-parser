#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::{Cursor, Read};
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::{readString, getManager};
use crate::platform::Games;
use super::util::TypeSize_RESREF;
use super::{Identity, InfinityEngineType, Tis, ResourceType_TIS};

const Signature: &str = "WED ";
const Version: &str = "V1.3";

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
	pub identity: Identity,
	pub overlayCount: u32,
	pub doorCount: u32,
	pub overlayOffset: u32,
	pub headerOffset: u32,
	pub doorOffset: u32,
	pub doorTileOffset: u32,
	pub overlays: Vec<Overlay>,
	pub secondaryHeader: SecondaryHeader,
	pub doors: Vec<Door>,
	pub tilemaps: HashMap<String, Vec<Tilemap>>,
	pub doorTileCellIndices: Vec<u32>,
	pub tileIndexLookupTable: Vec<u16>,
	pub wallGroups: Vec<WallGroup>,
	pub polygons: Vec<Polygon>,
	pub polygonIndexLookup: Vec<usize>,
}

impl InfinityEngineType for Wed
{
	type Output = Wed;
	
	fn fromCursor<T>(cursor: &mut Cursor<Vec<u8>>) -> Result<Self::Output>
		where T: InfinityEngineType
	{
		let identity = Identity::fromCursor(cursor)?;
		let overlayCount = cursor.read_u32::<LittleEndian>()?;
		let doorCount = cursor.read_u32::<LittleEndian>()?;
		let overlayOffset = cursor.read_u32::<LittleEndian>()?;
		let headerOffset = cursor.read_u32::<LittleEndian>()?;
		let doorOffset = cursor.read_u32::<LittleEndian>()?;
		let doorTileOffset = cursor.read_u32::<LittleEndian>()?;
		
		let mut overlays = vec![];
		for _ in 0..overlayCount
		{
			let overlay = Overlay::fromCursor(cursor)?;
			overlays.push(overlay);
		}
		
		let secondaryHeader = SecondaryHeader::fromCursor(cursor)?;
		
		let mut doors = vec![];
		for _ in 0..doorCount
		{
			let door = Door::fromCursor(cursor)?;
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
				let tilemap = Tilemap::fromCursor(cursor)?;
				tilesRead += tilemap.count as u32;
				instances.push(tilemap);
			}
			
			if !instances.is_empty()
			{
				tilemaps.insert(entry.name.to_owned(), instances);
			}
		}
		
		let mut doorTileCellIndices = vec![];
		cursor.set_position(doorTileOffset as u64);
		for _ in 0..doorCount
		{
			let index = cursor.read_u32::<LittleEndian>()?;
			doorTileCellIndices.push(index);
		}
		
		let lookupTableSize = tilemaps.iter().fold(0, |acc, (_, list)| acc + list.len());
		let mut tileIndexLookupTable = vec![];
		for _ in 0..lookupTableSize
		{
			let index = cursor.read_u16::<LittleEndian>()?;
			tileIndexLookupTable.push(index);
		}
		
		return Ok(Self
		{
			identity,
			overlayCount,
			doorCount,
			overlayOffset,
			headerOffset,
			doorOffset,
			doorTileOffset,
			overlays,
			secondaryHeader,
			doors,
			tilemaps,
			doorTileCellIndices,
			tileIndexLookupTable,
			..Default::default()
		});
	}
}

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

impl Overlay
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let width = cursor.read_u16::<LittleEndian>()?;
		let height = cursor.read_u16::<LittleEndian>()?;
		
		let mut nameBytes = [0; TypeSize_RESREF];
		cursor.read_exact(&mut nameBytes)?;
		let name = readString!(nameBytes);
		
		let uniqueTileCount = cursor.read_u16::<LittleEndian>()?;
		let movementType = cursor.read_u16::<LittleEndian>()?;
		let tilemapOffset = cursor.read_u32::<LittleEndian>()?;
		let lookupOffset = cursor.read_u32::<LittleEndian>()?;
		
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

/**
The contents of a WED Secondary Header.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

The "secondary header" contains more offsets, which would generally be found
within the primary header.

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Number of polygons used to represent walls
0x0004 | 4 | Offset to polygons
0x0008 | 4 | Offset to vertices
0x000c | 4 | Offset to wall groups
0x0010 | 4 | Offset to polygon indices lookup table
*/
#[derive(Clone, Copy, Debug, Default)]
pub struct SecondaryHeader
{
	pub polygonCount: u32,
	pub polygonOffset: u32,
	pub verticesOffset: u32,
	pub wallGroupsOffset: u32,
	pub lookupOffset: u32,
}

impl SecondaryHeader
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let polygonCount = cursor.read_u32::<LittleEndian>()?;
		let polygonOffset = cursor.read_u32::<LittleEndian>()?;
		let verticesOffset = cursor.read_u32::<LittleEndian>()?;
		let wallGroupsOffset = cursor.read_u32::<LittleEndian>()?;
		let lookupOffset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			polygonCount,
			polygonOffset,
			verticesOffset,
			wallGroupsOffset,
			lookupOffset,
		});
	}
}

/**
The contents of WED Doors data.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

This section generally describes doors, though the concept can be extended to
describe any object which acts like a door; i.e. has an open state and a closed
state, and optionally blocks certain cells from being reached. The door tile
cells for a given door are those cells which are impeded, and whose graphics
change when this door is closed. See the door tile cell indices section for
details.

---

Offset | Size | Description
---|---|---
0x0000 | 8 | Name of door
0x0008 | 2 | Open (0) / Closed (1)
0x000a | 2 | First door tile cell index
0x000c | 2 | Count of door tile cells for this door
0x000e | 2 | Count of polygons open state
0x0010 | 2 | Count of polygons closed state
0x0012 | 4 | Offset (from start of file) to polygons open state
0x0016 | 4 | Offset (from start of file) to polygons closed state
*/
#[derive(Clone, Debug, Default)]
pub struct Door
{
	pub name: String,
	pub openClosed: u16,
	pub firstDoorIndex: u16,
	pub tileCellCount: u16,
	pub openCount: u16,
	pub closedCount: u16,
	pub openOffset: u32,
	pub closedOffset: u32,
}

const DoorValue_Open: u16 = 0;

impl Door
{
	pub fn isOpen(&self) -> bool { return self.openClosed == DoorValue_Open; }
	
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let mut nameBytes = [0; TypeSize_RESREF];
		cursor.read_exact(&mut nameBytes)?;
		let name = readString!(nameBytes);
		
		let openClosed = cursor.read_u16::<LittleEndian>()?;
		let firstDoorIndex = cursor.read_u16::<LittleEndian>()?;
		let doorCount = cursor.read_u16::<LittleEndian>()?;
		let openCount = cursor.read_u16::<LittleEndian>()?;
		let closedCount = cursor.read_u16::<LittleEndian>()?;
		let openOffset = cursor.read_u32::<LittleEndian>()?;
		let closedOffset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			name,
			openClosed,
			firstDoorIndex,
			tileCellCount: doorCount,
			openCount,
			closedCount,
			openOffset,
			closedOffset,
		});
	}
}

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

const Tilemap_UnknownSize: usize = 3;

impl Tilemap
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let start = cursor.read_u16::<LittleEndian>()?;
		let count = cursor.read_u16::<LittleEndian>()?;
		let secondary = cursor.read_u16::<LittleEndian>()?;
		let mask = cursor.read_u8()?;
		
		let mut unknown = [0; Tilemap_UnknownSize];
		cursor.read_exact(&mut unknown)?;
		
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
	pub startIndex: u16,
	pub indexCount: u16,
}

impl WallGroup
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let startIndex = cursor.read_u16::<LittleEndian>()?;
		let indexCount = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			startIndex,
			indexCount,
		});
	}
}

/**
A WED polygon.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

A polygon consists of a list of vertices, a minimum bounding box, and some
unknown state information.

The order of vertices can be important when describing polygins:

- For door walls, the vertices have to be listed clockwise, starting with the
	rightmost vertex first. If there are two or more vertices on the same
	rightmost vertical line, the lowest point should be listed first.
- For normal walls, the vertices can be listed in clockwise or
	counter-clockwise order, starting with the lowest most vertex.

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Starting vertex index
0x0004 | 4 | Count of vertices
0x0008 | 1 | Indicates whether this polygon is a passable or not.
0x0009 | 1 | Height
0x000a | 2 | Minimum X coordinate of bounding box
0x000c | 2 | Maximum X coordinate of bounding box
0x000e | 2 | Minimum Y coordinate of bounding box
0x0010 | 2 | Maximum Y coordinate of bounding box

### Polygon mask

- bit 0: Shade wall
- bit 1: Hovering
- bit 2: Cover animations
- bit 3: Cover animations
- bit 4: Unknown
- bit 5: Unknown
- bit 6: Unknown
- bit 7: Door?
*/
#[derive(Clone, Copy, Debug, Default)]
pub struct Polygon
{
	pub start: u32,
	pub count: u32,
	pub mask: u8,
	pub height: u8,
	pub boundMinimum: Vertex,
	pub boundMaximum: Vertex,
}

/**
A point in 2D space used to represent a WED vertex.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

Each component is a **Little Endian** 16-bit integer.
*/
#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex
{
	pub x: i16,
	pub y: i16,
}

impl Vertex
{
	pub fn new(x: i16, y: i16) -> Self
	{
		return Self { x, y };
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
	use crate::types::ResourceType_WED;
	
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
		let expectedPolygonCount = 957;
		
		let resourceManager = ResourceManager::default();
		let result = resourceManager.loadResource::<Wed>(game, ResourceType_WED, name.to_owned()).unwrap();
		
		assert_eq!(Signature, result.identity.signature);
		assert_eq!(Version, result.identity.version);
		assert_eq!(expectedDoors.len(), result.doorCount as usize);
		assert_eq!(result.doorCount as usize, result.doors.len());
		assert_eq!(expectedOverlays.len(), result.overlayCount as usize);
		assert_eq!(result.overlayCount as usize, result.overlays.len());
		assert_eq!(expectedPolygonCount, result.secondaryHeader.polygonCount);
		
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
		assert_eq!(expectedOverlays.iter().fold(0, |acc, (_, _, _, _, _, count, _)| acc + count), result.tileIndexLookupTable.len());
		
		//Verify with eyes
		/*
		let outPath = Path::new("../../target").join(format!("testoutput_{}.png", name));
		let mut file = File::create(outPath.as_path())
			.expect("Output file couldn't be created");
		let bytes = result.toImageBytes(5120, 3904, Some(ImageOutputFormat::Png)).unwrap();
		let result = file.write_all(&bytes);
		assert!(result.is_ok());
		// */
	}
}
