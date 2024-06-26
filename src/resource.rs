use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use ::glob::glob;
use crate::platform::{Games, KeyFileName};
use crate::types::{ResourceType_TIS, Are, Bif, InfinityEngineType, Key, Readable, Tis, Tlk, ReadFromFile};

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
	pub bifs: RefCell<HashMap<Games, HashMap<String, Bif>>>,
	pub keys: RefCell<HashMap<Games, Key>>,
	pub paths: RefCell<HashMap<Games, String>>,
	pub tlks: RefCell<HashMap<Games, HashMap<String, Tlk>>>,
}

impl ResourceManager
{
	/**
	Swap the file extension between `.bif` and `.BIF`, depending on the given
	file path.
	
	## Parameters
	
	- **filePath** - The `PathBuf` being altered.
	
	## Remarks
	
	Some platforms are case sensitive but the games were not developed with case
	sensitivity in mind. So when the file cannot be found, check for alternate
	file extensions based on case.
	*/
	fn alternateBifExtension(&self, filePath: PathBuf) -> Option<PathBuf>
	{
		let p = filePath.to_str()?;
		let extension = &p[p.len()-4..];
		return match extension
		{
			".bif" => Some(PathBuf::from(format!("{}.BIF", &p[..p.len()-4]))),
			_ => Some(PathBuf::from(format!("{}.bif", &p[..p.len()-4]))),
		};
	}
	
	/**
	Generate a consistently formatted `PathBuf` instance based on the given file
	name and game.
	
	## Parameters
	
	- **game** - The game used to identify the installation path.
	- **fileName** - The path, relative to the installation directory, and file
		name.
	
	## Remarks
	
	Some platforms don't recognize '\' as a path separator so, when '\' is found,
	split the string and rebuild it letting PathBuf determine the appropriate
	separator via .join().
	*/
	fn formatFilePath(&self, game: Games, fileName: String) -> Option<PathBuf>
	{
		let installPath = self.getInstallPath(game)?;
		let mut filePath = Path::new(installPath.as_str()).to_path_buf();
		
		if fileName.contains("\\")
		{
			for p in fileName.split("\\")
			{
				filePath = filePath.join(p);
			}
		}
		else
		{
			filePath = filePath.join(fileName.to_owned());
		}
		
		return Some(filePath);
	}
	
	/**
	Retrieve the installation path for a game, if one has been set.
	
	## Parameters
	
	- **game** - The game whose path is being retrieved.
	*/
	pub fn getInstallPath(&self, game: Games) -> Option<String>
	{
		return self.paths.borrow()
			.get(&game)
			.cloned();
	}
	
	/**
	Load a named ARE resource from a `Bif`'s `FileEntry` list.
	
	Relies on `ResourceManager::loadResource` to load the ARE file and its
	related files (WED).
	
	---
	
	Parameter | Description
	---|---
	game | The game which identifies the installation path from which to read.
	resourceType | The type of resource to be loaded.
	resourceName | The name of the resource to be loaded. Typically a `RESREF` value.
	
	---
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::{Are, ResourceType_ARE}};
	
	let resourceManager: ResourceManager = ResourceManager::default();
	let are: Option<Are> = resourceManager.loadAre(Games::BaldursGate1, ResourceType_ARE, "AR2600".to_string());
	assert!(are.is_some());
	```
	
	## Remarks
	
	This method searches through the resource entries in the `game`'s `Key` to
	find the appropriate `Bif` which contains the required `FileEntry`. Since
	this method relies on `loadBif` and `loadKey`, both of which cache their
	results, it will minimize the interaction with the file system when loading
	multiple resources.
	*/
	pub fn loadAre(&self, game: Games, resourceType: i16, resourceName: String) -> Option<Are>
	{
		let mut result = self.loadResource::<Are>(game, resourceType, resourceName);
		if let Some(are) = result.as_mut()
		{
			are.readWed(&self, game);
		}
		return result;
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
	let bif: Option<Bif> = resourceManager.loadKey(Games::BaldursGate1, "data\\Default.bif".to_string());
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
			let filePath = self.formatFilePath(game, fileName.clone())?;
			
			if !self.readBifFromFile(game, fileName.clone(), filePath.clone())
			{
				let _ = self.readBifFromFile(game, fileName.clone(), self.alternateBifExtension(filePath)?);
			}
		}
		
		return Some(self.bifs.borrow().get(&game)?.get(&fileName)?.to_owned());
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
			let installPath = self.getInstallPath(game)?;
			let keyFile = KeyFileName(game)?;
			let filePath = Path::new(installPath.as_str()).join(keyFile);
			
			if let Ok(instance) = ReadFromFile::<Key>(filePath.as_path())
			{
				self.keys.borrow_mut().insert(game, instance);
			}
		};
		
		return Some(self.keys.borrow().get(&game)?.to_owned());
	}
	
	/**
	Load a named resource from a `Bif`'s `FileEntry` list.
	
	---
	
	Parameter | Description
	---|---
	game | The game which identifies the installation path from which to read.
	resourceType | The type of resource to be loaded.
	resourceName | The name of the resource to be loaded. Typically a `RESREF` value.
	
	---
	
	## Generic Types
	
	- **T** - The expected output type, which must implement `InfinityEngineType`
		and `Readable`.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::{Bmp, ResourceType_BMP}};
	
	let resourceManager: ResourceManager = ResourceManager::default();
	let bmp: Option<Bmp> = resourceManager.loadResource::<Bmp>(Games::BaldursGate1, ResourceType_BMP, "AJANTISG".to_string());
	assert!(bmp.is_some());
	```
	
	## Remarks
	
	This method searches through the resource entries in the `game`'s `Key` to
	find the appropriate `Bif` which contains the required `FileEntry`. Since
	this method relies on `loadBif` and `loadKey`, both of which cache their
	results, it will minimize the interaction with the file system when loading
	multiple resources.
	*/
	pub fn loadResource<T>(&self, game: Games, resourceType: i16, resourceName: String) -> Option<T>
		where T: InfinityEngineType + Readable
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
		return match T::fromCursor(&mut cursor)
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
	let tis: Option<Tis> = resourceManager.loadTileset(Games::BaldursGate1, "AR2600".to_string());
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
			.find(|entry| entry.r#type == ResourceType_TIS as u16 && entry.name.to_string() == resourceName.to_string())?;
		
		let bifEntry = key.bifEntries.get(resourceEntry.indexBifEntry() as usize)?;
		let bif = self.loadBif(game, bifEntry.fileName.to_owned())?;
		
		let tilesetEntry = bif.tilesetEntries
			.iter()
			.find(|entry| entry.index() == resourceEntry.indexTileset())?;
		
		return tilesetEntry.data.to_owned();
	}
	
	/**
	Load a named `Tlk` file from a game's install directory.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	- **fileName** - The name of the TLK file to be loaded.
	
	## Usage
	
	```
	use crate::{platform::Games, resources::ResourceManager, types::Tlk};

	let resourceManager: ResourceManager = ResourceManager::default();
	let tlk: Option<Tlk> = resourceManager.loadTileset(Games::BaldursGate1, "dialog.tlk".to_string());
	assert!(tlk.is_some());
	```
	
	## Remarks
	
	This method searches through the game install directory and subdirectories
	to find the desired TLK file. The results are cached to minimize the
	interaction with the file system on subsequent requests.
	*/
	pub fn loadTlk(&self, game: Games, fileName: String) -> Option<Tlk>
	{
		if !self.tlks.borrow().contains_key(&game) || !self.tlks.borrow()[&game].contains_key(&fileName)
		{
			let installPath = self.getInstallPath(game)?;
			let patternString = Path::new(installPath.as_str())
				.join("**")
				.join(fileName.to_owned());
			
			if let Ok(paths) = glob(&patternString.to_str()?)
			{
				for entry in paths
				{
					if let Ok(path) = entry
					{
						if let Ok(instance) = ReadFromFile::<Tlk>(path.as_path())
						{
							let mut tlks = self.tlks.borrow_mut();
							if !tlks.contains_key(&game)
							{
								tlks.insert(game.to_owned(), HashMap::new());
							}
							
							if let Some(map) = tlks.get_mut(&game)
							{
								map.insert(fileName.to_owned(), instance);
								break;
							}
						}
					}
				}
			}
		};
		
		return Some(self.tlks.borrow().get(&game)?.get(&fileName)?.to_owned());
	}
	
	/**
	Read a Bif file at the given file path and, if successful, cache the result.
	
	## Parameters
	
	- **game** - The game which identifies the installation path from which to
		read.
	- **fileName** - The path, relative to the installation directory, and file
		name of the BIF file to load.
	- **filePath** - The `PathBuf` instance generated from the file name and
		game installation path.
	*/
	fn readBifFromFile(&self, game: Games, fileName: String, filePath: PathBuf) -> bool
	{
		if let Ok(instance) = ReadFromFile::<Bif>(filePath.as_path())
		{
			let mut bifs = self.bifs.borrow_mut();
			if !bifs.contains_key(&game)
			{
				bifs.insert(game, HashMap::new());
			}
			
			if let Some(map) = bifs.get_mut(&game)
			{
				map.insert(fileName.to_owned(), instance);
			}
			
			return true;
		}
		
		return false;
	}
	
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
	Remove a `game`'s `Tlk` from the cache.
	
	## Parameters
	
	- **game** - The game which identifies the `Tlk` list containing the `Tlk`
		to be freed.
	- **fileName** - The path, relative to the installation directory, and file
		name of the TLK file used to identify the `Tlk` to free.
	*/
	pub fn removeTlk(&self, game: Games, fileName: String)
	{
		let mut tlks = self.tlks.borrow_mut();
		if let Some(map) = tlks.get_mut(&game)
		{
			if map.contains_key(&fileName)
			{
				map.remove(&fileName);
			}
			
			if map.is_empty()
			{
				tlks.remove(&game);
			}
		}
	}
	
	/**
	Assign an installation path to a game.
	
	Only assigns paths which exist and for `Games` values which are not `Games::None`.
	
	## Parameters
	
	- **game** - The game whose path is being set.
	- **path** - The absolute path to the game's installation directory.
	*/
	pub fn setInstallPath(&self, game: Games, path: String)
	{
		if game != Games::None && Path::new(&path).exists()
		{
			self.paths.borrow_mut()
				.insert(game, path.to_owned());
		}
	}
}
