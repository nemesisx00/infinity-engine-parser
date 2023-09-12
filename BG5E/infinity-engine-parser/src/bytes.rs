#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

/**
Convert an array of bytes into a String.

---

Parameter | Description
--- | ---
bytes | The array of bytes to be converted. Expects a slice.
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

/**
Convert four bytes into a u32 value.

Returns a std::io::Error to the containing scope if the conversion from slice
to fixed-size array fails.

---

Parameter | Description
--- | ---
bytes | The array of bytes to be converted. Expects a slice.
*/
#[macro_export]
macro_rules! readU32
{
	($bytes:expr) => {
		{
			let val;
			match $bytes.try_into()
			{
				//TODO: Handle variable Endian-ness
				Ok(buf) => val = u32::from_le_bytes(buf),
				Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
			}
			val
		}
	}
}
