#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod key;
mod game;
mod util;

use ::safer_ffi::prelude::*;

#[ffi_export]
fn realTest() -> char_p::Box
{
	let msg = "Hi C#, I'm Rust!".to_string();
	return msg
		.try_into()
		.unwrap();
}

#[ffi_export]
fn getBytes() -> repr_c::Vec<u8>
{
	let data = vec![4,5,3,6,2,7,1,8];
	return data.into();
}
