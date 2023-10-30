#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::util::Readable;

/**
The fully parsed contents of an Item in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 8 | Item resref
0x0008 | 2 | Item expiration time (replace with drained item)
0x000a | 2 | Quantity/Charges 1
0x000c | 2 | Quantity/Charges 2
0x000e | 2 | Quantity/Charges 3
0x0010 | 4 | Flags
*/
#[derive(Clone, Debug, Default)]
pub struct AreItem
{
	pub resref: String,
	pub expirationTime: u16,
	pub quantities: Vec<u16>,
	pub flags: u32,
}

impl Readable for AreItem
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let resref = readResRef(cursor)?;
		let expirationTime = cursor.read_u16::<LittleEndian>()?;
		
		let mut quantities = vec![];
		for _ in 0..3
		{
			let quantity = cursor.read_u16::<LittleEndian>()?;
			quantities.push(quantity);
		}
		
		let flags = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			resref,
			expirationTime,
			quantities,
			flags,
		});
	}
}
