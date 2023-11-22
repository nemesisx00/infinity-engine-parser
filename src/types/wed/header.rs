#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::types::{Identity, Readable};

#[derive(Clone, Debug, Default)]
pub struct WedHeader
{
	pub identity: Identity,
	pub overlayCount: u32,
	pub doorCount: u32,
	pub overlayOffset: u32,
	pub headerOffset: u32,
	pub doorOffset: u32,
	pub doorTileOffset: u32,
}

impl Readable for WedHeader
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let identity = Identity::fromCursor(cursor)
			.context("Error parsing WED Identity")?;
		let overlayCount = cursor.read_u32::<LittleEndian>()?;
		let doorCount = cursor.read_u32::<LittleEndian>()?;
		let overlayOffset = cursor.read_u32::<LittleEndian>()?;
		let headerOffset = cursor.read_u32::<LittleEndian>()?;
		let doorOffset = cursor.read_u32::<LittleEndian>()?;
		let doorTileOffset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			identity,
			overlayCount,
			doorCount,
			overlayOffset,
			headerOffset,
			doorOffset,
			doorTileOffset,
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
	pub polygonLookupOffset: u32,
}

impl Readable for SecondaryHeader
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let polygonCount = cursor.read_u32::<LittleEndian>()?;
		let polygonOffset = cursor.read_u32::<LittleEndian>()?;
		let verticesOffset = cursor.read_u32::<LittleEndian>()?;
		let wallGroupsOffset = cursor.read_u32::<LittleEndian>()?;
		let polygonLookupOffset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			polygonCount,
			polygonOffset,
			verticesOffset,
			wallGroupsOffset,
			polygonLookupOffset,
		});
	}
}
