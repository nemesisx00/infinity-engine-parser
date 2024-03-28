use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::bytes::readResRef;
use crate::types::Identity;
use crate::types::util::{BitmaskAddress, SectionAddress, Readable};
use super::util::AreRef;

/**
The fully parsed contents of a ARE file's Header section.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('AREA')
0x0004 | 4 | Version ('V1.0')
0x0008 | 8 | Area WED
0x0010 | 4 | Last saved (seconds, real time)
0x0014 | 4 | Area flags (AREAFLAG.IDS)
0x0018 | 8 | Resref of the area to the North of this area
0x0020 | 4 | North area flags
0x0024 | 8 | Resref of the area to the East of this area
0x002c | 4 | East area flags
0x0030 | 8 | Resref of the area to the South of this area
0x0038 | 4 | South area flags
0x003c | 8 | Resref of the area to the West of this area
0x0044 | 4 | West area flags
0x0048 | 2 | Area type flags (AREATYPE.IDS)
0x004a | 2 | Rain probabiliy
0x004c | 2 | Snow probability
0x004e | 2 | Fog probability (BGEE only)
0x0050 | 2 | Lightning probability
0x0052 | 2 | BG1:TotS, IWD:ToTL, BG2:ToB - Wind speed (unused?); BGEE - Overlay transparency (only first byte)
0x0054 | 4 | Offset to actors
0x0058 | 2 | Count of actors
0x005a | 2 | Count of regions
0x005c | 4 | Offset to regions
0x0060 | 4 | Offset to spawn points
0x0064 | 4 | Count of spawn points
0x0068 | 4 | Offset to entrances
0x006c | 4 | Count of entrances
0x0070 | 4 | Offset to containers
0x0074 | 2 | Count of containers
0x0076 | 2 | Count of items
0x0078 | 4 | Offset to items
0x007c | 4 | Offset to vertices
0x0080 | 2 | Count of vertices
0x0082 | 2 | Count of ambients
0x0084 | 4 | Offset to ambients
0x0088 | 4 | Offset to variables
0x008c | 4 | Count of variables
0x0090 | 2 | Offset to tiled object flags
0x0092 | 2 | Count of tiled object flags
0x0094 | 8 | Area script
0x009c | 4 | Size of explored bitmask
0x00a0 | 4 | Offset to explored bitmask
0x00a4 | 4 | Count of doors
0x00a8 | 4 | Offset to doors
0x00ac | 4 | Count of animations
0x00b0 | 4 | Offset to animation
0x00b4 | 4 | Count of tiled objects
0x00b8 | 4 | Offset to tiled objects
0x00bc | 4 | Offset to song entries
0x00c0 | 4 | Offset to rest interruptions
0x00c4 | 4 | PST: 0xFFFFFFFF; Other: Offset to the automap note section
0x00c8 | 4 | PST: Offset of the automap note section; Other: Number of entries in the automap note section
0x00cc | 4 | PST: Number of entries in the automap note section; Other: Offset to the projectile traps section
0x00d0 | 4 | Number of entries in the projectile traps section
0x00d4 | 8 | BG2: ToB, BGEE - Rest movie (day); Others - Unknown
0x00dc | 8 | BG2: ToB, BGEE - Rest movie (night); Others - Unknown
0x00e4 | 56 | Unused

---

### Area Flags

#### BGS1: TotS, IDW: ToTL, BG2:ToB

Bit | Description
---|---
0 | Save not allowed
1 | Tutorial area (not BG1)
2 | Dead magic zone
3 | Dream

#### BGEE

Bit | Description
---|---
0 | Save not allowed
1 | Tutorial area
2 | Dead magic zone
3 | Dream
4 | Player1 death does not end the game
5 | Resting not allowed
6 | Travel not allowed

#### PST

Bit | Description
---|---
0 | Save not allowed
1 | "You cannot rest here."
2 | "Too dangerous to rest."
1+2 | "You must obtain permission to rest here."

#### PSTEE

Bit | Description
---|---
0 | Save not allowed
1 | Reform Party not allowed
2 | Dead magic zone
3 | Dream
4 | Player1 death does not end the game
5 | Resting not allowed
6 | Travel not allowed
7 | "You cannot rest here."
8 | "Too dangerous to rest here."
7+8 | "You must obtain permission to rest here."

---

### Area Transition Flags

Bit | Description
---|---
0 | Party Required
1 | Party Enabled

---

### Area Type Flags

#### BG1:TotS, IWD:ToTL, BG2:ToB, BGEE

Bit | Description
---|---
0 | Outdoor
1 | Day/night
2 | Weather
3 | City
4 | Forest
5 | Dungeon
6 | Extended night
7 | Can rest indoors

#### PST, PSTEE

Bit | Description
---|---
0 | Hive
1 | Hive night?
2 | Clerk's Ward
3 | Lower Ward
4 | Ravel's Maze
5 | Baator
6 | Rubikon
7 | Fortress of Regrets
8 | Curst
9 | Carceri
10 | Outdoors
*/
#[derive(Clone, Debug, Default)]
pub struct AreHeader
{
	pub identity: Identity,
	pub wedName: String,
	pub lastSaved: u32,
	pub areaFlags: u32,
	pub north: AreRef,
	pub east: AreRef,
	pub south: AreRef,
	pub west: AreRef,
	pub areaTypeFlags: u16,
	pub rain: u16,
	pub snow: u16,
	pub fog: u16,
	pub lightning: u16,
	pub wind: u16,
	pub actors: SectionAddress<u32, u16>,
	pub regions: SectionAddress<u32, u16>,
	pub spawnPoints: SectionAddress<u32, u32>,
	pub entrances: SectionAddress<u32, u32>,
	pub containers: SectionAddress<u32, u16>,
	pub items: SectionAddress<u32, u16>,
	pub vertices: SectionAddress<u32, u16>,
	pub ambients: SectionAddress<u32, u16>,
	pub variables: SectionAddress<u32, u32>,
	pub tiledObjectFlags: SectionAddress<u16, u16>,
	pub scriptName: String,
	pub explored: BitmaskAddress<u32, u32>,
	pub doors: SectionAddress<u32, u32>,
	pub animations: SectionAddress<u32, u32>,
	pub tiledObjects: SectionAddress<u32, u32>,
	pub songEntriesOffset: u32,
	pub restInterruptions: u32,
	pub automapNotes: SectionAddress<u32, u32>,
	pub projectileTraps: SectionAddress<u32, u32>,
	pub restMovieDay: String,
	pub restMovieNight: String,
}

impl AreHeader
{
	pub const UnusedPadding: u64 = 56;
}

impl Readable for AreHeader
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
	{
		let identity = Identity::fromCursor(cursor)?;
		let wedName = readResRef(cursor)?;
		let lastSaved = cursor.read_u32::<LittleEndian>()?;
		let areaFlags = cursor.read_u32::<LittleEndian>()?;
		let north = AreRef::fromCursor(cursor)?;
		let east = AreRef::fromCursor(cursor)?;
		let south = AreRef::fromCursor(cursor)?;
		let west = AreRef::fromCursor(cursor)?;
		let areaTypeFlags = cursor.read_u16::<LittleEndian>()?;
		let rain = cursor.read_u16::<LittleEndian>()?;
		let snow = cursor.read_u16::<LittleEndian>()?;
		let fog = cursor.read_u16::<LittleEndian>()?;
		let lightning = cursor.read_u16::<LittleEndian>()?;
		let wind = cursor.read_u16::<LittleEndian>()?;
		let actors = SectionAddress::<u32, u16>::fromCursor(cursor)?;
		let regions = SectionAddress::<u32, u16>::fromCursorInverted(cursor)?;
		let spawnPoints = SectionAddress::<u32, u32>::fromCursor(cursor)?;
		let entrances = SectionAddress::<u32, u32>::fromCursor(cursor)?;
		let containers = SectionAddress::<u32, u16>::fromCursor(cursor)?;
		let items = SectionAddress::<u32, u16>::fromCursorInverted(cursor)?;
		let vertices = SectionAddress::<u32, u16>::fromCursor(cursor)?;
		let ambients = SectionAddress::<u32, u16>::fromCursorInverted(cursor)?;
		let variables = SectionAddress::<u32, u32>::fromCursor(cursor)?;
		let tiledObjectFlags = SectionAddress::<u16, u16>::fromCursor(cursor)?;
		let scriptName = readResRef(cursor)?;
		let explored = BitmaskAddress::<u32, u32>::fromCursorInverted(cursor)?;
		let doors = SectionAddress::<u32, u32>::fromCursorInverted(cursor)?;
		let animations = SectionAddress::<u32, u32>::fromCursorInverted(cursor)?;
		let tiledObjects = SectionAddress::<u32, u32>::fromCursorInverted(cursor)?;
		let songEntriesOffset = cursor.read_u32::<LittleEndian>()?;
		let restInterruptions = cursor.read_u32::<LittleEndian>()?;
		let automapNotes = SectionAddress::<u32, u32>::fromCursor(cursor)?;
		let projectileTraps = SectionAddress::<u32, u32>::fromCursor(cursor)?;
		let restMovieDay = readResRef(cursor)?;
		let restMovieNight = readResRef(cursor)?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			identity,
			wedName,
			lastSaved,
			areaFlags,
			north,
			east,
			south,
			west,
			areaTypeFlags,
			rain,
			snow,
			fog,
			lightning,
			wind,
			actors,
			regions,
			spawnPoints,
			entrances,
			containers,
			items,
			vertices,
			ambients,
			variables,
			tiledObjectFlags,
			scriptName,
			explored,
			doors,
			animations,
			tiledObjects,
			songEntriesOffset,
			restInterruptions,
			automapNotes,
			projectileTraps,
			restMovieDay,
			restMovieNight,
		});
	}
}
