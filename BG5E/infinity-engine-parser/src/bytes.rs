#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

/**
Convert an array of bytes into a String.

---

Parameter | Description
--- | ---
bytes | The array of bytes to be converted. Expects a Vec\<u8\> or slice.
*/
#[macro_export]
macro_rules! readString
{
	($bytes:expr) => {
		String::from_utf8($bytes.into())
			.map_err(|nonUtf8| String::from_utf8_lossy(nonUtf8.as_bytes()).into_owned())
			.unwrap()
	}
}
