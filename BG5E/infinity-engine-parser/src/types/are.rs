#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::io::{Cursor, Read};
use ::anyhow::{Context, Result};
use ::byteorder::{LittleEndian, ReadBytesExt};
use crate::readString;
use super::util::TypeSize_RESREF;
use super::{Identity, InfinityEngineType};

const Signature: &str = "AREA";
const Version: &str = "V1.0";

/**
The fully parsed contents of a ARE file.

See https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.0.htm

The ARE file format describes the content of an area, as opposed to its visual
representation. ARE files contain the list of actors, items, entrances and exits,
spawn points, and other area-associated info. The ARE file may contain
references to other files, however these other files are not embedded in the ARE
file.

---

### Header Data

Offset | Size | Description
---|---|---
0x0000 | 4 | Signature ('AREA')
0x0004 | 4 | Version ('V1.0')
*/
#[derive(Clone, Debug, Default)]
pub struct Are
{
	pub identity: Identity,
	
}
