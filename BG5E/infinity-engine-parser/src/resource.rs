#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{
	collections::HashMap,
	io::Cursor,
	path::Path
};

use crate::{
	platform::{Games, FindInstallationPath, KeyFileName},
	types::{Bif, InfinityEngineType, Key, ReadFromFile}
};

/**
A convenient interface for retrieving resources from Infinity Engine game files.

Caches the result of any file system access, namely loading KEY and BIF files.
These cached instances are reused when the same resource is requested subsequent
times. They can also be freed manually.

# Usage

The `ResourceManager` will generally always return an `Option<T>` where `T`
implements `InfinityEngineType` regardless of which load* function is called.
On some functions, such as `loadFileResource`, you must specify a type when
calling.

```
use crate::{platform::Games, resources::ResourceManager, types::Bmp};

let bmp: Option<Bmp> = resourceManager.loadFileResource::<Bmp>(Games::BaldursGate1, "AJANTISG".to_owned());
```
*/
#[derive(Clone, Debug, Default)]
pub struct ResourceManager
{
	pub keys: HashMap<Games, Key>,
	pub bifs: HashMap<Games, HashMap<String, Bif>>,
}

impl ResourceManager
{
	pub fn freeKey(&mut self, game: Games)
	{
		if self.keys.contains_key(&game)
		{
			self.keys.remove(&game);
		}
	}
	
	pub fn freeBif(&mut self, game: Games, fileName: String)
	{
		if let Some(map) = self.bifs.get_mut(&game)
		{
			if map.contains_key(&fileName)
			{
				map.remove(&fileName);
			}
			
			if map.is_empty()
			{
				self.bifs.remove(&game);
			}
		}
	}
	
	pub fn loadKey(&mut self, game: Games) -> Option<Key>
	{
		if !self.keys.contains_key(&game)
		{
			let installPath = FindInstallationPath(game)?;
			let keyFile = KeyFileName(game)?;
			let filePath = Path::new(installPath.as_str()).join(keyFile);
			
			if let Ok(instance) = ReadFromFile::<Key>(filePath.as_path())
			{
				self.keys.insert(game, instance);
			}
		};
		
		let key = self.keys.get(&game)?;
		return Some(key.to_owned());
	}
	
	pub fn loadBif(&mut self, game: Games, fileName: String) -> Option<Bif>
	{
		if !self.bifs.contains_key(&game) || !self.bifs[&game].contains_key(&fileName)
		{
			let installPath = FindInstallationPath(game)?;
			let filePath = Path::new(installPath.as_str()).join(fileName.to_owned());
			
			if let Ok(instance) = ReadFromFile::<Bif>(filePath.as_path())
			{
				if !self.bifs.contains_key(&game)
				{
					self.bifs.insert(game, HashMap::<String, Bif>::default());
				}
				
				if let Some(map) = self.bifs.get_mut(&game)
				{
					map.insert(fileName.to_owned(), instance);
				}
			}
		}
		
		let bif = &self.bifs[&game][&fileName];
		return Some(bif.to_owned());
	}
	
	pub fn loadFileResource<T>(&mut self, game: Games, resourceName: String) -> Option<T::Output>
		where T: InfinityEngineType
	{
		let key = self.loadKey(game)?;
		let resourceEntry = key.resourceEntries
			.iter()
			.find(|entry| entry.name.eq(&resourceName))?;
		
		let bifEntry = key.bifEntries.get(resourceEntry.indexBifEntry() as usize)?;
		let bif = self.loadBif(game, bifEntry.fileName.to_owned())?;
		
		let fileEntry = bif.fileEntries
			.iter()
			.find(|entry| entry.index() == resourceEntry.indexFile())?;
		
		let mut cursor = Cursor::new(fileEntry.data.clone());
		
		return match T::fromCursor::<T>(&mut cursor)
		{
			Ok(res) => Some(res),
			Err(_) => None,
		};
	}
}
