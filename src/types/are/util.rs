use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::util::Readable;

#[derive(Clone, Debug, Default)]
pub struct AreRef
{
	pub name: String,
	pub flags: u32,
}

impl Readable for AreRef
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readResRef(cursor)?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			name,
			flags,
		});
	}
}
