#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::Cursor;
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::types::util::BoundingBox;
use crate::types::Readable;

/**
A WED polygon.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wed_v1.3.htm

A polygon consists of a list of vertices, a minimum bounding box, and some
unknown state information.

The order of vertices can be important when describing polygins:

- For door walls, the vertices have to be listed clockwise, starting with the
	rightmost vertex first. If there are two or more vertices on the same
	rightmost vertical line, the lowest point should be listed first.
- For normal walls, the vertices can be listed in clockwise or
	counter-clockwise order, starting with the lowest most vertex.

---

Offset | Size | Description
---|---|---
0x0000 | 4 | Starting vertex index
0x0004 | 4 | Count of vertices
0x0008 | 1 | Indicates whether this polygon is a passable or not.
0x0009 | 1 | Height
0x000a | 2 | Minimum X coordinate of bounding box
0x000c | 2 | Maximum X coordinate of bounding box
0x000e | 2 | Minimum Y coordinate of bounding box
0x0010 | 2 | Maximum Y coordinate of bounding box

### Polygon mask

- bit 0: Shade wall
- bit 1: Hovering
- bit 2: Cover animations
- bit 3: Cover animations
- bit 4: Unknown
- bit 5: Unknown
- bit 6: Unknown
- bit 7: Door?
*/
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Polygon
{
	pub start: u32,
	pub count: u32,
	pub mask: u8,
	pub height: u8,
	pub boundingBox: BoundingBox,
}

impl Readable for Polygon
{
	fn fromCursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self>
		where Self: Sized
	{
		let start = cursor.read_u32::<LittleEndian>()?;
		let count = cursor.read_u32::<LittleEndian>()?;
		let mask = cursor.read_u8()?;
		let height = cursor.read_u8()?;
		let boundingBox = BoundingBox::fromCursor(cursor)?;
		
		return Ok(Self
		{
			start,
			count,
			mask,
			height,
			boundingBox,
		});
	}
}
