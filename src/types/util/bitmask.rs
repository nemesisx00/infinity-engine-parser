#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use super::Readable;

#[derive(Clone, Debug, Default)]
pub struct BitmaskAddress<A, B>
	where A: Copy,
		B: Copy,
{
	pub offset: A,
	pub size: B,
}

impl Readable for BitmaskAddress<u16, u16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u16::<LittleEndian>()?;
		let size = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl BitmaskAddress<u16, u16>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let size = cursor.read_u16::<LittleEndian>()?;
		let offset = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl Readable for BitmaskAddress<u16, u32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u16::<LittleEndian>()?;
		let size = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl BitmaskAddress<u16, u32>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let size = cursor.read_u32::<LittleEndian>()?;
		let offset = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl Readable for BitmaskAddress<u32, u16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u32::<LittleEndian>()?;
		let size = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl BitmaskAddress<u32, u16>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let size = cursor.read_u16::<LittleEndian>()?;
		let offset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl Readable for BitmaskAddress<u32, u32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let offset = cursor.read_u32::<LittleEndian>()?;
		let size = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}

impl BitmaskAddress<u32, u32>
{
	pub fn fromCursorInverted(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let size = cursor.read_u32::<LittleEndian>()?;
		let offset = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			offset,
			size,
		});
	}
}
