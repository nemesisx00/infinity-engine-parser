#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

/**
Read an arbitrary number of bytes from a `std::io::Cursor` instance.

---

Parameter | Description
---|---
$cursor | The cursor being read.
$length | The number of bytes to read.
*/
#[macro_export]
macro_rules! readBytes
{
	($cursor:expr, $length:expr) => {
		{
			let mut nameBytes = vec![];
			for _ in 0..$length
			{
				let bite = $cursor.read_u8()?;
				nameBytes.push(bite);
			}
			nameBytes
		}
	}
}

/**
Convert an array of bytes into a String.

---

Parameter | Description
---|---
$bytes | The array of bytes to be converted. Expects a slice.
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
