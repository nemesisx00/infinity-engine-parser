use std::io::{Cursor, Read};
use ::anyhow::{Result, Context};
use ::byteorder::{LittleEndian, ReadBytesExt};
use ::flate2::read::ZlibDecoder;
use crate::{readBytes, parseString};
use crate::types::util::{Identity, InfinityEngineType, Readable};
use super::Bif;

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

impl Bifc
{
	pub const Signature: &'static str = "BIF ";
	pub const Version: &'static str = "V1.0";

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
		return Bif::fromCursor(&mut bifCursor);
	}
}

impl InfinityEngineType for Bifc {}

impl Readable for Bifc
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let identity = Identity::fromCursor(cursor)
			.context("Failed to read BIFC Identity")?;
		let fileNameLength = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BIFC file name length")?;
		
		let fileNameBytes = readBytes!(cursor, fileNameLength - 1);
		let fileName = parseString!(fileNameBytes);
		
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

#[cfg(test)]
mod tests
{
	use std::path::Path;
    use super::*;
	use crate::platform::{FindInstallationPath, Games};
	use crate::types::util::ReadFromFile;
	
	#[test]
	fn BifcTest()
	{
		let fileName = "CD2/Data/AR100A.cbf";
		let installPath = FindInstallationPath(Games::IcewindDale1).unwrap();
		let filePath = Path::new(installPath.as_str()).join(fileName);
		
		let bifc = ReadFromFile::<Bifc>(filePath.as_path()).unwrap();
		assert_eq!(Bifc::Signature, bifc.identity.signature);
		assert_eq!(Bifc::Version, bifc.identity.version);
		assert_eq!(11, bifc.fileNameLength);
		// The NUL is dropped when reading
		assert_eq!((bifc.fileNameLength - 1) as usize, bifc.fileName.len());
		assert_eq!(6613876, bifc.uncompressedLength);
		assert_eq!(3322859, bifc.compressedLength);
		assert_eq!(bifc.compressedLength as usize, bifc.compressedData.len());
		
		let bif = bifc.toBif().unwrap();
		assert_eq!(Bif::Signature, bif.identity.signature);
		assert_eq!(Bif::Version, bif.identity.version);
		assert_eq!(20, bif.offset);
		assert_eq!(bif.fileCount as usize, bif.fileEntries.len());
		assert_eq!(bif.tilesetCount as usize, bif.tilesetEntries.len());
	}
}
