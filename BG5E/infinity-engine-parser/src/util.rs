#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub fn ReadBit(value: u32, bitIndex: u32) -> bool
{
	let check = 1 << bitIndex;
	return (value & check) == check;
}

pub fn ReadValue(value: u32, bitIndex: u32, shift: u32) -> u32
{
	return (value >> shift) & (1 << bitIndex) - 1;
}

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
		let val: u32 = 0b01111101;
		
		let mut result = ReadValue(val, 4, 0);
		assert_eq!(0b1101, result);
		
		result = ReadValue(val, 4, 4);
		assert_eq!(0b0111, result);
	}
}
