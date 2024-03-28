use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::{Readable, Point2D};


/**
The fully parsed contents of an Animation in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | Current X coordinate
0x0022 | 2 | Current Y coordinate
0x0024 | 4 | Animation appearance schedule
0x0028 | 8 | Animation resref
0x0030 | 2 | BAM sequence number
0x0032 | 2 | BAM frame number
0x0034 | 4 | Flags
0x0038 | 2 | Height
0x003a | 2 | Transparency (0xFF is invisible)
0x003c | 2 | Starting frame (0 indicates random frame. Synchronized will clear this)
0x003e | 1 | Change of looping (0 defaults to 100)
0x003f | 1 | Skip cycles
0x0040 | 8 | Palette
0x0048 | 2 | Animation width
0x004a | 2 | Animation height
*/
#[derive(Clone, Debug, Default)]
pub struct AreAnimation
{
	pub name: String,
	pub coordinate: Point2D<u16>,
	pub appearanceSchedule: u32,
	pub resref: String,
	pub bamSequence: u16,
	pub bamFrame: u16,
	pub flags: u32,
	pub height: u16,
	pub transparency: u16,
	pub startFrame: u16,
	pub loopChance: u8,
	pub skipCycles: u8,
	pub palette: String,
	pub animationWidth: u16,
	pub animationHeight: u16,
}

impl Readable for AreAnimation
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let name = readName(cursor)?;
		let coordinate = Point2D::<u16>::fromCursor(cursor)?;
		let appearanceSchedule = cursor.read_u32::<LittleEndian>()?;
		let resref = readResRef(cursor)?;
		let bamSequence = cursor.read_u16::<LittleEndian>()?;
		let bamFrame = cursor.read_u16::<LittleEndian>()?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		let height = cursor.read_u16::<LittleEndian>()?;
		let transparency = cursor.read_u16::<LittleEndian>()?;
		let startFrame = cursor.read_u16::<LittleEndian>()?;
		let loopChance = cursor.read_u8()?;
		let skipCycles = cursor.read_u8()?;
		let palette = readResRef(cursor)?;
		let animationWidth = cursor.read_u16::<LittleEndian>()?;
		let animationHeight = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			name,
			coordinate,
			appearanceSchedule,
			resref,
			bamSequence,
			bamFrame,
			flags,
			height,
			transparency,
			startFrame,
			loopChance,
			skipCycles,
			palette,
			animationWidth,
			animationHeight,
		});
	}
}
