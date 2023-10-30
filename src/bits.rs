#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

/**
Read the value of a specific bit within a given value.

---

Parameter | Description
--- | ---
value | The value whose bits are being read.
bitIndex | The index of the specific bit to read.

---

**Returns**: The boolean value of the bit.
*/
pub fn ReadBit(value: u32, bitIndex: u32) -> bool
{
	let check = 1 << bitIndex;
	return (value & check) == check;
}

/**
Read a bitwise subsection of a given value.

Using right bit shift and a bit mask, read a specific subset of bits from the
given value.

---

Parameter | Description
--- | ---
value | The value whose bits are being read.
bitIndex | The index of the most significant bit to read. Used to generate the mask.
shift | The number of bits to shift to the right before applying the mask.

---

**Returns**: The numeric value of the bits which were read.
*/
pub fn ReadValue(value: u64, bitIndex: u64, shift: u64) -> u64
{
	return (value >> shift) & (1 << bitIndex) - 1;
}

// --------------------------------------------------

#[cfg(test)]
mod tests
{
    use super::*;
	
    #[test]
    fn ReadBitTest()
	{
		let expected = true;
		let val: u32 = 0b0111;
		
		let mut result = ReadBit(val, 2);
		assert_eq!(expected, result);
		
		result = ReadBit(val, 1);
		assert_eq!(expected, result);
		
		result = ReadBit(val, 0);
		assert_eq!(expected, result);
    }
	
	#[test]
	fn ReadValueTest()
	{
		let val: u64 = 0b01111101;
		
		let mut result = ReadValue(val, 4, 0);
		assert_eq!(0b1101, result);
		
		result = ReadValue(val, 4, 4);
		assert_eq!(0b0111, result);
	}
}
