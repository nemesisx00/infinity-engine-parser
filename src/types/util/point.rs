use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use super::Readable;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point2D<T>
	where T: Copy,
{
	pub x: T,
	pub y: T,
}

impl Into<Point2D<u32>> for Point2D<u16>
{
	fn into(self) -> Point2D<u32>
	{
		return Point2D
		{
			x: self.x.into(),
			y: self.y.into(),
		};
	}
}

impl Readable for Point2D<i16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_i16::<LittleEndian>()?;
		let y = cursor.read_i16::<LittleEndian>()?;
		
		return Ok(Self { x, y });
	}
}

impl Readable for Point2D<u16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_u16::<LittleEndian>()?;
		let y = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self { x, y });
	}
}

impl Readable for Point2D<i32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_i32::<LittleEndian>()?;
		let y = cursor.read_i32::<LittleEndian>()?;
		
		return Ok(Self { x, y });
	}
}

impl Readable for Point2D<u32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_u32::<LittleEndian>()?;
		let y = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self { x, y });
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point3D<T>
	where T: Copy,
{
	pub x: T,
	pub y: T,
	pub z: T,
}

impl Into<Point3D<u32>> for Point3D<u16>
{
	fn into(self) -> Point3D<u32>
	{
		return Point3D
		{
			x: self.x.into(),
			y: self.y.into(),
			z: self.z.into(),
		};
	}
}

impl Readable for Point3D<i16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_i16::<LittleEndian>()?;
		let y = cursor.read_i16::<LittleEndian>()?;
		let z = cursor.read_i16::<LittleEndian>()?;
		
		return Ok(Self { x, y, z });
	}
}

impl Readable for Point3D<u16>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_u16::<LittleEndian>()?;
		let y = cursor.read_u16::<LittleEndian>()?;
		let z = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self { x, y, z });
	}
}

impl Readable for Point3D<i32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_i32::<LittleEndian>()?;
		let y = cursor.read_i32::<LittleEndian>()?;
		let z = cursor.read_i32::<LittleEndian>()?;
		
		return Ok(Self { x, y, z });
	}
}

impl Readable for Point3D<u32>
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let x = cursor.read_u32::<LittleEndian>()?;
		let y = cursor.read_u32::<LittleEndian>()?;
		let z = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self { x, y, z });
	}
}
