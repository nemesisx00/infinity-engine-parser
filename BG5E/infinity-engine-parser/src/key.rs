#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs;
use std::io;
use crate::bits::ReadValue;
use crate::readString;

const FileName: &str = "chitin.key";

const Signature: &str = "KEY ";
const Version: &str = "V1  ";

const ResourceLocator_BifEntry: u32 = 12;
const ResourceLocator_File: u32 = 14;
const ResourceLocator_Tileset: u32 = 6;

pub fn ReadKey(path: String) -> io::Result<Key>
{
	let buffer = fs::read(path)?;
	
	// parse
	let sig = readString!(buffer[0..4]);
	let ver = readString!(buffer[4..8]);
	let bec = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
	let rec = u32::from_le_bytes(buffer[12..16].try_into().unwrap());
	let boffset = u32::from_le_bytes(buffer[16..20].try_into().unwrap());
	let roffset = u32::from_le_bytes(buffer[20..24].try_into().unwrap());
	
	return Ok(Key {
		signature: sig,
		version: ver,
		bifEntryCount: bec,
		resourceEntryCount: rec,
		bifOffset: boffset,
		resourceOffset: roffset,
		..Default::default()
	});
}

/**
The fully parsed contents of a KEY V1 file.
 
See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm

This file format acts as a central reference point to locate files required
by the game (in a BIFF file on a CD or in the override directory). The key
e also maintains a mapping from an 8 byte resource name (resref) to a 32
byte ID (using the lowest 12 bits to identify a resource). There is generally
only one key file with each game (chitin.key).

---

Offset | Size | Description
--- | --- | ---
0x0000 | 4 | Signature ('KEY ')
0x0004 | 4 | Version ('V1  ')
0x0008 | 4 | Count of BIF entries
0x000c | 4 | Count of resource entries
0x0010 | 4 | Offset (from start of file) to BIF entries
0x0014 | 4 | Offset (from start of file) to resource entries
*/
#[derive(Debug, Default, Clone)]
pub struct Key
{
	pub signature: String,
	pub version: String,
	pub bifEntryCount: u32,
	pub resourceEntryCount: u32,
	pub bifOffset: u32,
	pub resourceOffset: u32,
	pub bifEntries: Vec<BifEntry>,
	pub resourceEntries: Vec<ResourceEntry>,
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
#[derive(Debug, Clone)]
pub struct BifEntry
{
	pub fileName: String,
	pub fileLength: u32,
	pub fileNameOffset: u32,
	pub fileNameLength: u16,
	pub locator: u16,
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
#[derive(Clone, Debug)]
pub struct ResourceEntry
{
	pub name: String,
	pub r#type: u16,
	pub locator: u32,
}

impl ResourceEntry
{
	pub fn IndexFile(&self) -> u32
	{
		return ReadValue(self.locator, ResourceLocator_File, 0);
	}
	
	pub fn IndexTileset(&self) -> u32
	{
		return ReadValue(self.locator, ResourceLocator_Tileset, ResourceLocator_File);
	}
	
	pub fn IndexBifEntry(&self) -> u32
	{
		return ReadValue(self.locator, ResourceLocator_BifEntry, ResourceLocator_File + ResourceLocator_Tileset);
	}
}

#[cfg(test)]
mod tests
{
	use std::path::Path;
    use super::*;
	use crate::game::{FindInstallationPath, Games, KeyFileName};
	
    #[test]
    fn LocatorTest()
	{
		let locator = 0xF00028;
		
		let fileExpected = 40;
		let tileExpected = 0;
		let bifExpected = 15;
		
		let instance = ResourceEntry { name: String::default(), r#type: 0, locator };
		
		assert_eq!(fileExpected, instance.IndexFile());
		assert_eq!(tileExpected, instance.IndexTileset());
		assert_eq!(bifExpected, instance.IndexBifEntry());
    }
	
	#[test]
	fn ReadKeyTest()
	{
		let installPath = FindInstallationPath(Games::BaldursGate1).unwrap();
		let keyFile = KeyFileName(&Games::BaldursGate1).unwrap();
		let filePath = Path::new(installPath.as_str()).join(keyFile);
		
		let result = ReadKey(filePath.to_str().unwrap().to_string()).unwrap();
		
		assert_eq!("KEY ", result.signature);
		assert_eq!("V1  ", result.version);
		assert_eq!(159, result.bifEntryCount);
		assert_eq!(16694, result.resourceEntryCount);
		assert_eq!(24, result.bifOffset);
		assert_eq!(4780, result.resourceOffset);
	}
}
