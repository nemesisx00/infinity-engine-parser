#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod global;

#[cfg(target_os = "windows")]
mod win32;
