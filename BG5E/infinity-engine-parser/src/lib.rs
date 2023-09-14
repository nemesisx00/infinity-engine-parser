#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod bits;
mod bytes;
mod platform;
mod resource;
mod types;

use ::image::ImageOutputFormat;
use ::safer_ffi::prelude::*;
use platform::Games;
use resource::ResourceManager;
use types::Bmp;

#[ffi_export]
fn loadBmp(gameValue: u32, resourceName: char_p::Ref<'_>) -> repr_c::Vec<u8>
{
	let name = resourceName.to_str();
	let mut data = vec![];
	if let Some(game) = Games::from_repr(gameValue)
	{
		let mut resourceManager = ResourceManager::default();
		if let Some(bmp) = resourceManager.loadFileResource::<Bmp>(game, name.to_owned())
		{
			if let Ok(image) = bmp.toImageBytes(Some(ImageOutputFormat::Png))
			{
				data = image;
			}
		}
	}
	
	return data.into();
}

#[ffi_export]
fn freeBytes(data: repr_c::Vec<u8>)
{
	drop(data);
}

/*
#[ffi_export]
fn realTest() -> char_p::Box
{
	let msg = "Hi C#, I'm Rust!".to_string();
	return msg
		.try_into()
		.unwrap();
}

#[ffi_export]
fn freeRealTest(msg: char_p::Box)
{
	drop(msg);
}
*/
