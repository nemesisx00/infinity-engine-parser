use std::io::{Cursor, Read};
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::parseString;
use crate::types::util::{Readable, Point2D};

/**
The fully parsed contents of an AutomapNote in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 2 | X coordinate
0x0002 | 2 | Y coordinate
0x0004 | 4 | Note text (dialog.tlk or TOH/TOT file)
0x0008 | 2 | Strref location. 0: External (TOH/TOT) / 1: Internal (TLK)
0x000a | 2 | Color of automap marker
0x000c | 4 | Note count + 10

#### PST

Offset | Size | Description
---|---|---
0x0000 | 4 | X coordinate
0x0004 | 4 | Y coordinate
0x0008 | 500 | Text
0x01fc | 4 | Note color. 0: Blue user note / 1: Red game note
*/
#[derive(Clone, Debug, Default)]
pub struct AreAutomapNote
{
	/// Identifies this instance as data from PST or not
	pub planescape: bool,
	/// Most titles 2 byte values, PST 4 byte values
	pub coordinate: Point2D<u32>,
	/// Not used in PST
	pub textIndex: u32,
	/// Only used in PST
	pub text: String,
	/// Not used in PST
	pub location: u16,
	/// Most titles 2 bytes, PST 4 bytes
	pub color: u32,
	/// Not used in PST
	pub count: u32,
}

impl AreAutomapNote
{
	const PstTextLength: usize = 500;
	const PstUnusedPadding: u64 = 20;
	const UnusedPadding: u64 = 36;
	
	pub fn fromCursorPst(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let coordinate = Point2D::<u32>::fromCursor(cursor)?;
		
		let mut textBytes: [u8; Self::PstTextLength] = [0; Self::PstTextLength];
		cursor.read_exact(&mut textBytes)?;
		let text = parseString!(textBytes);
		let color = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::PstUnusedPadding);
		
		return Ok(Self
		{
			planescape: true,
			coordinate,
			text,
			color,
			..Default::default()
		});
	}
}

impl Readable for AreAutomapNote
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let coordinate = Point2D::<u16>::fromCursor(cursor)?;
		let textIndex = cursor.read_u32::<LittleEndian>()?;
		let location = cursor.read_u16::<LittleEndian>()?;
		let color = cursor.read_u16::<LittleEndian>()?;
		let count = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			planescape: false,
			coordinate: coordinate.into(),
			textIndex,
			location,
			color: color.into(),
			count,
			..Default::default()
		});
	}
}
