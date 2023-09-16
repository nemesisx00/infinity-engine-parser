#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod bits;
mod bytes;
mod platform;
mod resource;
mod types;

use std::mem;
use std::sync::{Mutex, OnceLock};
use ::image::ImageOutputFormat;
use ::safer_ffi::prelude::*;
use platform::Games;
use resource::ResourceManager;
use types::{Bmp, ResourceType_BMP};

fn getManager() -> &'static Mutex<ResourceManager>
{
	static Manager: OnceLock<Mutex<ResourceManager>> = OnceLock::new();
	return Manager.get_or_init(|| Mutex::new(ResourceManager::default()));
}

#[ffi_export]
pub fn FreeBytes(data: repr_c::Vec<u8>) { drop(data); }

#[ffi_export]
pub fn FreeString(str: char_p::Box) { drop(str); }

#[ffi_export]
pub fn LoadResource(game: i32, resourceType: i16, resourceName: char_p::Ref<'_>) -> repr_c::Vec<u8>
{
	let result: repr_c::Vec<u8> = match resourceType
	{
		ResourceType_BMP => LoadBmp(game, resourceName.to_string()),
		_ => vec![],
	}.into();
	
	return result;
}

#[ffi_export]
pub fn ResourceSize(game: i32, resourceType: i16, resourceName: char_p::Ref<'_>) -> usize
{
	let size = match resourceType
	{
		ResourceType_BMP => SizeBmp(game, resourceName),
		_ => 0,
	};
	
	return size;
}

fn LoadBmp(game: i32, name: String) -> Vec<u8>
{
	let mut data = vec![];
	if let Ok(mut resourceManager) = getManager().lock()
	{
		if let Some(bmp) = resourceManager.loadFileResource::<Bmp>(Games::from_repr(game.to_owned()).unwrap_or(Games::None), name.to_owned())
		{
			if let Ok(image) = bmp.toImageBytes(Some(ImageOutputFormat::Png))
			{
				data = image;
			}
		}
	}
	
	return data;
}

fn SizeBmp(game: i32, resourceName: char_p::Ref<'_>) -> usize
{
	let name = resourceName.to_str();
	let mut size = 0;
	if let Ok(mut resourceManager) = getManager().lock()
	{
		if let Some(bmp) = resourceManager.loadFileResource::<Bmp>(Games::from_repr(game.to_owned()).unwrap_or(Games::None), name.to_owned())
		{
			if let Ok(image) = bmp.toImageBytes(Some(ImageOutputFormat::Png))
			{
				size = mem::size_of_val(&*image);
			}
		}
	}
	
	return size;
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	#[test]
	fn TestCache()
	{
		let game = Games::BaldursGate1;
		
		//Load a file resource.
		if let Ok(mut resourceManager) = getManager().lock()
		{
			let _ = resourceManager.loadFileResource::<Bmp>(game, "AJANTISG".to_owned());
		}
		
		//Order of the tests being run is not guaranteed, so we have to get a
		//little creative with the expected values.
		let mut keyExpected = false;
		let mut bifExpected = 0;
		if let Ok(resourceManager) = getManager().lock()
		{
			keyExpected = resourceManager.keys.contains_key(&game);
			bifExpected = match resourceManager.bifs.get(&game).is_some()
			{
				true => resourceManager.bifs.get(&game).unwrap().len(),
				false => 0,
			};
		}
		
		//Load a different file resource from the same BIF.
		if let Ok(mut resourceManager) = getManager().lock()
		{
			let _ = resourceManager.loadFileResource::<Bmp>(game, "AJANTISS".to_owned());
		}
		
		if let Ok(resourceManager) = getManager().lock()
		{
			let keyResult = resourceManager.keys.contains_key(&game);
			let bifResult = match resourceManager.bifs.get(&game).is_some()
			{
				true => resourceManager.bifs.get(&game).unwrap().len(),
				false => 0,
			};
			
			assert_eq!(keyExpected, keyResult);
			assert!(bifResult > 0);
			assert_eq!(bifExpected, bifResult);
		}
	}
	
	#[test]
	fn TestLoadBmp()
	{
		let name = "AJANTISG".to_string();
		let result = LoadBmp(Games::BaldursGate1 as i32, name);
		
		assert!(!result.is_empty());
	}
	
	#[test]
	fn TestLoadResource()
	{
		let game = Games::BaldursGate1 as i32;
		let r#type = ResourceType_BMP;
		let name = char_p::new("AJANTISG");
		let expected = LoadBmp(game, name.to_string());
		
		let result = LoadResource(game, r#type, name.as_ref());
		
		assert_eq!(expected.len(), result.len());
		
		FreeBytes(result);
		drop(name);
	}
	
	#[test]
	fn TestResourceSize()
	{
		let game = Games::BaldursGate1;
		let name = char_p::new("AJANTISG");
		let r#type = ResourceType_BMP;
		let expected = 132629;
		
		let result = ResourceSize(game as i32, r#type, name.as_ref());
		assert!(result > 0);
		assert_eq!(expected, result);
	}
}
