use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::Readable;

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

impl Door
{
	const ValueOpen: u16 = 0;
	
	pub fn isOpen(&self) -> bool { return self.openClosed == Self::ValueOpen; }
}

impl Readable for Door
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readResRef(cursor)
			.context("Failed to read the RESREF name")?;
		let openClosed = cursor.read_u16::<LittleEndian>()
			.context("Failed to read the u16 openClosed")?;
		let firstDoorIndex = cursor.read_u16::<LittleEndian>()
			.context("Failed to read the u16 firstDoorIndex")?;
		let doorCount = cursor.read_u16::<LittleEndian>()
			.context("Failed to read the u16 doorCount")?;
		let openCount = cursor.read_u16::<LittleEndian>()
			.context("Failed to read the u16 openCount")?;
		let closedCount = cursor.read_u16::<LittleEndian>()
			.context("Failed to read the u16 closedCount")?;
		let openOffset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read the u32 openOffset")?;
		let closedOffset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read the u32 closedOffset")?;
		
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
