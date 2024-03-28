use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use super::Readable;

#[derive(Clone, Copy, Debug, Default)]
pub struct SectionAddress<A, B>
	where A: Copy,
		B: Copy,
{
	pub offset: A,
	pub count: B,
}

impl SectionAddress<u16, u16>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let count = cursor.read_u16::<LittleEndian>()?;
		let offset = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl Readable for SectionAddress<u16, u16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u16::<LittleEndian>()?;
		let count = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl SectionAddress<u16, u32>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let count = cursor.read_u32::<LittleEndian>()?;
		let offset = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl Readable for SectionAddress<u16, u32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u16::<LittleEndian>()?;
		let count = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl SectionAddress<u32, u16>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let count = cursor.read_u16::<LittleEndian>()?;
		let offset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl Readable for SectionAddress<u32, u16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u32::<LittleEndian>()?;
		let count = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl SectionAddress<u32, u32>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let count = cursor.read_u32::<LittleEndian>()?;
		let offset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}

impl Readable for SectionAddress<u32, u32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u32::<LittleEndian>()?;
		let count = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			count,
		});
	}
}
