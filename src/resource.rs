#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use crate::platform::{Games, FindInstallationPath, KeyFileName};
use crate::types::{Bif, InfinityEngineType, Key, ReadFromFile, ResourceType_TIS, Tis};

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
	pub keys: RefCell<HashMap<Games, Key>>,
	pub bifs: RefCell<HashMap<Games, HashMap<String, Bif>>>,
}

impl ResourceManager
{
	/**
	Remove a `game`'s `Key` from the cache.
	
	## Parameters
	
	- **game** - The game which identifies the `Key` to be freed.
	*/
	pub fn removeKey(&self, game: Games)
	{
		let mut keys = self.keys.borrow_mut();
		if keys.contains_key(&game)
		{
			keys.remove(&game);
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
	pub fn removeBif(&self, game: Games, fileName: String)
	{
		let mut bifs = self.bifs.borrow_mut();
		if let Some(map) = bifs.get_mut(&game)
		{
			if map.contains_key(&fileName)
			{
				map.remove(&fileName);
			}
			
			if map.is_empty()
			{
				bifs.remove(&game);
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
	
	let resourceManager: ResourceManager = ResourceManager::default();
	let bif: Option<Bif> = resourceManager.loadKey(Games::BaldursGate1, "data\\Default.bif".to_owned());
	assert!(bif.is_some());
	```
	
	## Remarks
	
	More often than not, this function will not be called directly but rather
	used internally by other more convenient `ResourceManager` functions.
	*/
	pub fn loadBif(&self, game: Games, fileName: String) -> Option<Bif>
	{
		if !self.bifs.borrow().contains_key(&game) || !self.bifs.borrow()[&game].contains_key(&fileName)
		{
			let installPath = FindInstallationPath(game)?;
			let filePath = Path::new(installPath.as_str()).join(fileName.to_owned());
			
			if let Ok(instance) = ReadFromFile::<Bif>(filePath.as_path())
			{
				let mut bifs = self.bifs.borrow_mut();
				if !bifs.contains_key(&game)
				{
					bifs.insert(game, HashMap::<String, Bif>::default());
				}
				
				if let Some(map) = bifs.get_mut(&game)
				{
					map.insert(fileName.to_owned(), instance);
				}
			}
		}
		
		let bif = &self.bifs.borrow()[&game][&fileName];
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
	
	let resourceManager: ResourceManager = ResourceManager::default();
	let key: Option<Key> = resourceManager.loadKey(Games::BaldursGate1);
	assert!(key.is_some());
	```
	
	## Remarks
	
	More often than not, this function will not be called directly but rather
	used internally by other more convenient `ResourceManager` functions.
	*/
	pub fn loadKey(&self, game: Games) -> Option<Key>
	{
		if !self.keys.borrow().contains_key(&game)
		{
			let installPath = FindInstallationPath(game)?;
			let keyFile = KeyFileName(game)?;
			let filePath = Path::new(installPath.as_str()).join(keyFile);
			
			if let Ok(instance) = ReadFromFile::<Key>(filePath.as_path())
			{
				self.keys.borrow_mut().insert(game, instance);
			}
		};
		
		let key = &self.keys.borrow()[&game];
		return Some(key.to_owned());
	}
	
	/**
	Load a named resource from a `Bif`'s `FileEntry` list.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	- **resourceType** - The type of resource to be loaded.
	- **resourceName** - The name of the resource to be loaded. Typically a
		`RESREF` value.
	
	## Generic Types
	
	- **T** - The expected output type, which must implement `InfinityEngineType`.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::{Bmp, ResourceType_BMP}};
	
	let resourceManager: ResourceManager = ResourceManager::default();
	let bmp: Option<Bmp> = resourceManager.loadResource::<Bmp>(Games::BaldursGate1, ResourceType_BMP, "AJANTISG".to_owned());
	assert!(bmp.is_some());
	```
	
	## Remarks
	
	This method searches through the resource entries in the `game`'s `Key` to
	find the appropriate `Bif` which contains the required `FileEntry`. Since
	this method relies on `loadBif` and `loadKey`, both of which cache their
	results, it will minimize the interaction with the file system when loading
	multiple resources.
	*/
	pub fn loadResource<T>(&self, game: Games, resourceType: i16, resourceName: String) -> Option<T::Output>
		where T: InfinityEngineType
	{
		let key = self.loadKey(game)?;
		let resourceEntry = key.resourceEntries
			.iter()
			.find(|entry|  entry.r#type == resourceType as u16 && entry.name == resourceName)?;
		
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
	
	/**
	Load a named `Tis` resource from a `Bif`'s `TilesetEntry` list.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	- **resourceName** - The name of the resource to be loaded. Typically a
		`RESREF` value.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::Tis};

	let resourceManager: ResourceManager = ResourceManager::default();
	let tis: Option<Tis> = resourceManager.loadTileset(Games::BaldursGate1, "AJANTISG".to_owned());
	assert!(tis.is_some());
	```
	
	## Remarks
	
	This method searches through the resource entries in the `game`'s `Key` to
	find the appropriate `Bif` which contains the required `TilesetEntry`. Since
	this method relies on `loadBif` and `loadKey`, both of which cache their
	results, it will minimize the interaction with the file system when loading
	multiple resources.
	*/
	pub fn loadTileset(&self, game: Games, resourceName: String) -> Option<Tis>
	{
		let key = self.loadKey(game)?;
		let resourceEntry = key.resourceEntries
			.iter()
			.find(|entry|  entry.r#type == ResourceType_TIS as u16 && entry.name == resourceName)?;
		
		let bifEntry = key.bifEntries.get(resourceEntry.indexBifEntry() as usize)?;
		let bif = self.loadBif(game, bifEntry.fileName.to_owned())?;
		
		let tilesetEntry = bif.tilesetEntries
			.iter()
			.find(|entry| entry.index() == resourceEntry.indexTileset())?;
		
		return tilesetEntry.data.to_owned();
	}
}
