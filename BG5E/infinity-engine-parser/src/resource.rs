#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use crate::platform::{Games, FindInstallationPath, KeyFileName};
use crate::types::{Bif, InfinityEngineType, Key, ReadFromFile};

/**
A convenient interface for retrieving resources from Infinity Engine game files.

Caches the result of any file system access, namely loading KEY and BIF files.
These cached instances are reused when the same resource is requested subsequent
times. They can also be freed manually.

The `ResourceManager` will generally always return an `Option<T>` where `T`
implements `InfinityEngineType` regardless of which load* function is called.
On some functions, such as `loadFileResource`, you must specify a type when
calling.
*/
#[derive(Clone, Debug, Default)]
pub struct ResourceManager
{
	pub keys: HashMap<Games, Key>,
	pub bifs: HashMap<Games, HashMap<String, Bif>>,
}

impl ResourceManager
{
	/**
	Remove a `game`'s `Key` from the cache.
	
	## Parameters
	
	- **game** - The game which identifies the `Key` to be freed.
	*/
	pub fn freeKey(&mut self, game: Games)
	{
		if self.keys.contains_key(&game)
		{
			self.keys.remove(&game);
		}
	}
	
	/**
	Remove a `game`'s `Bif` from the cache.
	
	## Parameters
	
	- **game** - The game which identifies the `Bif` list containing the `Bif`
		to be freed.
	- **fileName** - The path, relative to the installation directory, and file
		name of the BIF file used to identify the `Bif` to free.
	*/
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
	
	/**
	Load a `game`'s BIF file.
	
	When read from the file system, the `Bif` is stored in the `self.bifs`
	cache for reuse on subsequent calls.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	- **fileName** - The path, relative to the installation directory, and file
		name of the BIF file to load.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::Bif};
	
	let mut resourceManager: ResourceManager = ResourceManager::default();
	let bif: Option<Bif> = resourceManager.loadKey(Games::BaldursGate1, "data\\Default.bif".to_owned());
	assert!(bif.is_some());
	```
	
	## Remarks
	
	More often than not, this function will not be called directly but rather
	used internally by other more convenient `ResourceManager` functions.
	*/
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
	
	/**
	Load a `game`'s KEY file.
	
	When read from the file system, the `Key` is stored in the `self.keys` cache
	for reuse on subsequent calls.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::Key};
	
	let mut resourceManager: ResourceManager = ResourceManager::default();
	let key: Option<Key> = resourceManager.loadKey(Games::BaldursGate1);
	assert!(key.is_some());
	```
	
	## Remarks
	
	More often than not, this function will not be called directly but rather
	used internally by other more convenient `ResourceManager` functions.
	*/
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
	
	/**
	Load a named resource from a `Bif`'s `FileEntry` list.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	- **resourceName** - The name of the resource to be loaded. Typically a
		`RESREF` value.
	
	## Generic Types
	
	- **T** - The expected output type, which must implement `InfinityEngineType`.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::Bmp};

	let mut resourceManager: ResourceManager = ResourceManager::default();
	let bmp: Option<Bmp> = resourceManager.loadFileResource::<Bmp>(Games::BaldursGate1, "AJANTISG".to_owned());
	assert!(bmp.is_some());
	```
	
	## Remarks
	
	This method searches through the resource entries in the `game`'s `Key` to
	find the appropriate `Bif` which contains the required `FileEntry`. Since
	this method relies on `loadBif` and `loadKey`, both of which cache their
	results, it will minimize the interaction with the file system when loading
	multiple resources.
	*/
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
