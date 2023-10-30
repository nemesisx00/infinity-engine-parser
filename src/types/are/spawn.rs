#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::{readName, readResRef};
use crate::types::util::Readable;

/**
The fully parsed contents of a Spawn Point in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 2 | X coordinate
0x0022 | 2 | Y coordinate
0x0024 | 8 | 1st resref of creature to spawn
0x002c | 8 | 2nd resref of creature to spawn
0x0034 | 8 | 3rd resref of creature to spawn
0x003c | 8 | 4th resref of creature to spawn
0x0044 | 8 | 5th resref of creature to spawn
0x004c | 8 | 6th resref of creature to spawn
0x0054 | 8 | 7th resref of creature to spawn
0x005c | 8 | 8th resref of creature to spawn
0x0064 | 8 | 9th resref of creature to spawn
0x006c | 8 | 10th resref of creature to spawn
0x0074 | 2 | Count of spawned creatures
0x0076 | 2 | Base creature number to spawn. The actual number to spawn is given by the following equation, with the results rounded down: (Frequency * Average Party Level) / Creature Power
0x0078 | 2 | Frequency (i.e. Seconds between spawns)
0x007a | 2 | Spawn method
0x007c | 4 | Actor removal timer (seconds) (usually -1 to avoid removal)
0x0080 | 2 | Movement restriction distance
0x0082 | 2 | Movement restriction distance (move to object)
0x0084 | 2 | Maximum creatures to spawn
0x0086 | 2 | Is spawn point enabled? 0: No / 1: Yes
0x0088 | 4 | Spawn point appearance schedule. Bits 0-23 represent an hour of game time. Setting a bit means the actor will appear in the area for the corresponding hour.
0x008c | 2 | Probability (day)
0x008e | 2 | Probability (night)
0x0090 | 4 | Spawn frequency
0x0094 | 4 | Countdown
0x0098 | 1 | Spawn weight of 1st creature slot (see offset 0x0024)
0x0099 | 1 | Spawn weight of 2nd creature slot (see offset 0x002c)
0x009a | 1 | Spawn weight of 3rd creature slot (see offset 0x0034)
0x009b | 1 | Spawn weight of 4th creature slot (see offset 0x003c)
0x009c | 1 | Spawn weight of 5th creature slot (see offset 0x0044)
0x009d | 1 | Spawn weight of 6th creature slot (see offset 0x004c)
0x009e | 1 | Spawn weight of 7th creature slot (see offset 0x0054)
0x00a0 | 1 | Spawn weight of 8th creature slot (see offset 0x005c)
0x00a1 | 1 | Spawn weight of 9th creature slot (see offset 0x0064)
0x00a2 | 1 | Spawn weight of 10th creature slot (see offset 0x006c)
*/
#[derive(Clone, Debug, Default)]
pub struct AreSpawnPoint
{
	pub name: String,
	pub x: u16,
	pub y: u16,
	pub spawnCount: u16,
	pub spawnBaseCount: u16,
	pub frequency: u16,
	pub spawnMethod: u16,
	pub removalTimer: u32,
	pub restrictionDistance: u16,
	pub restrictionDistanceObject: u16,
	pub spawnMaxCount: u16,
	pub enabled: u16,
	pub schedule: u32,
	pub probabilityDay: u16,
	pub probabilityNight: u16,
	
	//Only used in BGEE
	pub spawnFrequency: u32,
	pub countdown: u32,
	
	//Composed of <creatureRef, spawnWeight>; spawnWeight is only used in BGEE
	pub creatures: HashMap<String, u8>,
}

impl AreSpawnPoint
{
	const CreatureRefMax: usize = 10;
	const UnusedPadding: u64 = 56;
	const UnusedPadding_BGEE: u64 = 38;
	
	pub fn isEnabled(&self) -> bool
	{
		return self.enabled == 1;
	}
}

impl Readable for AreSpawnPoint
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let name = readName(cursor)?;
		let x = cursor.read_u16::<LittleEndian>()?;
		let y = cursor.read_u16::<LittleEndian>()?;
		
		let mut creatureRefs = vec![];
		for _ in 0..Self::CreatureRefMax
		{
			let resref = readResRef(cursor)?;
			creatureRefs.push(resref);
		}
		
		let spawnCount = cursor.read_u16::<LittleEndian>()?;
		let spawnBaseCount = cursor.read_u16::<LittleEndian>()?;
		let frequency = cursor.read_u16::<LittleEndian>()?;
		let spawnMethod = cursor.read_u16::<LittleEndian>()?;
		let removalTimer = cursor.read_u32::<LittleEndian>()?;
		let restrictionDistance = cursor.read_u16::<LittleEndian>()?;
		let restrictionDistanceObject = cursor.read_u16::<LittleEndian>()?;
		let spawnMaxCount = cursor.read_u16::<LittleEndian>()?;
		let enabled = cursor.read_u16::<LittleEndian>()?;
		let schedule = cursor.read_u32::<LittleEndian>()?;
		let probabilityDay = cursor.read_u16::<LittleEndian>()?;
		let probabilityNight = cursor.read_u16::<LittleEndian>()?;
		let spawnFrequency = cursor.read_u32::<LittleEndian>()?;
		let countdown = cursor.read_u32::<LittleEndian>()?;
		
		let mut spawnWeight = vec![];
		for _ in 0..Self::CreatureRefMax
		{
			let weight = cursor.read_u8()?;
			spawnWeight.push(weight);
		}
		
		let mut creatures = HashMap::<String, u8>::new();
		for i in 0..Self::CreatureRefMax
		{
			creatures.insert(creatureRefs[i].to_owned(), spawnWeight[i]);
		}
		
		cursor.set_position(cursor.position() + match spawnFrequency > 0
		{
			true => Self::UnusedPadding_BGEE,
			false => Self::UnusedPadding,
		});
		
		return Ok(Self
		{
			name,
			x,
			y,
			spawnCount,
			spawnBaseCount,
			frequency,
			spawnMethod,
			removalTimer,
			restrictionDistance,
			restrictionDistanceObject,
			spawnMaxCount,
			enabled,
			schedule,
			probabilityDay,
			probabilityNight,
			spawnFrequency,
			countdown,
			creatures,
		});
	}
}
