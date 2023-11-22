#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::{BoundingBox, Readable, Point2D};

/**
The fully parsed contents of a Container in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | Current X coordinate
0x0022 | 2 | Current Y coordinate
0x0024 | 2 | Container type
0x0026 | 2 | Lock difficulty
0x0028 | 4 | Flags
0x002c | 2 | Trap detection difficulty
0x002e | 2 | Trap removal difficulty
0x0030 | 2 | Is container trapped? 0: No / 1: Yes
0x0032 | 2 | Is trap detected? 0: No / 1: Yes
0x0034 | 2 | Trap launch X coordinate
0x0036 | 2 | Trap launch Y coordinate
0x0038 | 8 | Bounding box of container polygon
0x0040 | 4 | Index of first item in this container
0x0044 | 4 | Count of items in this container
0x0048 | 8 | Trap script
0x0050 | 4 | Index of first vertex making up the outline of this container
0x0054 | 2 | Count of vertices making up the outline of this container
0x0056 | 2 | Trigger range
0x0058 | 32 | Owner (script name)
0x0078 | 8 | Key item
0x0080 | 4 | Break difficulty
0x0084 | 4 | Lockpick string
*/
#[derive(Clone, Debug, Default)]
pub struct AreContainer
{
	pub name: String,
	pub coordinates: Point2D<u16>,
	pub containerType: u16,
	pub lockDifficulty: u16,
	pub flags: u32,
	pub trapDetectionDifficulty: u16,
	pub trapRemovalDifficulty: u16,
	pub trapped: u16,
	pub trapDetected: u16,
	pub trapLaunchCoordinates: Point2D<u16>,
	pub boundingBox: BoundingBox,
	pub firstItemIndex: u32,
	pub itemCount: u32,
	pub trapScript: String,
	pub firstVertexIndex: u32,
	pub vertexCount: u16,
	pub triggerRange: u16,
	pub owner: String,
	pub keyItem: String,
	pub breakDifficulty: u32,
	pub lockpickStringIndex: u32,
}

impl AreContainer
{
	const UnusedPadding: u64 = 56;
}

impl Readable for AreContainer
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readResRef(cursor)?;
		let coordinates = Point2D::<u16>::fromCursor(cursor)?;
		let containerType = cursor.read_u16::<LittleEndian>()?;
		let lockDifficulty = cursor.read_u16::<LittleEndian>()?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		let trapDetectionDifficulty = cursor.read_u16::<LittleEndian>()?;
		let trapRemovalDifficulty = cursor.read_u16::<LittleEndian>()?;
		let trapped = cursor.read_u16::<LittleEndian>()?;
		let trapDetected = cursor.read_u16::<LittleEndian>()?;
		let trapLaunchCoordinates = Point2D::<u16>::fromCursor(cursor)?;
		let boundingBox = BoundingBox::fromCursor(cursor)?;
		let firstItemIndex = cursor.read_u32::<LittleEndian>()?;
		let itemCount = cursor.read_u32::<LittleEndian>()?;
		let trapScript = readResRef(cursor)?;
		let firstVertexIndex = cursor.read_u32::<LittleEndian>()?;
		let vertexCount = cursor.read_u16::<LittleEndian>()?;
		let triggerRange = cursor.read_u16::<LittleEndian>()?;
		let owner = readName(cursor)?;
		let keyItem = readResRef(cursor)?;
		let breakDifficulty = cursor.read_u32::<LittleEndian>()?;
		let lockpickStringIndex = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			coordinates,
			containerType,
			lockDifficulty,
			flags,
			trapDetectionDifficulty,
			trapRemovalDifficulty,
			trapped,
			trapDetected,
			trapLaunchCoordinates,
			boundingBox,
			firstItemIndex,
			itemCount,
			trapScript,
			firstVertexIndex,
			vertexCount,
			triggerRange,
			owner,
			keyItem,
			breakDifficulty,
			lockpickStringIndex,
		});
	}
}
