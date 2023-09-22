#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod are;
mod bif;
mod bmp;
mod key;
mod tis;
mod util;
mod wed;

pub use are::Are;
pub use bif::Bif;
pub use bmp::Bmp;
pub use key::Key;
pub use tis::Tis;
pub use util::{Dimensions, Identity, InfinityEngineType, ReadFromFile};

pub use bif::{
	ResourceType_ARE,
	ResourceType_BAM,
	ResourceType_BAMC,
	ResourceType_BMP,
	ResourceType_MOS,
	ResourceType_MOSC,
	ResourceType_TIS,
	ResourceType_WAV,
	ResourceType_WAVC,
	ResourceType_WED,
};
