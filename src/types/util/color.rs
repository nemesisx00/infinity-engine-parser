use ::anyhow::Result;
use ::byteorder::ReadBytesExt;
use crate::bits::ReadValue;
use super::Readable;

/**
Data structure representing a single 32-bit color value.

Supported color orders:

- `RGBA`
- `BGRA`

The default order is `RGBA`, which is used in the implementations of the
following traits:

- `From<u32>`
- `Into<u32>`
-  `Readable`
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color
{
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

impl Color
{
	const ByteSize: u8 = 8;
	const One: u8 = 24;
	const Two: u8 = 16;
	const Three: u8 = 8;
	const Four: u8 = 0;
	
	pub fn bytes(&self) -> Vec<u8>
	{
		return vec![self.red, self.green, self.blue, self.alpha];
	}
	
	pub fn fromBGRA(value: u32) -> Self
	{
		let blue = ReadValue(value.into(), Self::ByteSize.into(), Self::One.into()) as u8;
		let green = ReadValue(value.into(), Self::ByteSize.into(), Self::Two.into()) as u8;
		let red = ReadValue(value.into(), Self::ByteSize.into(), Self::Three.into()) as u8;
		let alpha = ReadValue(value.into(), Self::ByteSize.into(), Self::Four.into()) as u8;
		
		return Self
		{
			red,
			green,
			blue,
			alpha,
		}
	}
	
	pub fn intoBGRA(&self) -> u32
	{
		return ((self.red as u32) << Self::Three)
			| ((self.green as u32) << Self::Two)
			| ((self.blue as u32) << Self::One)
			| self.alpha as u32;
	}
}

impl From<u32> for Color
{
	fn from(value: u32) -> Self
	{
		let red = ReadValue(value.into(), Self::ByteSize.into(), Self::One.into()) as u8;
		let green = ReadValue(value.into(), Self::ByteSize.into(), Self::Two.into()) as u8;
		let blue = ReadValue(value.into(), Self::ByteSize.into(), Self::Three.into()) as u8;
		let alpha = ReadValue(value.into(), Self::ByteSize.into(), Self::Four.into()) as u8;
		
		return Self
		{
			red,
			green,
			blue,
			alpha,
		}
	}
}

impl Into<u32> for Color
{
	fn into(self) -> u32
	{
		return ((self.red as u32) << Self::One)
			| ((self.green as u32) << Self::Two)
			| ((self.blue as u32) << Self::Three)
			| self.alpha as u32;
	}
}

impl Readable for Color
{
	fn fromCursor(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let red = cursor.read_u8()?;
		let green = cursor.read_u8()?;
		let blue = cursor.read_u8()?;
		let alpha = cursor.read_u8()?;
		
		return Ok(Self
		{
			red,
			green,
			blue,
			alpha,
		});
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	#[test]
	fn fromInto()
	{
		let red: u32 = 0xFF000000;
		let green: u32 = 0x00FF0000;
		let blue: u32 = 0x0000FF00;
		let alpha: u32 = 0x000000FF;
		
		let rc = Color::from(red);
		assert_eq!(255, rc.red);
		assert_eq!(0, rc.green);
		assert_eq!(0, rc.blue);
		assert_eq!(0, rc.alpha);
		
		let gc = Color::from(green);
		assert_eq!(0, gc.red);
		assert_eq!(255, gc.green);
		assert_eq!(0, gc.blue);
		assert_eq!(0, gc.alpha);
		
		let bc = Color::from(blue);
		assert_eq!(0, bc.red);
		assert_eq!(0, bc.green);
		assert_eq!(255, bc.blue);
		assert_eq!(0, bc.alpha);
		
		let ac = Color::from(alpha);
		assert_eq!(0, ac.red);
		assert_eq!(0, ac.green);
		assert_eq!(0, ac.blue);
		assert_eq!(255, ac.alpha);
		
		let rresult: u32 = rc.into();
		let gresult: u32 = gc.into();
		let bresult: u32 = bc.into();
		let aresult: u32 = ac.into();
		
		assert_eq!(red, rresult);
		assert_eq!(green, gresult);
		assert_eq!(blue, bresult);
		assert_eq!(alpha, aresult);
	}
	
	#[test]
	fn fromIntoBgra()
	{
		let complex: u32 = 0xAABBCCDD;
		let expected: u32 = 0xCCBBAADD;
		let result = Color::fromBGRA(complex);
		assert_eq!(complex, result.intoBGRA());
		assert_eq!(expected, result.into());
	}
}
