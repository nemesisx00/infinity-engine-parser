#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::safer_ffi::derive_ReprC;

/**
Data structure for passing a resource's height and width across the FFI border.
*/
#[derive_ReprC]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Dimensions
{
	pub height: i32,
	pub width: i32,
}

impl Dimensions
{
	pub fn new(height: i32, width: i32) -> Self
	{
		return Self
		{
			height,
			width,
		};
	}
}

