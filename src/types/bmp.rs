use std::io::{Cursor, Read};
use ::anyhow::{Result, Context};
use ::byteorder::{LittleEndian, ReadBytesExt};
use ::image::ImageFormat;
use ::image::io::Reader as ImageReader;
use ::strum::FromRepr;
use crate::readString;
use super::{InfinityEngineType, Readable};

const BPP_1bit: u16 = 1;
const BPP_4bit: u16 = 4;
const BPP_8bit: u16 = 8;
const BPP_16bit: u16 = 16;
const BPP_24bit: u16 = 24;

#[derive(Clone, Copy, Debug, FromRepr, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bmp
{
	pub file: BmpFile,
	pub info: BmpInfo,
	pub colors: Vec<u32>,
	pub encoded: Vec<u8>,
}

impl Bmp
{
	const Type: &'static str = "BM";
	const TypeLength: usize = 2;
	
	pub fn adhoc(width: i32, height: i32, pixels: Vec<u8>, palette: Option<Vec<u32>>) -> Self
	{
		return Self
		{
			file: BmpFile::adhoc(pixels.len() as u32),
			info: BmpInfo::adhoc(width, height),
			colors: palette.unwrap_or_default(),
			encoded: pixels.clone(),
		};
	}
	
	pub fn toBytes(&self) -> Vec<u8>
	{
		let mut bytes = vec![];
		bytes.append(self.file.toBytes().as_mut());
		bytes.append(self.info.toBytes().as_mut());
		
		for color in self.colors.clone()
		{
			bytes.append(color.to_le_bytes().to_vec().as_mut());
		}
		
		bytes.append(self.encoded.to_vec().as_mut());
		
		return bytes;
	}
	
	pub fn toImageBytes(&self, format: Option<ImageFormat>) -> Result<Vec<u8>>
	{
		let reader = ImageReader::with_format(Cursor::new(self.toBytes()), ImageFormat::Bmp)
			.decode()?;
		
		let mut data = vec![];
		let mut cursor = Cursor::new(&mut data);
		reader.write_to(&mut cursor, format.unwrap_or(ImageFormat::Png))
			.context("")?;
		
		return Ok(data);
	}
}

impl InfinityEngineType for Bmp {}

impl Readable for Bmp
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
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
		
		return Ok(Self
		{
			file,
			info,
			colors,
			encoded,
		});
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BmpFile
{
	pub r#type: String,
	pub size: u32,
	pub reserved: u32,
	pub offset: u32,
}

impl BmpFile
{
	pub fn adhoc(pixelSize: u32) -> Self
	{
		let offset = 54;
		return Self
		{
			r#type: Bmp::Type.to_string(),
			size: offset + pixelSize,
			reserved: 0,
			offset: offset,
		};
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

impl Readable for BmpFile
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let r#type = readString!(cursor, Bmp::TypeLength);
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
	pub fn adhoc(width: i32, height: i32) -> Self
	{
		return Self
		{
			size: 40,
			width,
			height,
			planes: 1,
			bitsPerPixel: BPP_24bit,
			compression: 0,
			compressedSize: 0,
			resolutionHorizontal: width * 7,
			resolutionVertical: height * 7,
			colorsUsed: 0,
			colorsImportant: 0,
		};
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

impl Readable for BmpInfo
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
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
}

#[cfg(test)]
mod tests
{
	#[allow(unused_imports)]
	use std::fs::File;
	#[allow(unused_imports)]
	use std::io::Write;
	#[allow(unused_imports)]
	use std::path::Path;
	#[allow(unused_imports)]
	use image::io::Reader as ImageReader;
    use super::*;
	use crate::platform::Games;
	use crate::resource::ResourceManager;
	use crate::test::updateResourceManager;
	use crate::types::ResourceType_BMP;
	
	#[test]
	fn BmpTest()
	{
		let resourceNames = vec![
			"AR0002SR", //4 bit
			"AJANTISS", //8 bit
			"AJANTISG", //24 bit
		];
		
		let resourceManager = ResourceManager::default();
		let _ = updateResourceManager(&resourceManager);
		
		for name in resourceNames.clone()
		{
			let bmp = resourceManager.loadResource::<Bmp>(Games::BaldursGate1, ResourceType_BMP, name.to_owned()).unwrap();
			
			assert_eq!(Bmp::Type, bmp.file.r#type);
			assert_eq!(14, bmp.file.toBytes().len());
			assert_eq!(bmp.info.size as usize, bmp.info.toBytes().len());
			
			if name.contains(resourceNames[0])
			{
				assert_eq!(BPP_4bit, bmp.info.bitsPerPixel);
				assert_eq!(56, bmp.info.width);
				assert_eq!(54, bmp.info.height);
				assert_eq!((BPP_4bit * BPP_4bit) as usize, bmp.colors.len());
			}
			else if name.contains(resourceNames[1])
			{
				assert_eq!(BPP_8bit, bmp.info.bitsPerPixel);
				assert_eq!(38, bmp.info.width);
				assert_eq!(60, bmp.info.height);
				assert_eq!((BPP_8bit * BPP_8bit * 4) as usize, bmp.colors.len());
			}
			else if name.contains(resourceNames[2])
			{
				assert_eq!(210, bmp.info.width);
				assert_eq!(330, bmp.info.height);
				assert_eq!(BPP_24bit, bmp.info.bitsPerPixel);
				assert_eq!(0, bmp.colors.len());
			}
			
			//Verify with eyes
			/*
			let outPath = Path::new("target").join(format!("testoutput_{}.png", name));
			let mut file = File::create(outPath.as_path())
				.expect("Output file couldn't be created");
			let bytes = bmp.toImageBytes(Some(ImageFormat::Png)).unwrap();
			let result = file.write_all(&bytes);
			assert!(result.is_ok());
			// */
		}
	}
}
