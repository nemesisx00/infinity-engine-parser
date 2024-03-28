use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::util::{Readable, Point2D};

/**
The fully parsed contents of an Entrance in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | Current X coordinate
0x0022 | 2 | Current Y coordinate
0x0024 | 2 | Orientation
*/
#[derive(Clone, Debug, Default)]
pub struct AreEntrance
{
	pub name: String,
	pub coordinates: Point2D<u16>,
	pub orientation: u16,
}

impl AreEntrance
{
	const UnusedPadding: u64 = 66;
}

impl Readable for AreEntrance
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readResRef(cursor)?;
		let coordinates = Point2D::<u16>::fromCursor(cursor)?;
		let orientation = cursor.read_u16::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			coordinates,
			orientation,
		});
	}
}
