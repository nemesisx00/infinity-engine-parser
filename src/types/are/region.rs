#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::{BoundingBox, Readable, Point2D};

/**
The fully parsed contents of a Region in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | Region type
0x0022 | 8 | Minimum bounding box of this point
0x002a | 2 | Count of vertices composing the perimeter of this region
0x002c | 4 | Index of first vertex for this region
0x0030 | 4 | Trigger value
0x0034 | 4 | Cursor index (cursors.bam)
0x0038 | 8 | Destination area (for travel regions)
0x0040 | 32 | Entrance name in destination area (for travel regions)
0x0060 | 4 | Flags
0x0064 | 4 | Information text (for info points)
0x0068 | 2 | Trap detection difficulty percentage
0x006a | 2 | Trap removal difficulty percentage
0x006c | 2 | Is region trapped? 0: No / 1: Yes
0x006e | 2 | Is trap detected? 0: No / 1: Yes
0x0070 | 4 | Trap launch location
0x0074 | 8 | Key item
0x007c | 8 | Region script
0x0084 | 2 | Alternative use point X coordinate
0x0086 | 2 | Alternative use point Y coordinate
0x0088 | 4 | Unknown
0x008c | 32 | Unknown
0x00ac | 8 | Sound (PST, PSTEE)
0x00b4 | 2 | Talk location point X coordinate (PST, PSTEE)
0x00b6 | 2 | Talk location point Y coordinate (PST, PSTEE)
0x00b8 | 4 | Speaker name (PST, PSTEE)
0x00bc | 8 | Dialog file (PST, PSTEE)
*/
#[derive(Clone, Debug, Default)]
pub struct AreRegion
{
	pub name: String,
	pub regionType: u16,
	pub boundingBox: BoundingBox,
	pub vertexCount: u16,
	pub vertexFirst: u32,
	pub trigger: u32,
	pub cursorIndex: u32,
	pub destination: String,
	pub entranceName: String,
	pub flags: u32,
	pub textIndex: u32,
	pub trapDetectionDifficulty: u16,
	pub trapRemovalDifficulty: u16,
	pub trapped: u16,
	pub trapDetected: u16,
	pub trapLaunchLocation: u32,
	pub keyItem: String,
	pub script: String,
	pub alternativeUse: Point2D<u16>,
	pub sound: String,
	pub talkLocation: Point2D<u16>,
	pub speaker: u32,
	pub dialog: String,
}

impl AreRegion
{
	const UnknownSize: u64 = 36;
}

impl Readable for AreRegion
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readName(cursor)?;
		let regionType = cursor.read_u16::<LittleEndian>()?;
		
		let bbValue = cursor.read_u64::<LittleEndian>()?;
		let boundingBox = BoundingBox::from(bbValue);
		
		let vertexCount = cursor.read_u16::<LittleEndian>()?;
		let vertexFirst = cursor.read_u32::<LittleEndian>()?;
		let trigger = cursor.read_u32::<LittleEndian>()?;
		let cursorIndex = cursor.read_u32::<LittleEndian>()?;
		let destination = readResRef(cursor)?;
		let entranceName = readName(cursor)?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		let textIndex = cursor.read_u32::<LittleEndian>()?;
		let trapDetectionDifficulty = cursor.read_u16::<LittleEndian>()?;
		let trapRemovalDifficulty = cursor.read_u16::<LittleEndian>()?;
		let trapped = cursor.read_u16::<LittleEndian>()?;
		let trapDetected = cursor.read_u16::<LittleEndian>()?;
		let trapLaunchLocation = cursor.read_u32::<LittleEndian>()?;
		let keyItem = readResRef(cursor)?;
		let script = readResRef(cursor)?;
		let alternativeUse = Point2D::<u16>::fromCursor(cursor)?;
		
		cursor.set_position(cursor.position() + Self::UnknownSize);
		
		let sound = readResRef(cursor)?;
		let talkLocation = Point2D::<u16>::fromCursor(cursor)?;
		let speaker = cursor.read_u32::<LittleEndian>()?;
		let dialog = readResRef(cursor)?;
		
		return Ok(Self
		{
			name,
			regionType,
			boundingBox,
			vertexCount,
			vertexFirst,
			trigger,
			cursorIndex,
			destination,
			entranceName,
			flags,
			textIndex,
			trapDetectionDifficulty,
			trapRemovalDifficulty,
			trapped,
			trapDetected,
			trapLaunchLocation,
			keyItem,
			script,
			alternativeUse,
			sound,
			talkLocation,
			speaker,
			dialog,
		});
	}
}
