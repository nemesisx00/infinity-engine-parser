#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::{Result, Context};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::readBytes;
use crate::bits::ReadValue;
use crate::types::Tis;
use crate::types::util::{Identity, InfinityEngineType, Readable, ReadIntoSelf};

/**
The fully parsed metadata contents of a BIFF V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

This file format is a simple archive format, used mainly both to simplify
organization of the files by grouping logically related files together
(especially for areas). There is also a gain from having few large files rather
than many small files, due to the wastage in the FAT and NTFS file systems. BIF
files containing areas typically contain:

- One ore more WED files, detailing tiles and wallgroups
- One or more TIS files, containing the tileset itself
- One or more MOS files, containing the minimap graphic
- Three or four bitmap files which contain one pixel for each tile needed to
cover the region
	- **xxxxxxHT.BMP** - Height map, detailing altitude of each tile cell in the
	associated WED file
	- **xxxxxxLM.BMP** - Light map, detailing the level and color of illumination
	for each tile cell on the map. Used during daytime
	- **xxxxxxLN.BMP** - Light map, detailing the level and color of illumination
	for each tile cell on the map. Used during nighttime
	- **xxxxxxSR.BMP** - Search Map, detailing where characters cannot walk and
	the footstep sounds

---

### Header Data

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('BIFF')
0x0004 | 4 | Version ('V1  ')
0x0008 | 4 | Count of file entries
0x000c | 4 | Count of tileset entries
0x0010 | 4 | Offset (from start of file) to file entries
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bif
{
	pub identity: Identity,
	pub fileCount: u32,
	pub tilesetCount: u32,
	pub offset: u32,
	pub fileEntries: Vec<FileEntry>,
	pub tilesetEntries: Vec<TilesetEntry>,
}

impl Bif
{
	pub const Signature: &str = "BIFF";
	pub const Version: &str = "V1  ";
}

impl InfinityEngineType for Bif {}

impl Readable for Bif
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let identity = Identity::fromCursor(cursor)
			.context("Failed to read BIFF identity")?;
		let fileCount = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF file count")?;
		let tilesetCount = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF tileset count")?;
		let offset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF offset")?;
		
		let mut fileEntries = vec![];
		for i in 0..fileCount
		{
			let entry = FileEntry::fromCursor(cursor)
				.context(format!("Failed to parse file entry #{}", i))?;
			fileEntries.push(entry);
		}
		
		let mut tilesetEntries = vec![];
		for i in 0..tilesetCount
		{
			let entry = TilesetEntry::fromCursor(cursor)
				.context(format!("Failed to parse tileset entry #{}", i))?;
			tilesetEntries.push(entry);
		}
		
		for mut entry in fileEntries.as_mut_slice()
		{
			cursor.set_position(entry.offset as u64);
			let bytes = readBytes!(cursor, entry.size);
			entry.data = bytes;
		}
		
		for mut entry in tilesetEntries.as_mut_slice()
		{
			cursor.set_position(entry.offset as u64);
			let mut tis = Tis::new(entry.tileCount);
			tis.read(cursor)?;
			entry.data = Some(tis);
		}
		
		return Ok(Self
		{
			identity,
			fileCount,
			tilesetCount,
			offset,
			fileEntries,
			tilesetEntries,
		});
	}
}

// --------------------------------------------------

/**
Metadata defining the details of a file included in a given BIFF V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Resource locator. Only bits 0-13 are matched against the file index in the "resource locator" field from the KEY file resource entry.
0x0004 | 4 | Offset (from start of file) to resource data
0x0008 | 4 | Size of this resource
0x000c | 2 | Type of this resource
0x000e | 2 | Unknown
*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileEntry
{
	pub locator: u32,
	pub offset: u32,
	pub size: u32,
	pub r#type: u16,
	pub unknown: u16,
	pub data: Vec<u8>,
}

const FileEntryIndex_MaskBits: u64 = 14;

impl FileEntry
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let locator = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF FileEntry locator")?;
		let offset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF FileEntry offset")?;
		let size = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF FileEntry size")?;
		let r#type = cursor.read_u16::<LittleEndian>()
			.context("Failed to read BIFF FileEntry type")?;
		let unknown = cursor.read_u16::<LittleEndian>()
			.context("Failed to read BIFF FileEntry unknown")?;
		
		return Ok(Self
		{
			locator,
			offset,
			size,
			r#type,
			unknown,
			..Default::default()
		});
	}
	
	pub fn index(&self) -> u32
	{
		return ReadValue(self.locator.into(), FileEntryIndex_MaskBits, 0) as u32;
	}
}

// --------------------------------------------------

/**
Metadata defining the details of a tileset included in a given BIFF V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Resource locator. Only bits 14-19 are matched against the file index in the "resource locator" field from the KEY file resource entry.
0x0004 | 4 | Offset (from start of file) to resource data
0x0008 | 4 | Count of tiles in this resource
0x000c | 4 | Size of each tile in this resource
0x0010 | 2 | Type of this resource (always 0x3eb - TIS)
0x0012 | 2 | Unknown
*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TilesetEntry
{
	pub locator: u32,
	pub offset: u32,
	pub tileCount: u32,
	pub tileSize: u32,
	pub r#type: u16,
	pub unknown: u16,
	pub data: Option<Tis>,
}

const TilesetEntryIndex_MaskBits: u64 = 6;
const TilesetEntryIndex_Shift: u64 = 14;

impl TilesetEntry
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let locator = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF TilesetEntry locator")?;
		let offset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF TilesetEntry offset")?;
		let tileCount = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF TilesetEntry tile count")?;
		let tileSize = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFF TilesetEntry tile size")?;
		let r#type = cursor.read_u16::<LittleEndian>()
			.context("Failed to read BIFF TilesetEntry type")?;
		let unknown = cursor.read_u16::<LittleEndian>()
			.context("Failed to read BIFF TilesetEntry unknown")?;
		
		return Ok(Self
		{
			locator,
			offset,
			tileCount,
			tileSize,
			r#type,
			unknown,
			..Default::default()
		});
	}
	
	pub fn index(&self) -> u32
	{
		return ReadValue(self.locator.into(), TilesetEntryIndex_MaskBits, TilesetEntryIndex_Shift) as u32;
	}
}

#[cfg(test)]
mod tests
{
	use std::path::Path;
    use super::*;
	use crate::platform::{FindInstallationPath, Games, KeyFileName};
	use crate::types::Key;
	use crate::types::util::ReadFromFile;
	
	#[test]
	fn BifTest()
	{
		//TODO: Make this test not rely on actually reading a file from the file system.
		let installPath = FindInstallationPath(Games::BaldursGate1).unwrap();
		let keyFile = KeyFileName(Games::BaldursGate1).unwrap();
		let filePath = Path::new(installPath.as_str()).join(keyFile);
		
		let key = ReadFromFile::<Key>(filePath.as_path()).unwrap();
		let bifFileName = key.bifEntries[0].fileName.clone();
		
		assert_eq!("data\\Default.bif", bifFileName);
		
		let bifPath = Path::new(installPath.as_str()).join(bifFileName);
		let result = ReadFromFile::<Bif>(bifPath.as_path()).unwrap();
		
		assert_eq!(Bif::Signature, result.identity.signature);
		assert_eq!(Bif::Version, result.identity.version);
		assert_eq!(181, result.fileCount);
		assert_eq!(0, result.tilesetCount);
		assert_eq!(20, result.offset);
		assert_eq!(result.fileCount as usize, result.fileEntries.len());
		assert_eq!(result.tilesetCount as usize, result.tilesetEntries.len());
	}
}
