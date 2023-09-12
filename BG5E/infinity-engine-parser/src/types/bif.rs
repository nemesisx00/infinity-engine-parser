#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::types::util::Identity;

const Signature: &str = "BIFF";
const Version: &str = "V1  ";

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
pub struct Bif
{
	pub identity: Identity,
	pub fileCount: u32,
	pub tilesetCount: u32,
	pub offset: u32,
	pub fileEntries: Vec<()>,
	pub tilesetEntries: Vec<()>,
}

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
pub struct FileEntry
{
	
}
