use std::io::Cursor;
use ::anyhow::Result;
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::readString;
use crate::bytes::{readName, readResRef};
use crate::types::util::{BoundingBox, Readable, Point2D};

/**
The fully parsed contents of a Door in an ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

---

Offset | Size | Description
---|---|---
0x0000 | 32 | Name
0x0020 | 8 | Door ID (to link with WED)
0x0028 | 4 | Flags
0x002c | 4 | Index of first vertex comprising the door outline (when open)
0x0030 | 2 | Count of vertices comprising the door outline (when open)
0x0032 | 2 | Count of vertices comprising the door outline (when closed)
0x0034 | 4 | Index of first vertex comprising the door outline (when closed)
0x0038 | 8 | Minimum bounding box of the door polygon (when open)
0x0040 | 8 | Minimum bounding box of the door polygon (when closed)
0x0048 | 4 | Index of first vertex in the impeded cell block (when open). These entries are x,y coordinates in the area search map. When the door is open, these cells cannot be entered by any object.
0x004c | 2 | Count of vertices in impeded cell block (when open)
0x004e | 2 | Count of vertices in impeded cell block (when closed)
0x0050 | 4 | Index of first vertex in the impeded cell block (when closed). These entries are x,y coordinates in the area search map. When the door is closed, these cells cannot be entered by any object.
0x0054 | 2 | Hit points
0x0056 | 2 | Armor class
0x0058 | 8 | Door open sound
0x0060 | 8 | Door close sound
0x0068 | 4 | Cursor index (cursors.bam)
0x006c | 2 | Trap detection difficulty
0x006e | 2 | Trap removal difficulty
0x0070 | 2 | Is door trapped? 0: No / 1: Yes
0x0072 | 2 | Is trap detected? 0: No / 1: Yes
0x0074 | 2 | Trap launch target X coordinate
0x0076 | 2 | Trap launch target Y coordinate
0x0078 | 8 | Key item
0x0080 | 8 | Door script
0x0088 | 4 | Detection difficulty (secret doors)
0x008c | 4 | Lock difficulty (0-100)
0x0090 | 8 | Two points. The player will move to the closest of these to toggle the door state.
0x0098 | 4 | Lockpick string
0x009c | 24 | Travel trigger name
0x00b4 | 4 | Dialog speaker name
0x00b8 | 8 | Dialog resref
*/
#[derive(Clone, Debug, Default)]
pub struct AreDoor
{
	pub name: String,
	pub id: String,
	pub flags: u32,
	pub outlineOpenFirst: u32,
	pub outlineOpenCount: u16,
	pub outlineClosedCount: u16,
	pub outlineClosedFirst: u32,
	pub boundingBoxOpen: BoundingBox,
	pub boundingBoxClosed: BoundingBox,
	pub impededOpenFirst: u32,
	pub impededOpenCount: u16,
	pub impededClosedCount: u16,
	pub impededClosedFirst: u32,
	pub hitPoints: u16,
	pub armorClass: u16,
	pub openSound: String,
	pub closeSound: String,
	pub cursorIndex: u32,
	pub trapDetectionDifficulty: u16,
	pub trapRemovalDifficulty: u16,
	pub trapped: u16,
	pub trapDetected: u16,
	pub trapLaunchTarget: Point2D<u16>,
	pub keyItem: String,
	pub script: String,
	pub detectionDifficulty: u32,
	pub lockDifficulty: u32,
	pub togglePoint1: Point2D<u16>,
	pub togglePoint2: Point2D<u16>,
	pub lockpickStringIndex: u32,
	pub travelTriggerName: String,
	pub dialogSpeakerName: String,
	pub dialog: String,
}

impl AreDoor
{
	const TravelTriggerNameLength: usize = 24;
	const DialogSpeakerNameLength: usize = 4;
	const UnusedPadding: u64 = 8;
}

impl Readable for AreDoor
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let name = readName(cursor)?;
		let id = readResRef(cursor)?;
		let flags = cursor.read_u32::<LittleEndian>()?;
		let outlineOpenFirst = cursor.read_u32::<LittleEndian>()?;
		let outlineOpenCount = cursor.read_u16::<LittleEndian>()?;
		let outlineClosedCount = cursor.read_u16::<LittleEndian>()?;
		let outlineClosedFirst = cursor.read_u32::<LittleEndian>()?;
		let boundingBoxOpen = BoundingBox::fromCursor(cursor)?;
		let boundingBoxClosed = BoundingBox::fromCursor(cursor)?;
		let impededOpenFirst = cursor.read_u32::<LittleEndian>()?;
		let impededOpenCount = cursor.read_u16::<LittleEndian>()?;
		let impededClosedCount = cursor.read_u16::<LittleEndian>()?;
		let impededClosedFirst = cursor.read_u32::<LittleEndian>()?;
		let hitPoints = cursor.read_u16::<LittleEndian>()?;
		let armorClass = cursor.read_u16::<LittleEndian>()?;
		let openSound = readResRef(cursor)?;
		let closeSound = readResRef(cursor)?;
		let cursorIndex = cursor.read_u32::<LittleEndian>()?;
		let trapDetectionDifficulty = cursor.read_u16::<LittleEndian>()?;
		let trapRemovalDifficulty = cursor.read_u16::<LittleEndian>()?;
		let trapped = cursor.read_u16::<LittleEndian>()?;
		let trapDetected = cursor.read_u16::<LittleEndian>()?;
		let trapLaunchTarget = Point2D::<u16>::fromCursor(cursor)?;
		let keyItem = readResRef(cursor)?;
		let script = readResRef(cursor)?;
		let detectionDifficulty = cursor.read_u32::<LittleEndian>()?;
		let lockDifficulty = cursor.read_u32::<LittleEndian>()?;
		let togglePoint1 = Point2D::<u16>::fromCursor(cursor)?;
		let togglePoint2 = Point2D::<u16>::fromCursor(cursor)?;
		let lockpickStringIndex = cursor.read_u32::<LittleEndian>()?;
		let travelTriggerName = readString!(cursor, Self::TravelTriggerNameLength);
		let dialogSpeakerName = readString!(cursor, Self::DialogSpeakerNameLength);
		let dialog = readResRef(cursor)?;
		
		cursor.set_position(cursor.position() + Self::UnusedPadding);
		
		return Ok(Self
		{
			name,
			id,
			flags,
			outlineOpenFirst,
			outlineOpenCount,
			outlineClosedCount,
			outlineClosedFirst,
			boundingBoxOpen,
			boundingBoxClosed,
			impededOpenFirst,
			impededOpenCount,
			impededClosedCount,
			impededClosedFirst,
			hitPoints,
			armorClass,
			openSound,
			closeSound,
			cursorIndex,
			trapDetectionDifficulty,
			trapRemovalDifficulty,
			trapped,
			trapDetected,
			trapLaunchTarget,
			keyItem,
			script,
			detectionDifficulty,
			lockDifficulty,
			togglePoint1,
			togglePoint2,
			lockpickStringIndex,
			travelTriggerName,
			dialogSpeakerName,
			dialog,
		});
	}
}
