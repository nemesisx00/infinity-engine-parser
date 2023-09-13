#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use anyhow::{Result, Context};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{readBytes, readString};
use crate::types::util::InfinityEngineType;
const Type: &str = "BM";

const BPP_1bit: u16 = 1;
const BPP_4bit: u16 = 4;
const BPP_8bit: u16 = 8;
const BPP_16bit: u16 = 16;
const BPP_24bit: u16 = 24;

pub enum BPP
{
	Monochrome,
	Palletized4bit,
	Palletized8bit,
	Rgb16bit,
	Rgb24bit,
}

/**
The fully parsed contents of a BMP file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bmp.htm

This file format is the MS-Windows standard format. It holds black & white,
16-color, and 256-color images which may be compressed via run length encoding.
Notice there is also an OS/2-BMP format.

---

Offset | Name | Size | Description
---|---|---|---
0x00 | FileHeader | 14 | Windows Structure: BITMAPFILEHEADER
FileHeader size | InfoHeader | 40 | Windows Structure: BITMAPINFOHEADER
FileHeader size + InfoHeader size | RasterData | variable | The pixel data
*/
#[derive(Debug, Clone)]
pub struct Bmp
{
	pub file: BmpFile,
	pub info: BmpInfo,
	pub colors: Vec<u32>,
	pub decoded: Vec<u8>,
}

impl InfinityEngineType for Bmp
{
	type Output = Bmp;
	
	fn fromCursor<T>(cursor: &mut Cursor<Vec<u8>>) -> Result<Self::Output>
		where T: InfinityEngineType
	{
		let file = BmpFile::fromCursor(cursor)
			.context("Failed to read BMP file header")?;
		let info = BmpInfo::fromCursor(cursor)
			.context("Failed to read BMP info header")?;
		
		//Read the Color Table colors, if necessary
		let mut colors = vec![];
		if info.bitsPerPixel == BPP_1bit || info.bitsPerPixel == BPP_4bit || info.bitsPerPixel == BPP_8bit
		{
			let count = match info.colorsUsed
			{
				0 => 1 << info.bitsPerPixel,
				_ => info.colorsUsed,
			};
			
			for _ in 0..count
			{
				let color = cursor.read_u32::<LittleEndian>()
					.context("Failed to read BMP color for color table")?;
				colors.push(color);
			}
		}
		
		let mut encoded = vec![];
		cursor.read_to_end(&mut encoded)
			.context("Failed to read BMP encoded pixel data")?;
		
		let mut decoded = vec![];
		//This only works for 24-bit but good enough for a test
		decoded.append(&mut encoded);
		
		return Ok(Self
		{
			file,
			info,
			colors,
			decoded,
		});
	}
}

impl Bmp
{
	pub fn toBytes(&self) -> Vec<u8>
	{
		let mut bytes = vec![];
		bytes.append(self.file.toBytes().as_mut());
		bytes.append(self.info.toBytes().as_mut());
		
		for color in self.colors.clone()
		{
			bytes.append(color.to_le_bytes().to_vec().as_mut());
		}
		
		bytes.append(self.decoded.to_vec().as_mut());
		
		return bytes;
	}
}

// --------------------------------------------------

/**
The contents of a BMP file's FileHeader.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bmp.htm

---

Offset | Name | Size | Description
---|---|---|---
0x00 | Signature | 2 | Signature ('BM')
0x02 | FileSize | 4 | File size in bytes
0x06 | Reserved | 4 | Reserved space - unused
0x0a | DataOffset | 4 | File offset to Raster Data
*/
#[derive(Debug, Clone)]
pub struct BmpFile
{
	pub r#type: String,
	pub size: u32,
	pub reserved: u32,
	pub offset: u32,
}

impl BmpFile
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let typeBytes = readBytes!(cursor, Type.len());
		let r#type = readString!(typeBytes);
		let size = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP FileHeader size")?;
		let reserved = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP FileHeader reserved")?;
		let offset = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP FileHeader data offset")?;
		
		return Ok(Self
		{
			r#type,
			size,
			reserved,
			offset,
		});
	}
	
	pub fn toBytes(&self) -> Vec<u8>
	{
		let mut bytes = vec![];
		bytes.append(self.r#type.as_bytes().to_vec().as_mut());
		bytes.append(self.size.to_le_bytes().to_vec().as_mut());
		bytes.append(self.reserved.to_le_bytes().to_vec().as_mut());
		bytes.append(self.offset.to_le_bytes().to_vec().as_mut());
		return bytes;
	}
}

// --------------------------------------------------

/**
The contents of a BMP file's InfoHeader.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bmp.htm

---

Offset | Name | Size | Description
---|---|---|---
0x0e | Size | 4 | Size of InfoHeader - 40
0x12 | Width | 4 | Bitmap Width
0x16 | Height | 4 | Bitmap Height
0x1a | Planes | 2 | Number of Planes
0x1c | BitCount | 2 | Bits per Pixel
0x1e | Compression | 4 | Type of Compression
0x22 | ImageSize | 4 | Size of the image
0x26 | XpixelsPerM | 4 | Horizontal resolution: pixels/meter
0x2a | YpixelsPerM | 4 | Vertical resolution: pixels/meter
0x2e | ColorsUsed | 4 | Number of actually used colors
0x32 | ColorsImportant | 4 | Number of important colors (0 = all)
0x36 | ColorTable | variable | 4 bytes * ColorsUsed value
*/
#[derive(Debug, Clone)]
pub struct BmpInfo
{
	pub size: u32,
	pub width: i32,
	pub height: i32,
	pub planes: u16,
	pub bitsPerPixel: u16,
	pub compression: u32,
	pub compressedSize: u32,
	pub resolutionHorizontal: i32,
	pub resolutionVertical: i32,
	pub colorsUsed: u32,
	pub colorsImportant: u32,
}

impl BmpInfo
{
	pub fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let size = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader size")?;
		let width = cursor.read_i32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader width")?;
		let height = cursor.read_i32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader height")?;
		let planes = cursor.read_u16::<LittleEndian>()
			.context("Failed to read BMP InfoHeader planes")?;
		let bitsPerPixel = cursor.read_u16::<LittleEndian>()
			.context("Failed to read BMP InfoHeader bits per pixel")?;
		let compression = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader compression")?;
		let compressedSize = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader compressed size")?;
		let resolutionHorizontal = cursor.read_i32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader resolution horizontal")?;
		let resolutionVertical = cursor.read_i32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader resolution vertical")?;
		let colorsUsed = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader colors used")?;
		let colorsImportant = cursor.read_u32::<LittleEndian>()
			.context("Failed to read BMP InfoHeader colors important")?;
		
		return Ok(Self
		{
			size,
			width,
			height,
			planes,
			bitsPerPixel,
			compression,
			compressedSize,
			resolutionHorizontal,
			resolutionVertical,
			colorsUsed,
			colorsImportant,
		});
	}
	
	pub fn toBytes(&self) -> Vec<u8>
	{
		let mut bytes = vec![];
		bytes.append(self.size.to_le_bytes().to_vec().as_mut());
		bytes.append(self.width.to_le_bytes().to_vec().as_mut());
		bytes.append(self.height.to_le_bytes().to_vec().as_mut());
		bytes.append(self.planes.to_le_bytes().to_vec().as_mut());
		bytes.append(self.bitsPerPixel.to_le_bytes().to_vec().as_mut());
		bytes.append(self.compression.to_le_bytes().to_vec().as_mut());
		bytes.append(self.compressedSize.to_le_bytes().to_vec().as_mut());
		bytes.append(self.resolutionHorizontal.to_le_bytes().to_vec().as_mut());
		bytes.append(self.resolutionVertical.to_le_bytes().to_vec().as_mut());
		bytes.append(self.colorsUsed.to_le_bytes().to_vec().as_mut());
		bytes.append(self.colorsImportant.to_le_bytes().to_vec().as_mut());
		return bytes;
	}
}

#[cfg(test)]
mod tests
{
	#[allow(unused_imports)]
	use std::fs::File;
	#[allow(unused_imports)]
	use std::io::Write;
	use std::path::Path;
    use super::*;
	use crate::platform::{FindInstallationPath, Games, KeyFileName};
	use crate::types::{Bif, Key};
	use crate::types::util::ReadFromFile;
	
	#[test]
	fn BmpTest()
	{
		let resourceName = "AJANTISG";
		//TODO: Make this test not rely on actually reading a file from the file system.
		let installPath = FindInstallationPath(Games::BaldursGate1).unwrap();
		let keyFile = KeyFileName(&Games::BaldursGate1).unwrap();
		let filePath = Path::new(installPath.as_str()).join(keyFile);
		
		let key = ReadFromFile::<Key>(filePath.as_path()).unwrap();
		let resourceEntry = key.resourceEntries
			.iter()
			.find(|entry| entry.name.eq(resourceName))
			.unwrap();
		
		let bifEntry = &key.bifEntries[resourceEntry.indexBifEntry() as usize];
		let bifFileName = bifEntry.fileName.clone();
		let bifPath = Path::new(installPath.as_str()).join(bifFileName);
		let bif = ReadFromFile::<Bif>(bifPath.as_path()).unwrap();
		
		let fileEntry = bif.fileEntries
			.iter()
			.find(|entry| entry.index() == resourceEntry.indexFile())
			.unwrap();
		
		let mut cursor = Cursor::new(fileEntry.data.clone());
		let bmp = Bmp::fromCursor::<Bmp>(&mut cursor).unwrap();
		
		assert_eq!(Type, bmp.file.r#type);
		assert_eq!(14, bmp.file.toBytes().len());
		assert_eq!(bmp.info.size as usize, bmp.info.toBytes().len());
		assert_eq!(210, bmp.info.width);
		assert_eq!(330, bmp.info.height);
		assert_eq!(BPP_24bit, bmp.info.bitsPerPixel);
		assert_eq!(0, bmp.colors.len());
		
		//Verify with eyes
		/*
		let outPath = Path::new("../../target").join("Ajantis.bmp");
		let mut file = File::create(outPath.as_path()).expect("Output file couldn't be created");
		let result = file.write_all(bmp.toBytes().as_slice());
		assert!(result.is_ok());
		// */
	}
}
