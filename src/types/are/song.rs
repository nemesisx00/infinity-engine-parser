#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::util::Readable;

/**
The fully parsed contents of the Song Entries in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Day song reference number
0x0004 | 4 | Night song reference number
0x0008 | 4 | Win song reference number
0x000c | 4 | Battle song reference number
0x0010 | 4 | Lose song reference number
0x0014 | 4 | Alt music 1
0x0018 | 4 | Alt music 2
0x001c | 4 | Alt music 3
0x0020 | 4 | Alt music 4
0x0024 | 4 | Alt music 5
0x0028 | 8 | Main day ambient 1 (WAV)
0x0030 | 8 | Main day ambient 2 (WAV)
0x0038 | 4 | Main day ambient volume %
0x003c | 8 | Main night ambient 1 (WAV)
0x0044 | 8 | Main night ambient 2 (WAV)
0x004c | 4 | Main night ambient volume %
0x0050 | 4 | Reverb from REVERB.IDS, if it exists; Reverb from REVERB.2DA, if it exists
*/
#[derive(Clone, Debug, Default)]
pub struct AreSongEntries
{
	pub refDay: u32,
	pub refNight: u32,
	pub refWin: u32,
	pub refBattle: u32,
	pub refLose: u32,
	pub alt1: u32,
	pub alt2: u32,
	pub alt3: u32,
	pub alt4: u32,
	pub alt5: u32,
	pub ambientDay1: String,
	pub ambientDay2: String,
	pub ambientDayVolume: u32,
	pub ambientNight1: String,
	pub ambientNight2: String,
	pub ambientNightVolume: u32,
	pub reverb: u32,
}

impl AreSongEntries
{
	const UnusedPadding: u64 = 60;
}

impl Readable for AreSongEntries
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let refDay = cursor.read_u32::<LittleEndian>()?;
		let refNight = cursor.read_u32::<LittleEndian>()?;
		let refWin = cursor.read_u32::<LittleEndian>()?;
		let refBattle = cursor.read_u32::<LittleEndian>()?;
		let refLose = cursor.read_u32::<LittleEndian>()?;
		let alt1 = cursor.read_u32::<LittleEndian>()?;
		let alt2 = cursor.read_u32::<LittleEndian>()?;
		let alt3 = cursor.read_u32::<LittleEndian>()?;
		let alt4 = cursor.read_u32::<LittleEndian>()?;
		let alt5 = cursor.read_u32::<LittleEndian>()?;
		let ambientDay1 = readResRef(cursor)?;
		let ambientDay2 = readResRef(cursor)?;
		let ambientDayVolume = cursor.read_u32::<LittleEndian>()?;
		let ambientNight1 = readResRef(cursor)?;
		let ambientNight2 = readResRef(cursor)?;
		let ambientNightVolume = cursor.read_u32::<LittleEndian>()?;
		let reverb = cursor.read_u32::<LittleEndian>()?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			refDay,
			refNight,
			refWin,
			refBattle,
			refLose,
			alt1,
			alt2,
			alt3,
			alt4,
			alt5,
			ambientDay1,
			ambientDay2,
			ambientDayVolume,
			ambientNight1,
			ambientNight2,
			ambientNightVolume,
			reverb,
		});
	}
}
