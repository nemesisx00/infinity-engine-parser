use std::collections::HashMap;
use safer_ffi::derive_ReprC;
use serde::{Deserialize, Serialize};
use ::strum::FromRepr;

#[derive_ReprC]
#[derive(Clone, Copy, Debug, Deserialize, Eq, FromRepr, Hash, PartialEq, Serialize)]
#[repr(i32)]
pub enum Games
{
	None,
	BaldursGate1,
	BaldursGate1EnhancedEdition,
	BaldursGate2,
	BaldursGate2EnhancedEdition,
	IcewindDale1,
	IcewindDale1EnhancedEdition,
	IcewindDale2,
	PlanescapeTorment,
	PlanescapeTormentEnhancedEdition,
}

impl TryFrom<i32> for Games
{
	type Error = ();
	
	fn try_from(value: i32) -> Result<Self, Self::Error> {
		return match value
		{
			0 => Ok(Games::None),
			1 => Ok(Games::BaldursGate1),
			2 => Ok(Games::BaldursGate1EnhancedEdition),
			3 => Ok(Games::BaldursGate2),
			4 => Ok(Games::BaldursGate2EnhancedEdition),
			5 => Ok(Games::IcewindDale1),
			6 => Ok(Games::IcewindDale1EnhancedEdition),
			7 => Ok(Games::IcewindDale2),
			8 => Ok(Games::PlanescapeTorment),
			9 => Ok(Games::PlanescapeTormentEnhancedEdition),
			_ => Err(()),
		};
	}
}

#[allow(dead_code)]
pub fn GogGameId(game: Games) -> Option<u32>
{
	let map = HashMap::from([
		( Games::BaldursGate1, 1207658886 ),
		( Games::BaldursGate1EnhancedEdition, 1207666353 ),
		( Games::BaldursGate2, 1207658893 ),
		( Games::BaldursGate2EnhancedEdition, 1207666373 ),
		( Games::IcewindDale1, 1207658888 ),
		( Games::IcewindDale1EnhancedEdition, 1207666683 ),
		( Games::IcewindDale2, 1207658891 ),
		( Games::PlanescapeTorment, 1207658887 ),
		( Games::PlanescapeTormentEnhancedEdition, 1203613131 ),
	]);
	
	return map.get(&game).cloned();
}

pub fn KeyFileName(game: Games) -> Option<String>
{
	let map = HashMap::from([
		( Games::BaldursGate1, String::from("Chitin.key") ),
		( Games::BaldursGate1EnhancedEdition, String::from("chitin.key") ),
		( Games::BaldursGate2, String::from("CHITIN.KEY") ),
		( Games::BaldursGate2EnhancedEdition, String::from("chitin.key") ),
		( Games::IcewindDale1, String::from("CHITIN.KEY") ),
		( Games::IcewindDale1EnhancedEdition, String::from("chitin.key") ),
		( Games::IcewindDale2, String::from("CHITIN.KEY") ),
		( Games::PlanescapeTorment, String::from("CHITIN.KEY") ),
		( Games::PlanescapeTormentEnhancedEdition, String::from("chitin.key") ),
	]);
	
	return map.get(&game).cloned();
}

#[allow(dead_code)]
pub fn SteamAppId(game: Games) -> Option<u32>
{
	let map = HashMap::from([
		( Games::BaldursGate1, 24431 ),
		( Games::BaldursGate1EnhancedEdition, 228280 ),
		( Games::BaldursGate2, 99140 ),
		( Games::BaldursGate2EnhancedEdition, 257350 ),
		( Games::IcewindDale1, 206940 ),
		( Games::IcewindDale1EnhancedEdition, 321800 ),
		( Games::IcewindDale2, 206950 ),
		( Games::PlanescapeTorment, 205180 ),
		( Games::PlanescapeTormentEnhancedEdition, 466300 ),
	]);
	
	return map.get(&game).cloned();
}
