#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::{readBytes, parseString};
use super::{Identity, InfinityEngineType, Readable};

/**
The fully parsed contents of a TLK V1 file.
 
See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm

Most strings shown in Infinity Engine games are stored in a TLK file, usually
dialog.tlk (for male/default text) and/or dialogf.tlk (for female text).
Strings are stored with assocaited information, such as a reference to a sound
file, and are indexed by a 32-bit identifier called a STRREF. Storing text in
this way allows for easy implementation of internationalization.

---

### Header Data

Offset | Size | Description
--- | --- | ---
0x0000 | 4 | Signature ('TLK ')
0x0004 | 4 | Version ('V1  ')
0x0008 | 2 | Language ID
0x000a | 4 | Number of STRREF entries in this file
0x000e | 4 | Offset to string data
*/
#[derive(Clone, Debug, Default)]
pub struct Tlk
{
	pub identity: Identity,
	pub language: u16,
	pub count: u32,
	pub offset: u32,
	pub entries: Vec<TlkEntry>,
	pub strings: Vec<String>,
}

impl Tlk
{
	const Signature: &str = "TLK ";
	const Version: &str = "V1  ";
}

impl InfinityEngineType for Tlk {}

impl Readable for Tlk
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let identity = Identity::fromCursor(cursor)?;
		let language = cursor.read_u16::<LittleEndian>()?;
		let count = cursor.read_u32::<LittleEndian>()?;
		let offset = cursor.read_u32::<LittleEndian>()?;
		
		let mut entries = vec![];
		for strref in 0..count
		{
			let mut entry = TlkEntry::fromCursor(cursor)?;
			entry.strref = strref;
			entries.push(entry);
		}
		
		let mut strings = vec![];
		for entry in entries.iter()
		{
			cursor.set_position((offset + entry.offset).into());
			let bytes = readBytes!(cursor, entry.length);
			let string = parseString!(bytes);
			strings.insert(entry.strref as usize, string);
		}
		
		return Ok(Self {
			identity,
			language,
			count,
			offset,
			entries,
			strings,
		});
	}
}

// --------------------------------------------------

/**
The fully parsed contents of an Entry in a TLK V1 file.
 
See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm

---

Offset | Size | Description
--- | --- | ---
0x0000 | 2 | Bit field
0x0002 | 8 | Resource name of associated sound
0x000a | 4 | Volume variance (Unused, at minimum in BG1)
0x000e | 4 | Pitch variance (Unused, at minimum in BG1)
0x0012 | 4 | Offset of this string relative to the strings section
0x0016 | 4 | Length of this string
*/
#[derive(Clone, Debug, Default)]
pub struct TlkEntry
{
	pub strref: u32,
	pub info: u16,
	pub sound: String,
	pub volume: u32,
	pub pitch: u32,
	pub offset: u32,
	pub length: u32,
}

impl Readable for TlkEntry
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let info = cursor.read_u16::<LittleEndian>()?;
		let sound = readResRef(cursor)?;
		let volume = cursor.read_u32::<LittleEndian>()?;
		let pitch = cursor.read_u32::<LittleEndian>()?;
		let offset = cursor.read_u32::<LittleEndian>()?;
		let length = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			info,
			sound,
			volume,
			pitch,
			offset,
			length,
			..Default::default()
		});
	}
}

// --------------------------------------------------

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::platform::Games;
	use crate::resource::ResourceManager;
	
    #[test]
    fn ParseTlk()
	{
		let game = Games::BaldursGate1;
		let fileName = "dialog.tlk".to_string();
		
		let resourceManager = ResourceManager::default();
		let result = resourceManager.loadTlk(game, fileName).unwrap();
		
		assert_eq!(Tlk::Signature, result.identity.signature);
		assert_eq!(Tlk::Version, result.identity.version);
		assert_ne!(0, result.count);
		assert_eq!(result.count as usize, result.strings.len());
	}
}
