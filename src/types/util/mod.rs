#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod bitmask;
mod boundingbox;
mod color;
mod dimensions;
mod functions;
mod identity;
mod point;
mod section;
mod traits;

pub use bitmask::BitmaskAddress;
pub use boundingbox::BoundingBox;
pub use color::Color;
pub use dimensions::{Dimensions, Dimensions_Layout};
pub use functions::{ReadFromFile, ReadList};
pub use identity::Identity;
pub use point::{Point2D, Point3D};
pub use section::SectionAddress;
pub use traits::{InfinityEngineType, Readable, ReadIntoSelf};

pub const TypeSize_RESREF: usize = 8;
