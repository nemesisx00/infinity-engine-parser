#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod global;

#[cfg(target_os = "windows")]
mod win32;

pub use global::{Games, KeyFileName};
#[cfg(target_os = "windows")]
pub use win32::FindInstallationPath;
