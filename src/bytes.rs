#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use anyhow::Error;
use ::anyhow::Result;
use crate::types::TypeSize_RESREF;

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
			let parsed = String::from_utf8($bytes.into())
				.map_err(|nonUtf8| String::from_utf8_lossy(nonUtf8.as_bytes()).to_string());
			
			match parsed
			{
				Ok(success) => success,
				Err(notSuccess) => notSuccess,
			}
				.replace("\0", "")
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
			use crate::bytes::readToReal;
			
			let bytes = readBytesExact!($cursor, $length);
			let mut realBytes: [u8; $length] = [0; $length];
			readToReal(&bytes, &mut realBytes)?;
			
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

/**
Fill the `real` byte array with values from the `read` byte array in order until
a NUL is found.

#### Note

Strings stored in Infinity Engine resources are NUL terminated and are not
guaranteed to fill up the entire possible length available in the binary data.
Thus only values up to the first NUL should be considered valid data with any
remaining values being considered garbage to be discarded.
*/
pub fn readToReal(read: &[u8], real: &mut [u8]) -> Result<()>
{
	if read.len() != real.len()
	{
		return Err(
			Error::new(
				std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					format!("Byte arrays are not the same length! {} != {}", read.len(), real.len())
				)
			)
		);
	}
	
	for (i, byte) in read.iter().enumerate()
	{
		match *byte > 0
		{
			true => real[i] = *byte,
			false => break,
		}
	}
	
	return Ok(());
}
