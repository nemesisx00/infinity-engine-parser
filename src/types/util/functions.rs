use std::fs;
use std::io::Cursor;
use std::path::Path;
use ::anyhow::{Context, Result};
use super::{InfinityEngineType, Readable};

/**
Create a new instance of type `T` based on the data contained in `file`.

---

Name | Description
---|---
file | The fully qualified path to the file being read.
*/
pub fn ReadFromFile<T>(file: &Path) -> Result<T>
	where T: InfinityEngineType + Readable
{
	let buffer = fs::read(file)
		.context("Failed reading an Infinity Engine game file")?;
	let mut cursor = Cursor::new(buffer);
	
	return T::fromCursor(&mut cursor);
}

/**
Read a list of structs from a `std::io::Cursor` instance.

---

Name | Description
---|---
cursor | The cursor from which to read.
offset | The offset used to set the cursor's position before reading.
count | The number of structs to read from the cursor.

---

### Note

The cursor's position is updated before reading.
*/
pub fn ReadList<T>(cursor: &mut Cursor<Vec<u8>>, offset: u64, count: u64) -> Result<Vec<T>>
	where T: Readable
{
	let mut list = vec![];
	if offset != cursor.position()
	{
		cursor.set_position(offset);
	}
	
	for _ in 0..count
	{
		let instance = T::fromCursor(cursor)?;
		list.push(instance);
	}
	
	return Ok(list);
}
