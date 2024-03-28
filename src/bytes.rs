use std::io::Cursor;
use ::anyhow::Result;
use crate::types::TypeSize_RESREF;

pub const Nul: &str = "\0";
pub const StringNameLength: usize = 32;

/**
Convert an array of bytes into a String.

---

Parameter | Description
---|---
$bytes | The array of bytes to be converted. **Expects a slice.**
*/
#[macro_export]
macro_rules! parseString
{
	($bytes:expr) => {
		{
			use crate::bytes::Nul;
			
			let parsed = String::from_utf8($bytes.into())
				.map_err(|nonUtf8| String::from_utf8_lossy(nonUtf8.as_bytes()).to_string());
			
			let out = match parsed
			{
				Ok(success) => success,
				Err(notSuccess) => notSuccess,
			};
			
			//Trim NUL, and any following characters, from the end of the string
			match out.find(Nul)
			{
				Some(idx) => out[0..idx].to_string(),
				None => out,
			}
		}
	}
}

/**
Read an arbitrary number of bytes from a `std::io::Cursor` instance.

---

Parameter | Description
---|---
$cursor | The cursor being read.
$length | The amount of bytes to read.
*/
#[macro_export]
macro_rules! readBytes
{
	($cursor:expr, $length:expr) => {
		{
			let mut nameBytes = vec![];
			for _ in 0..$length
			{
				let byte = $cursor.read_u8()?;
				nameBytes.push(byte);
			}
			nameBytes
		}
	}
}

/**
Read an arbitrary number of bytes from a `std::io::Cursor` instance.

---

Parameter | Description
---|---
$cursor | The cursor being read.
$length | The amount of bytes to read. **Must be constant or literal!**
*/
#[macro_export]
macro_rules! readBytesExact
{
	($cursor:expr, $length:expr) => {
		{
			use std::io::Read;
			
			let mut bytes: [u8; $length] = [0; $length];
			$cursor.read_exact(&mut bytes)?;
			
			bytes
		}
	}
}

/**
Read an arbitrary number of bytes from a `std::io::Cursor` and convert them into
a `String`.

The list of bytes is first filtered to only include those bytes which occur
before the first NUL value. Then the resultant String also has any NUL characters
trimmed before returning.

---

Parameter | Description
---|---
$cursor | The `Cursor` instance from which to read the bytes.
$length | The amount of bytes to read. **Must be constant or literal!**
*/
#[macro_export]
macro_rules! readString
{
	($cursor:expr, $length:expr) => {
		{
			use crate::{parseString, readBytesExact};
			
			let bytes = readBytesExact!($cursor, $length);
			parseString!(bytes)
		}
	}
}

/**
Read a string, the size of a RESREF value (8 bytes), from a `std::io::Cursor`
instance.

Any NUL characters are removed from the string before returning.

---

Parameter | Description
---|---
cursor | The cursor from which to read the string.

---

#### Note

The cursor's position is not updated before reading.
*/
pub fn readResRef(cursor: &mut Cursor<Vec<u8>>) -> Result<String>
{
	let resref = readString!(cursor, TypeSize_RESREF);
	return Ok(resref);
}

/**
Read a string, the size of a typical name value (32 bytes), from a
`std::io::Cursor` instance.

Any NUL characters are removed from the string before returning.

---

Parameter | Description
---|---
cursor | The cursor from which to read the string.

---

#### Note

The cursor's position is not updated before reading.
*/
pub fn readName(cursor: &mut Cursor<Vec<u8>>) -> Result<String>
{
	let name = readString!(cursor, StringNameLength);
	return Ok(name);
}
