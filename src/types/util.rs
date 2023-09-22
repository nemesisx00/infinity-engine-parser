#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs;
use std::io::Cursor;
use std::path::Path;
use ::anyhow::{Result, Context};
use ::byteorder::ReadBytesExt;
use ::safer_ffi::derive_ReprC;
use crate::{readBytes, readString};

pub const TypeSize_RESREF: usize = 8;

/**
A data type which can be found in and read from Infinity Engine game files.
*/
pub trait InfinityEngineType
{
	type Output;
	
	/**
	Create a new instance of type `T` based on the data contained in `cursor`.
	
	---
	
	### Parameters
	- **cursor** - The cursor from which to read data.
	*/
	fn fromCursor<T>(cursor: &mut Cursor<Vec<u8>>) -> Result<Self::Output>
		where T: InfinityEngineType;
}

/**
Create a new instance of type `T` based on the data contained in `file`.

---

### Parameters
- **file** - The fully qualified path to the file being read.
*/
pub fn ReadFromFile<T>(file: &Path) -> Result<T::Output>
	where T: InfinityEngineType
{
	let buffer = fs::read(file)
		.context("Failed reading an Infinity Engine game file")?;
	let mut cursor = Cursor::new(buffer);
	
	return T::fromCursor::<T>(&mut cursor);
}

/**
Data structure for passing a resource's height and width across the FFI border.
*/
#[derive_ReprC]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Dimensions
{
	pub height: i32,
	pub width: i32,
}

impl Dimensions
{
	pub fn new(height: i32, width: i32) -> Self
	{
		return Self
		{
			height,
			width,
		};
	}
}

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
	
	/**
	Create a new instance of `Identity` based on the data contained in `cursor`.
	
	---
	
	### Parameters
	- **cursor** &mut Cursor<Vec<u8>> - The cursor from which to read data.
	*/
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let sigValue = readBytes!(cursor, 4);
		let signature = readString!(sigValue);
		let verValue = readBytes!(cursor, 4);
		let version = readString!(verValue);
		
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
	fn FromCursorTest()
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
