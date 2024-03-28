use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readName;
use crate::types::util::Readable;

/**
The fully parsed contents of an Item in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

Variables can be associated with an area by specifying the six character area
name as the variable namespace (i.e. AR1000). Variables associated with an area
in this way are stored in the ARE file, in this section (in save games). Note
the format specification allows variables of differeng types, however the engine
implementation only reads and writes INT variables.

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Variable name
0x0020 | 2 | Type
0x0022 | 2 | Resource type
0x0024 | 4 | Dword value
0x0028 | 4 | Int value
0x002c | 8 | Double value
0x0030 | 32 | Script name value
*/
#[derive(Clone, Debug, Default)]
pub struct AreVariable
{
	pub name: String,
	pub variableType: u16,
	pub resourceType: u16,
	pub dword: u32,
	pub int: u32,
	pub double: u64,
	pub scriptName: String,
}

impl Readable for AreVariable
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readName(cursor)?;
		let variableType = cursor.read_u16::<LittleEndian>()?;
		let resourceType = cursor.read_u16::<LittleEndian>()?;
		let dword = cursor.read_u32::<LittleEndian>()?;
		let int = cursor.read_u32::<LittleEndian>()?;
		let double = cursor.read_u64::<LittleEndian>()?;
		let scriptName = readName(cursor)?;
		
		return Ok(Self
		{
			name,
			variableType,
			resourceType,
			dword,
			int,
			double,
			scriptName,
		});
	}
}
