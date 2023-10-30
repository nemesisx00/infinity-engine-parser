#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use std::io::Cursor;
use ::anyhow::Result;
use crate::readString;
use super::Readable;

/**
Simple data structure containing only the Signature and Version of a file. Used
to quickly identify the type of a file without attempting to parse the entire
contents.
*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Identity
{
	pub signature: String,
	pub version: String,
}

impl Identity
{
	const StringLength: usize = 4;
	
	/**
	Create a new instance of `Identity` based on the data contained in `file`.
	
	---
	
	### Parameters
	- **file** &Path - The fully qualified path to the file being read.
	*/
	pub fn fromFile(file: &Path) -> Result<Self>
	{
		let buffer = std::fs::read(file)?;
		let mut cursor = Cursor::new(buffer);
		
		return Self::fromCursor(&mut cursor);
	}
}

impl Readable for Identity
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let signature = readString!(cursor, Self::StringLength);
		let version = readString!(cursor, Self::StringLength);
		
		return Ok(Self
		{
			signature,
			version,
		});
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	
	#[test]
	fn FromCursor()
	{
		let data: Vec<u8> = vec![0x4b, 0x45, 0x59, 0x20, 0x56, 0x31, 0x20, 0x20];
		let sig = "KEY ";
		let ver = "V1  ";
		
		let mut cursor = Cursor::new(data);
		let result = Identity::fromCursor(&mut cursor).unwrap();
		
		assert_eq!(sig, result.signature);
		assert_eq!(ver, result.version);
	}
}
