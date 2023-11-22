#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod actor;
mod ambient;
mod animation;
mod are;
mod automap;
mod container;
mod door;
mod entrance;
mod header;
mod item;
mod region;
mod rest;
mod song;
mod spawn;
mod tiled;
mod trap;
mod util;
mod variable;

pub use actor::AreActor;
pub use ambient::AreAmbient;
pub use animation::AreAnimation;
pub use are::Are;
pub use automap::AreAutomapNote;
pub use container::AreContainer;
pub use door::AreDoor;
pub use entrance::AreEntrance;
pub use header::AreHeader;
pub use item::AreItem;
pub use region::AreRegion;
pub use rest::AreRestInterruptions;
pub use song::AreSongEntries;
pub use spawn::AreSpawnPoint;
pub use tiled::AreTiledObject;
pub use trap::AreProjectileTrap;
pub use util::AreRef;
pub use variable::AreVariable;