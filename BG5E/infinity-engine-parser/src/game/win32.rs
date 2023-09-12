#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use registry::{Hive, RegKey, Security};
use crate::game::global::*;

const DisplayName: &str = "DisplayName";
const InstallLocation: &str = "InstallLocation";
const NewGogGameId: &str = "gameID";
const NewGogPath: &str = "path";

fn GameDisplayNames(game: &Games) -> Option<Vec<String>>
{
	let map = HashMap::from([
		( Games::BaldursGate1, vec![ String::from("Baldur's Gate -  The Original Saga") ] ),
		( Games::BaldursGate1EnhancedEdition, vec![ String::from("Baldur's Gate: Enhanced Edition") ] ),
		( Games::BaldursGate2, vec![ String::from("Baldur's Gate 2 Complete") ] ),
		( Games::BaldursGate2EnhancedEdition, vec![ String::from("Baldur's Gate II: Enhanced Edition") ] ),
		( Games::IcewindDale1, vec![ String::from("Icewind Dale Complete") ] ),
		( Games::IcewindDale1EnhancedEdition, vec![ String::from("Icewind Dale: Enhanced Edition") ] ),
		( Games::IcewindDale2, vec![ String::from("Icewind Dale 2") ] ),
		( Games::PlanescapeTorment, vec![ String::from("Planescape: Torment") ] ),
		( Games::PlanescapeTormentEnhancedEdition, vec![ String::from("Planescape: Torment - Enhanced Edition") ] ),
	]);
	
	return map.get(game).cloned();
}

//Non-Galaxy / old GOG installations
fn OldGogUninstallKeys(game: &Games) -> Option<String>
{
	let map = HashMap::from([
		( Games::BaldursGate1, String::from("GOGPACKBALDURSGATE1_is1") ),
		( Games::BaldursGate2, String::from("GOGPACKBALDURSGATE2_is1") ),
		( Games::IcewindDale1, String::from("GOGPACKICEWINDDALE1_is1") ),
	]);
	
	return map.get(game).cloned();
}

fn SearchKeys() -> Vec<String>
{
	return vec![
		String::from("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
		String::from("Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
		String::from("Software\\WOW6432Node\\GOG.com\\")
	];
}

pub fn FindInstallationPath(game: Games) -> Option<String>
{
	let gogGameId = GogGameId(&game);
	let oldGogKey = OldGogUninstallKeys(&game);
	let steamAppId = SteamAppId(&game);
	let gameDisplayNames = GameDisplayNames(&game);
	
	let mut path = None;
	
	if let Some(displayNames) = gameDisplayNames
	{
		if gogGameId != None || oldGogKey != None || steamAppId != None
		{
			let searchKeys = SearchKeys();
			for search in searchKeys
			{
				if let Ok(key) = Hive::LocalMachine.open(search, Security::Read)
				{
					for subKey in key.keys()
					{
						if let Ok(subKey) = subKey.unwrap().open(Security::Read)
						{
							let subKeyName = subKey.to_string();
							if let Some(id) = gogGameId
							{
								if subKeyName.ends_with(id.to_string().as_str())
								{
									path = ReadNewGog(&subKey, displayNames.clone());
									break;
								}
								else if subKeyName.ends_with(format!("{}_is1", id).as_str())
								{
									path = ReadInstallLocation(&subKey, displayNames.clone());
									break;
								}
							}
							
							if let Some(ref oldGog) = oldGogKey
							{
								if subKeyName.ends_with(oldGog)
								{
									path = ReadInstallLocation(&subKey, displayNames.clone());
									break;
								}
							}
							
							if let Some(id) = steamAppId
							{
								if subKeyName.ends_with(format!("Steam App {}", id).as_str())
								{
									path = ReadInstallLocation(&subKey, displayNames.clone());
									break;
								}
							}
						}
					}
					
					if path != None
					{
						break;
					}
				}
			}
		}
	}
	
	return path;
}

fn ReadNewGog(key: &RegKey, displayNames: Vec<String>) -> Option<String>
{
	let mut path = None;
	if let Ok(newId) = key.value(NewGogGameId)
	{
		if displayNames.contains(&newId.to_string())
		{
			if let Ok(installPath) = key.value(NewGogPath)
			{
				path = Some(installPath.to_string());
			}
		}
	}
	return path;
}

fn ReadInstallLocation(key: &RegKey, displayNames: Vec<String>) -> Option<String>
{
	let mut path = None;
	
	if let Ok(name) = key.value(DisplayName)
	{
		if displayNames.contains(&name.to_string())
		{
			if let Ok(installPath) = key.value(InstallLocation)
			{
				path = Some(installPath.to_string());
			}
		}
	}
	
	return path;
}

#[cfg(test)]
mod tests
{
    use super::*;
	
    #[test]
    fn FindInstallationPathTest()
	{
		assert_ne!(None, FindInstallationPath(Games::BaldursGate1));
		assert_ne!(None, FindInstallationPath(Games::BaldursGate1EnhancedEdition));
		assert_ne!(None, FindInstallationPath(Games::BaldursGate2));
		assert_ne!(None, FindInstallationPath(Games::BaldursGate2EnhancedEdition));
		assert_ne!(None, FindInstallationPath(Games::IcewindDale1));
		assert_ne!(None, FindInstallationPath(Games::IcewindDale1EnhancedEdition));
	}
}
