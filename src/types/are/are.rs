#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::ReadBytesExt;
use crate::readBytes;
use crate::types::{InfinityEngineType, ReadList};
use crate::types::util::{Readable, Point2D};
use super::*;

/**
The fully parsed contents of a ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

The ARE file format describes the content of an area, as opposed to its visual
representation. ARE files contain the list of actors, items, entrances and exits,
spawn points, and other area-associated info. The ARE file may contain
references to other files, however these other files are not embedded in the ARE
file.
*/
#[derive(Clone, Debug, Default)]
pub struct Are
{
	pub header: AreHeader,
	pub actors: Vec<AreActor>,
	pub regions: Vec<AreRegion>,
	pub spawnPoints: Vec<AreSpawnPoint>,
	pub entrances: Vec<AreEntrance>,
	pub containers: Vec<AreContainer>,
	pub items: Vec<AreItem>,
	pub vertices: Vec<Point2D<u16>>,
	pub ambients: Vec<AreAmbient>,
	pub variables: Vec<AreVariable>,
	pub explored: Vec<u8>,
	pub doors: Vec<AreDoor>,
	pub animations: Vec<AreAnimation>,
	pub tiledObjects: Vec<AreTiledObject>,
	pub songEntries: AreSongEntries,
	pub restInterruptions: AreRestInterruptions,
}

impl Are
{
	pub const Signature: &str = "AREA";
	pub const Version: &str = "V1.0";
	
	fn readVertices(cursor: &mut Cursor<Vec<u8>>, offset: u64, count: u16) -> Result<Vec<Point2D<u16>>>
	{
		let mut vertices = vec![];
		if cursor.position() != offset
		{
			cursor.set_position(offset);
		}
		
		for _ in 0..count
		{
			let vertex = Point2D::<u16>::fromCursor(cursor)?;
			vertices.push(vertex);
		}
		
		return Ok(vertices);
	}
	
	fn readExploredBitmask(cursor: &mut Cursor<Vec<u8>>, offset: u64, size: u32) -> Result<Vec<u8>>
	{
		if cursor.position() != offset
		{
			cursor.set_position(offset);
		}
		
		let explored = readBytes!(cursor, size);
		return Ok(explored);
	}
}

impl InfinityEngineType for Are {}

impl Readable for Are
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let header = AreHeader::fromCursor(cursor)
			.context("Error parsing ARE header")?;
		
		let actors = ReadList::<AreActor>(cursor, header.actors.offset.into(), header.actors.count.into())?;
		let regions = ReadList::<AreRegion>(cursor, header.regions.offset.into(), header.regions.count.into())?;
		let spawnPoints = ReadList::<AreSpawnPoint>(cursor, header.spawnPoints.offset.into(), header.spawnPoints.count.into())?;
		let entrances = ReadList::<AreEntrance>(cursor, header.entrances.offset.into(), header.entrances.count.into())?;
		let containers = ReadList::<AreContainer>(cursor, header.containers.offset.into(), header.containers.count.into())?;
		let items = ReadList::<AreItem>(cursor, header.items.offset.into(), header.items.count.into())?;
		//An array of points used to create the outlines of regions and containers. Elements are 16-bit words stored x0, y0, x1, y1 etc.
		let vertices = Self::readVertices(cursor, header.vertices.offset.into(), header.vertices.count)?;
		let ambients = ReadList::<AreAmbient>(cursor, header.ambients.offset.into(), header.ambients.count.into())?;
		let variables = ReadList::<AreVariable>(cursor, header.variables.offset.into(), header.variables.count.into())?;
		let explored = Self::readExploredBitmask(cursor, header.explored.offset.into(), header.explored.size)?;
		let doors = ReadList::<AreDoor>(cursor, header.doors.offset.into(), header.doors.count.into())?;
		let animations = ReadList::<AreAnimation>(cursor, header.animations.offset.into(), header.animations.count.into())?;
		let tiledObjects = ReadList::<AreTiledObject>(cursor, header.tiledObjects.offset.into(), header.tiledObjects.count.into())?;
		
		if cursor.position() != header.songEntriesOffset.into()
		{
			cursor.set_position(header.songEntriesOffset.into());
		}
		let songEntries = AreSongEntries::fromCursor(cursor)?;
		
		if cursor.position() != header.restInterruptions.into()
		{
			cursor.set_position(header.restInterruptions.into());
		}
		let restInterruptions = AreRestInterruptions::fromCursor(cursor)?;
		
		return Ok(Self
		{
			header,
			actors,
			regions,
			spawnPoints,
			entrances,
			containers,
			items,
			vertices,
			ambients,
			variables,
			explored,
			doors,
			animations,
			tiledObjects,
			songEntries,
			restInterruptions,
		});
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use crate::platform::Games;
	use crate::resource::ResourceManager;
	use crate::types::ResourceType_ARE;
	
	#[test]
	fn ParseAre()
	{
		let game = Games::BaldursGate1;
		let name = "AR2600";
		
		let resourceManager = ResourceManager::default();
		let result = resourceManager.loadResource::<Are>(game, ResourceType_ARE, name.to_owned()).unwrap();
		
		assert_eq!(Are::Signature, result.header.identity.signature);
		assert_eq!(Are::Version, result.header.identity.version);
		assert_eq!(name, result.header.wedName);
		
		assert_eq!(result.header.actors.count as usize, result.actors.len());
		assert_eq!(result.header.regions.count as usize, result.regions.len());
		assert_eq!(result.header.spawnPoints.count as usize, result.spawnPoints.len());
		assert_eq!(result.header.entrances.count as usize, result.entrances.len());
		assert_eq!(result.header.containers.count as usize, result.containers.len());
		assert_eq!(result.header.items.count as usize, result.items.len());
		assert_eq!(result.header.vertices.count as usize, result.vertices.len());
		assert_eq!(result.header.ambients.count as usize, result.ambients.len());
		assert_eq!(result.header.variables.count as usize, result.variables.len());
		assert_eq!(result.header.explored.size as usize, result.explored.len());
		assert_eq!(result.header.doors.count as usize, result.doors.len());
		assert_eq!(result.header.animations.count as usize, result.animations.len());
		assert_eq!(result.header.tiledObjects.count as usize, result.tiledObjects.len());
		assert!(!result.songEntries.ambientDay1.is_empty());
		assert_eq!(result.restInterruptions.creatureCount as usize, result.restInterruptions.creatures.iter().filter(|c| !c.is_empty()).count());
	}
}
