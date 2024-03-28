use std::io::Cursor;
use ::anyhow::Result;

/**
A data type which can be found in and read from Infinity Engine game files.
*/
pub trait InfinityEngineType {}

/**
A data type which can be read from a Cursor-wrapped byte array.
*/
pub trait Readable
{
	/**
	Create a new instance based on the data contained in `cursor`.
	
	---
	
	### Parameters
	- **cursor** - The cursor from which to read data.
	*/
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized;
}

pub trait ReadIntoSelf
{
	fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<()>;
}
