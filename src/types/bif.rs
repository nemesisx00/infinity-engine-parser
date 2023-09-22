#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use ::anyhow::{Result, Context};
use ::byteorder::{LittleEndian, ReadBytesExt};
use ::flate2::read::ZlibDecoder;
use crate::{readBytes, readString};
use crate::bits::ReadValue;
use super::util::{Identity, InfinityEngineType};
use super::Tis;

const BIFC_Signature: &str = "BIF ";
const BIFC_Version: &str = "V1.0";
const BIFCC_Signature: &str = "BIFC";
const BIFCC_Version: &str = "V1.0";
const BIFF_Signature: &str = "BIFF";
const BIFF_Version: &str = "V1  ";

/**
The parsed metadata and decompressed data of a BIFC V1 file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

This file format is comprised of a header section, containing metadata about
the BIF file, and the compressed data of a standard BIFF V1 file.

---

### Header Data

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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
The parsed metadata and decompressed data of a BIFC V1.0 (compressed) file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

This file format is comprised of a header section, containing metadata about
the BIF file, and a set of blocks containing compressed data which, when
decompressed and combined, amount to a BIFF V1 file.

---

### Header Data

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('BIFC')
0x0004 | 4 | Version ('V1.0')
0x0008 | 4 | Uncompressed BIF size
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bifcc
{
	pub identity: Identity,
	pub uncompressedSize: u32,
	pub blocks: Vec<BifccBlock>,
}

impl InfinityEngineType for Bifcc
{
	type Output = Bifcc;
	
	fn fromCursor<T>(cursor: &mut Cursor<Vec<u8>>) -> Result<Self::Output>
		where T: InfinityEngineType
	{
		let identity = Identity::fromCursor(cursor)
			.context("Failed to read BIFC Compressed identity")?;
		let uncompressedSize = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC Compressed uncompressed size")?;
		
		let mut blocks = vec![];
		while cursor.position() < cursor.get_ref().len() as u64
		{
			let block = BifccBlock::fromCursor(cursor)
				.context("Failed to read BIFC Compressed Block")?;
			blocks.push(block);
		}
		
		return Ok(Self{
			identity,
			uncompressedSize,
			blocks,
		});
	}
}

impl Bifcc
{
	pub fn toBif(&self) -> Result<Bif>
	{
		let mut decompressedData = vec![];
		
		for block in self.blocks.clone()
		{
			let mut data = vec![];
			let mut decoder = ZlibDecoder::new(block.compressedData.as_slice());
			decoder.read_to_end(&mut data)
				.context("Failed to decode BIFC Compressed Block compressed data")?;
			
			decompressedData.append(&mut data);
		}
		
		let mut bifCursor = Cursor::new(decompressedData);
		return Bif::fromCursor::<Bif>(&mut bifCursor);
	}
}

/**
Metadata defining the contents of a BIFC V1.0 (compressed) compressed data block.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Decompressed size
0x0004 | 4 | Compressed size
0x0008 | variable | Compressed data
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BifccBlock
{
	pub decompressedSize: u32,
	pub compressedSize: u32,
	pub compressedData: Vec<u8>,
}

impl BifccBlock
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let decompressedSize = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC Compressed Block decompressed size")?;
		let compressedSize = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC Compressed Block compressed size")?;
		let compressedData = readBytes!(cursor, compressedSize);
		
		return Ok(Self
		{
			decompressedSize,
			compressedSize,
			compressedData,
		});
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
		
		for mut entry in fileEntries.as_mut_slice()
		{
			cursor.set_position(entry.offset as u64);
			let bytes = readBytes!(cursor, entry.size);
			entry.data = bytes;
		}
		
		for mut entry in tilesetEntries.as_mut_slice()
		{
			cursor.set_position(entry.offset as u64);
			let mut tis = Tis::default();
			tis.readData(cursor, entry.tileCount)?;
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
			..Default::default()
		});
	}
	
	pub fn index(&self) -> u32
	{
		return ReadValue(self.locator, FileEntryIndex_MaskBits, 0);
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
			..Default::default()
		});
	}
	
	pub fn index(&self) -> u32
	{
		return ReadValue(self.locator, TilesetEntryIndex_MaskBits, TilesetEntryIndex_Shift);
	}
}

/// 0x0001
pub const ResourceType_BMP: i16 = 1;
/// 0x0002
pub const ResourceType_MVE: i16 = 2;
/// 0x0004
pub const ResourceType_WAV: i16 = 4;
/// 0x0004
pub const ResourceType_WAVC: i16 = 4;
/// 0x0005
pub const ResourceType_WFX: i16 = 5;
/// 0x0006
pub const ResourceType_PLT: i16 = 6;
/// 0x03e8
pub const ResourceType_BAM: i16 = 1000;
/// 0x03e8
pub const ResourceType_BAMC: i16 = 1000;
/// 0x03e9
pub const ResourceType_WED: i16 = 1001;
/// 0x03ea
pub const ResourceType_CHU: i16 = 1002;
/// 0x03eb
pub const ResourceType_TIS: i16 = 1003;
/// 0x03ec
pub const ResourceType_MOS: i16 = 1004;
/// 0x03ec
pub const ResourceType_MOSC: i16 = 1004;
/// 0x03ed
pub const ResourceType_ITM: i16 = 1005;
/// 0x03ee
pub const ResourceType_SPL: i16 = 1006;
/// 0x03ef
pub const ResourceType_BCS: i16 = 1007;
/// 0x03f0
pub const ResourceType_IDS: i16 = 1008;
/// 0x03f1
pub const ResourceType_CRE: i16 = 1009;
/// 0x03f2
pub const ResourceType_ARE: i16 = 1010;
/// 0x03f3
pub const ResourceType_DLG: i16 = 1011;
/// 0x03f4
pub const ResourceType_TwoDA: i16 = 1012;
/// 0x03f5
pub const ResourceType_GAM: i16 = 1013;
/// 0x03f6
pub const ResourceType_STO: i16 = 1014;
/// 0x03f7
pub const ResourceType_WMP: i16 = 1015;
/// 0x03f8
pub const ResourceType_CHR: i16 = 1016;
/// 0x03f8
pub const ResourceType_EFF: i16 = 1016;
/// 0x03f9
pub const ResourceType_BS: i16 = 1017;
/// 0x03fa
pub const ResourceType_CHR2: i16 = 1018;
/// 0x03fb
pub const ResourceType_VVC: i16 = 1019;
/// 0x03fc
pub const ResourceType_VEF: i16 = 1020;
/// 0x03fd
pub const ResourceType_PRO: i16 = 1021;
/// 0x03fe
pub const ResourceType_BIO: i16 = 1022;
/// 0x03ff
pub const ResourceType_WBM: i16 = 1023;
/// 0x0400
pub const ResourceType_FNT: i16 = 1024;
/// 0x0402
pub const ResourceType_GUI: i16 = 1026;
/// 0x0403
pub const ResourceType_SQL: i16 = 1027;
/// 0x0404
pub const ResourceType_PVRZ: i16 = 1028;
/// 0x0405
pub const ResourceType_GLSL: i16 = 1029;
/// 0x0408
pub const ResourceType_MENU: i16 = 1032;
/// 0x0409
pub const ResourceType_MENU2: i16 = 1033;
/// 0x040a
pub const ResourceType_TTF: i16 = 1034;
/// 0x040b
pub const ResourceType_PNG: i16 = 1035;
/// 0x044c
pub const ResourceType_BAH: i16 = 1100;
/// 0x0802
pub const ResourceType_INI: i16 = 2050;
/// 0x0803
pub const ResourceType_SRC: i16 = 2051;

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
		
		assert_eq!(BIFF_Signature, result.identity.signature);
		assert_eq!(BIFF_Version, result.identity.version);
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
		assert_eq!(BIFC_Signature, bifc.identity.signature);
		assert_eq!(BIFC_Version, bifc.identity.version);
		assert_eq!(11, bifc.fileNameLength);
		// The NUL is dropped when reading
		assert_eq!((bifc.fileNameLength - 1) as usize, bifc.fileName.len());
		assert_eq!(6613876, bifc.uncompressedLength);
		assert_eq!(3322859, bifc.compressedLength);
		assert_eq!(bifc.compressedLength as usize, bifc.compressedData.len());
		
		let bif = bifc.toBif().unwrap();
		assert_eq!(BIFF_Signature, bif.identity.signature);
		assert_eq!(BIFF_Version, bif.identity.version);
		assert_eq!(20, bif.offset);
		assert_eq!(bif.fileCount as usize, bif.fileEntries.len());
		assert_eq!(bif.tilesetCount as usize, bif.tilesetEntries.len());
	}
	
	#[test]
	fn BifccTest()
	{
		let fileName = "data/Data/AREA000A.bif";
		let installPath = FindInstallationPath(Games::BaldursGate2).unwrap();
		let filePath = Path::new(installPath.as_str()).join(fileName);
		
		let bifcc = ReadFromFile::<Bifcc>(filePath.as_path()).unwrap();
		assert_eq!(BIFCC_Signature, bifcc.identity.signature);
		assert_eq!(BIFCC_Version, bifcc.identity.version);
		assert_eq!(27729850, bifcc.uncompressedSize);
		assert_ne!(0 as usize, bifcc.blocks.len());
		
		let bif = bifcc.toBif().unwrap();
		assert_eq!(BIFF_Signature, bif.identity.signature);
		assert_eq!(BIFF_Version, bif.identity.version);
		assert_eq!(20, bif.offset);
		assert_eq!(bif.fileCount as usize, bif.fileEntries.len());
		assert_eq!(bif.tilesetCount as usize, bif.tilesetEntries.len());
	}
}