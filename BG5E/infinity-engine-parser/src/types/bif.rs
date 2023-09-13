#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use anyhow::{Result, Context};
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use crate::bits::ReadValue;
use crate::{readBytes, readString};
use crate::types::util::{Identity, InfinityEngineType};

const BIFCV1_Signature: &str = "BIF ";
const BIFCV1_Version: &str = "V1.0";
const BIFFV1_Signature: &str = "BIFF";
const BIFFV1_Version: &str = "V1  ";

/**
The parsed metadata and decompressed data of a BIFC V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

This file format is comprised of a header section, containing metadata about
the BIF file, and the compressed data of a standard BIFF V1 file.

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('BIF ')
0x0004 | 4 | Version ('V1.0')
0x0008 | 4 | Length of filename
0x000c | variable | Filename (length specified by previous field)
0x000c + sizeof(filename) | 4 | Uncompressed data length
0x0010 + sizeof(filename) | 4 | Compressed data length
0x0014 + sizeof(filename) | variable | Compressed data
*/
#[derive(Debug, Default, Clone)]
pub struct Bifc
{
	pub identity: Identity,
	pub fileNameLength: u32,
	pub fileName: String,
	pub uncompressedLength: u32,
	pub compressedLength: u32,
	pub compressedData: Vec<u8>,
}

impl InfinityEngineType for Bifc
{
	type Output = Bifc;
	
	fn fromCursor<T>(cursor: &mut Cursor<Vec<u8>>) -> Result<Self::Output>
		where T: InfinityEngineType
	{
		let identity = Identity::fromCursor(cursor)
			.context("Failed to read BIFC Identity")?;
		let fileNameLength = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC file name length")?;
		
		let fileNameBytes = readBytes!(cursor, fileNameLength - 1);
		let fileName = readString!(fileNameBytes);
		
		//Account for not reading the NUL in the file name
		cursor.set_position(cursor.position() + 1);
		let uncompressedLength = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC uncompressed length")?;
		let compressedLength = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC compressed length")?;
		let compressedData = readBytes!(cursor, compressedLength);
		
		return Ok(Self
		{
			identity,
			fileNameLength,
			fileName,
			uncompressedLength,
			compressedLength,
			compressedData,
		});
	}
}

impl Bifc
{
	/**
	Decompress and parse this `Bifc`'s compressed data into a fully parsed `Bif`
	instance.
	*/
	pub fn toBif(&self) -> Result<Bif>
	{
		let mut decompressedData = vec![];
		let mut decoder = ZlibDecoder::new(self.compressedData.as_slice());
		decoder.read_to_end(&mut decompressedData)
			.context("Failed to decode BIFC compressed data")?;
		
		let mut bifCursor = Cursor::new(decompressedData);
		return Bif::fromCursor::<Bif>(&mut bifCursor);
	}
}

// --------------------------------------------------

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
#[derive(Debug, Default, Clone)]
pub struct Bif
{
	pub identity: Identity,
	pub fileCount: u32,
	pub tilesetCount: u32,
	pub offset: u32,
	pub fileEntries: Vec<FileEntry>,
	pub tilesetEntries: Vec<TilesetEntry>,
}

impl InfinityEngineType for Bif
{
	type Output = Bif;
	
	fn fromCursor<T>(cursor: &mut Cursor<Vec<u8>>) -> Result<Self::Output>
		where T: InfinityEngineType
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
#[derive(Debug, Default, Clone)]
pub struct FileEntry
{
	pub locator: u32,
	pub offset: u32,
	pub size: u32,
	pub r#type: u16,
	pub unknown: u16,
}

const FileEntryIndex_MaskBits: u32 = 14;

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
		});
	}
	
	pub fn index(&self) -> u32
	{
		return ReadValue(self.locator, TilesetEntryIndex_MaskBits, TilesetEntryIndex_Shift);
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
#[derive(Debug, Default, Clone)]
pub struct TilesetEntry
{
	pub locator: u32,
	pub offset: u32,
	pub tileCount: u32,
	pub tileSize: u32,
	pub r#type: u16,
	pub unknown: u16,
}

const TilesetEntryIndex_MaskBits: u32 = 6;
const TilesetEntryIndex_Shift: u32 = 14;

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
		});
	}
	
	pub fn index(&self) -> u32
	{
		return ReadValue(self.locator, TilesetEntryIndex_MaskBits, TilesetEntryIndex_Shift);
	}
}

/// 0x0001
pub const ResourceType_BMP: u16 = 1;
/// 0x0002
pub const ResourceType_MVE: u16 = 2;
/// 0x0004
pub const ResourceType_WAV: u16 = 4;
/// 0x0004
pub const ResourceType_WAVC: u16 = 4;
/// 0x0005
pub const ResourceType_WFX: u16 = 5;
/// 0x0006
pub const ResourceType_PLT: u16 = 6;
/// 0x03e8
pub const ResourceType_BAM: u16 = 1000;
/// 0x03e8
pub const ResourceType_BAMC: u16 = 1000;
/// 0x03e9
pub const ResourceType_WED: u16 = 1001;
/// 0x03ea
pub const ResourceType_CHU: u16 = 1002;
/// 0x03eb
pub const ResourceType_TIS: u16 = 1003;
/// 0x03ec
pub const ResourceType_MOS: u16 = 1004;
/// 0x03ec
pub const ResourceType_MOSC: u16 = 1004;
/// 0x03ed
pub const ResourceType_ITM: u16 = 1005;
/// 0x03ee
pub const ResourceType_SPL: u16 = 1006;
/// 0x03ef
pub const ResourceType_BCS: u16 = 1007;
/// 0x03f0
pub const ResourceType_IDS: u16 = 1008;
/// 0x03f1
pub const ResourceType_CRE: u16 = 1009;
/// 0x03f2
pub const ResourceType_ARE: u16 = 1010;
/// 0x03f3
pub const ResourceType_DLG: u16 = 1011;
/// 0x03f4
pub const ResourceType_TwoDA: u16 = 1012;
/// 0x03f5
pub const ResourceType_GAM: u16 = 1013;
/// 0x03f6
pub const ResourceType_STO: u16 = 1014;
/// 0x03f7
pub const ResourceType_WMP: u16 = 1015;
/// 0x03f8
pub const ResourceType_CHR: u16 = 1016;
/// 0x03f8
pub const ResourceType_EFF: u16 = 1016;
/// 0x03f9
pub const ResourceType_BS: u16 = 1017;
/// 0x03fa
pub const ResourceType_CHR2: u16 = 1018;
/// 0x03fb
pub const ResourceType_VVC: u16 = 1019;
/// 0x03fc
pub const ResourceType_VEF: u16 = 1020;
/// 0x03fd
pub const ResourceType_PRO: u16 = 1021;
/// 0x03fe
pub const ResourceType_BIO: u16 = 1022;
/// 0x03ff
pub const ResourceType_WBM: u16 = 1023;
/// 0x0400
pub const ResourceType_FNT: u16 = 1024;
/// 0x0402
pub const ResourceType_GUI: u16 = 1026;
/// 0x0403
pub const ResourceType_SQL: u16 = 1027;
/// 0x0404
pub const ResourceType_PVRZ: u16 = 1028;
/// 0x0405
pub const ResourceType_GLSL: u16 = 1029;
/// 0x0408
pub const ResourceType_MENU: u16 = 1032;
/// 0x0409
pub const ResourceType_MENU2: u16 = 1033;
/// 0x040a
pub const ResourceType_TTF: u16 = 1034;
/// 0x040b
pub const ResourceType_PNG: u16 = 1035;
/// 0x044c
pub const ResourceType_BAH: u16 = 1100;
/// 0x0802
pub const ResourceType_INI: u16 = 2050;
/// 0x0803
pub const ResourceType_SRC: u16 = 2051;

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
		let keyFile = KeyFileName(&Games::BaldursGate1).unwrap();
		let filePath = Path::new(installPath.as_str()).join(keyFile);
		
		let key = ReadFromFile::<Key>(filePath.as_path()).unwrap();
		let bifFileName = key.bifEntries[0].fileName.clone();
		
		assert_eq!("data\\Default.bif", bifFileName);
		
		let bifPath = Path::new(installPath.as_str()).join(bifFileName);
		let result = ReadFromFile::<Bif>(bifPath.as_path()).unwrap();
		
		assert_eq!(BIFFV1_Signature, result.identity.signature);
		assert_eq!(BIFFV1_Version, result.identity.version);
		assert_eq!(181, result.fileCount);
		assert_eq!(0, result.tilesetCount);
		assert_eq!(20, result.offset);
		assert_eq!(result.fileCount as usize, result.fileEntries.len());
		assert_eq!(result.tilesetCount as usize, result.tilesetEntries.len());
	}
	
	#[test]
	fn BifcTest()
	{
		let fileName = "CD2/Data/AR100A.cbf";
		let installPath = FindInstallationPath(Games::IcewindDale1).unwrap();
		let filePath = Path::new(installPath.as_str()).join(fileName);
		
		let bifc = ReadFromFile::<Bifc>(filePath.as_path()).unwrap();
		assert_eq!(BIFCV1_Signature, bifc.identity.signature);
		assert_eq!(BIFCV1_Version, bifc.identity.version);
		assert_eq!(11, bifc.fileNameLength);
		// The NUL is dropped when reading
		assert_eq!((bifc.fileNameLength - 1) as usize, bifc.fileName.len());
		assert_eq!(6613876, bifc.uncompressedLength);
		assert_eq!(3322859, bifc.compressedLength);
		assert_eq!(bifc.compressedLength as usize, bifc.compressedData.len());
		
		let bif = bifc.toBif().unwrap();
		assert_eq!(BIFFV1_Signature, bif.identity.signature);
		assert_eq!(BIFFV1_Version, bif.identity.version);
		assert_eq!(20, bif.offset);
		assert_eq!(bif.fileCount as usize, bif.fileEntries.len());
		assert_eq!(bif.tilesetCount as usize, bif.tilesetEntries.len());
	}
}
