use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bits::ReadValue;
use super::Readable;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BoundingBox
{
	pub bottom: u16,
	pub left: u16,
	pub right: u16,
	pub top: u16,
}

impl From<u64> for BoundingBox
{
    fn from(value: u64) -> Self
	{
		let left = ReadValue(value.into(), 16, 0) as u16;
		let top = ReadValue(value.into(), 16, 16) as u16;
		let right = ReadValue(value.into(), 16, 32) as u16;
		let bottom = ReadValue(value.into(), 16, 48) as u16;
		
		return Self
		{
			bottom,
			left,
			right,
			top,
		};
    }
}

impl Readable for BoundingBox
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let left = cursor.read_u16::<LittleEndian>()?;
		let top = cursor.read_u16::<LittleEndian>()?;
		let right = cursor.read_u16::<LittleEndian>()?;
		let bottom = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			bottom,
			left,
			right,
			top,
		});
	}
}
