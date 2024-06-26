use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::{readBytes, parseString};
use crate::bits::ReadValue;
use super::{Identity, InfinityEngineType, Readable};

/**
The fully parsed contents of a KEY V1 file.
 
See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm

This file format acts as a central reference point to locate files required
by the game (in a BIFF file on a CD or in the override directory). The key
e also maintains a mapping from an 8 byte resource name (resref) to a 32
byte ID (using the lowest 12 bits to identify a resource). There is generally
only one key file with each game (chitin.key).

---

### Header Data

Offset | Size | Description
--- | --- | ---
0x0000 | 4 | Signature ('KEY ')
0x0004 | 4 | Version ('V1  ')
0x0008 | 4 | Count of BIF entries
0x000c | 4 | Count of resource entries
0x0010 | 4 | Offset (from start of file) to BIF entries
0x0014 | 4 | Offset (from start of file) to resource entries
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key
{
	pub identity: Identity,
	pub bifCount: u32,
	pub resourceCount: u32,
	pub bifOffset: u32,
	pub resourceOffset: u32,
	pub bifEntries: Vec<BifEntry>,
	pub resourceEntries: Vec<ResourceEntry>,
}

impl Key
{
	const FileName: &'static str = "chitin.key";
	const Signature: &'static str = "KEY ";
	const Version: &'static str = "V1  ";
}

impl InfinityEngineType for Key {}

impl Readable for Key
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let identity = Identity::fromCursor(cursor)?;
		let bifCount = cursor.read_u32::<LittleEndian>()?;
		let resourceCount = cursor.read_u32::<LittleEndian>()?;
		let bifOffset = cursor.read_u32::<LittleEndian>()?;
		let resourceOffset = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(bifOffset as u64);
		let mut bifEntries = vec![];
		for _ in 0..bifCount
		{
			let bifEntry = BifEntry::fromCursor(cursor)?;
			bifEntries.push(bifEntry);
		}
		
		cursor.set_position(resourceOffset as u64);
		let mut resourceEntries = vec![];
		for _ in 0..resourceCount
		{
			let resourceEntry = ResourceEntry::fromCursor(cursor)?;
			resourceEntries.push(resourceEntry);
		}
		
		for i in 0..bifEntries.len()
		{
			if let Some(entry) = bifEntries.get_mut(i)
			{
				cursor.set_position(entry.fileNameOffset as u64);
				let nameBytes = readBytes!(cursor, entry.fileNameLength);
				entry.fileName = parseString!(nameBytes);
			}
		}
		
		return Ok(Self {
			identity,
			bifCount,
			resourceCount,
			bifOffset,
			resourceOffset,
			bifEntries,
			resourceEntries,
		});
	}
}

/**
Metadata defining the details of a BIF file referenced in a given KEY V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Length of BIF file
0x0004 | 4 | Offset from start of file to ASCIIZ BIF filename
0x0008 | 2 | Length, including terminating NUL, of ASCIIZ BIF filename
0x000a | 2 | The 16 bits of this field are used individually to mark the location of the relevant file.

---

(MSB) xxxx xxxx ABCD EFGH (LSB)
	- Bits marked A to F determine on which CD the file is stored (A = CD6, F = CD1)
	- Bit G determines if the file is in the \cache directory
	- Bit H determines if the file is in the \data directory
*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BifEntry
{
	pub fileName: String,
	pub fileLength: u32,
	pub fileNameOffset: u32,
	pub fileNameLength: u16,
	pub locatorBits: u16,
}

impl Readable for BifEntry
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let fileLength = cursor.read_u32::<LittleEndian>()?;
		let fileNameOffset = cursor.read_u32::<LittleEndian>()?;
		let fileNameLength = cursor.read_u16::<LittleEndian>()?;
		let locatorBits = cursor.read_u16::<LittleEndian>()?;
		
		return Ok(Self
		{
			fileLength,
			fileNameOffset,
			fileNameLength,
			locatorBits,
			..Default::default()
		});
	}
}

/**
Metadata defining the details of a resource file referenced in a given KEY V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm

---

Offset | Size | Description
---|---|---
0x0000 | 8 | Resource Name
0x0008 | 2 | Resource type
0x000a | 4 | Resource locator

---

The IE resource manager uses 32-bit values as a 'resource index,' which codifies
the source of the resources as well as which source it refers to. The layout of
this value is below:
	- Bits 31-20: Source index (the ordinal value giving the index of the corresponding BIF entry)
	- Bits 19-14: Tileset index
	- Bits 13-0: Non-tileset file index (any 12 bit value, so long as it matches the value used in the BIF file)
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceEntry
{
	pub name: String,
	pub r#type: u16,
	pub locator: u32,
}

impl ResourceEntry
{
	const BifEntry: u64 = 12;
	const File: u64 = 14;
	const Tileset: u64 = 6;
	
	pub fn indexFile(&self) -> u32
	{
		return ReadValue(self.locator.into(), Self::File, 0) as u32;
	}
	
	pub fn indexTileset(&self) -> u32
	{
		return ReadValue(self.locator.into(), Self::Tileset, Self::File) as u32;
	}
	
	pub fn indexBifEntry(&self) -> u32
	{
		return ReadValue(self.locator.into(), Self::BifEntry, Self::File + Self::Tileset) as u32;
	}
}

impl Readable for ResourceEntry
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readResRef(cursor)?;
		let r#type = cursor.read_u16::<LittleEndian>()?;
		let locator = cursor.read_u32::<LittleEndian>()?;
		
		return Ok(Self
		{
			name,
			r#type,
			locator
		});
	}
}

#[cfg(test)]
mod tests
{
	use std::path::Path;
    use super::*;
	use crate::platform::{FindInstallationPath, Games, KeyFileName};
	use crate::resource::ResourceManager;
	use crate::test::updateResourceManager;
	use crate::types::util::ReadFromFile;
	
    #[test]
    fn LocatorTest()
	{
		let locator = 0xF00028;
		
		let fileExpected = 40;
		let tileExpected = 0;
		let bifExpected = 15;
		
		let instance = ResourceEntry { name: String::default(), r#type: 0, locator };
		
		assert_eq!(fileExpected, instance.indexFile());
		assert_eq!(tileExpected, instance.indexTileset());
		assert_eq!(bifExpected, instance.indexBifEntry());
    }
	
	#[test]
	fn KeyTest()
	{
		let resourceManager = ResourceManager::default();
		let _ = updateResourceManager(&resourceManager);
		
		if let Some(installPath) = resourceManager.getInstallPath(Games::BaldursGate1)
		{
			let keyFile = KeyFileName(Games::BaldursGate1).unwrap();
			let filePath = Path::new(installPath.as_str()).join(keyFile);
			
			let result = ReadFromFile::<Key>(filePath.as_path()).unwrap();
			
			assert_eq!(Key::Signature, result.identity.signature);
			assert_eq!(Key::Version, result.identity.version);
			assert_eq!(159, result.bifCount);
			assert_eq!(16694, result.resourceCount);
			assert_eq!(24, result.bifOffset);
			assert_eq!(4780, result.resourceOffset);
			assert_eq!(result.bifCount as usize, result.bifEntries.len());
			assert_ne!(String::default(), result.bifEntries[0].fileName);
			assert_eq!(result.resourceCount as usize, result.resourceEntries.len());
			assert_ne!(String::default(), result.resourceEntries[0].name);
		}
	}
}
