#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use ::anyhow::{Result, Context};
use ::byteorder::{LittleEndian, ReadBytesExt};
use ::flate2::read::ZlibDecoder;
use crate::readBytes;
use crate::types::util::{Identity, InfinityEngineType, Readable};
use super::Bif;

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

impl Bifcc
{
	pub const Signature: &str = "BIFC";
	pub const Version: &str = "V1.0";
	
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
		return Bif::fromCursor(&mut bifCursor);
	}
}

impl InfinityEngineType for Bifcc {}

impl Readable for Bifcc
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
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

#[cfg(test)]
mod tests
{
	use std::path::Path;
    use super::*;
	use crate::platform::{FindInstallationPath, Games};
	use crate::types::util::ReadFromFile;
	
	#[test]
	fn BifccTest()
	{
		let fileName = "data/Data/AREA000A.bif";
		let installPath = FindInstallationPath(Games::BaldursGate2).unwrap();
		let filePath = Path::new(installPath.as_str()).join(fileName);
		
		let bifcc = ReadFromFile::<Bifcc>(filePath.as_path()).unwrap();
		assert_eq!(Bifcc::Signature, bifcc.identity.signature);
		assert_eq!(Bifcc::Version, bifcc.identity.version);
		assert_eq!(27729850, bifcc.uncompressedSize);
		assert_ne!(0 as usize, bifcc.blocks.len());
		
		let bif = bifcc.toBif().unwrap();
		assert_eq!(Bif::Signature, bif.identity.signature);
		assert_eq!(Bif::Version, bif.identity.version);
		assert_eq!(20, bif.offset);
		assert_eq!(bif.fileCount as usize, bif.fileEntries.len());
		assert_eq!(bif.tilesetCount as usize, bif.tilesetEntries.len());
	}
}
