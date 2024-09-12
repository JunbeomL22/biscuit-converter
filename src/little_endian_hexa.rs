use crate::utils::{le_bytes_to_u64, le_bytes_to_u128, le_bytes_to_u32, le_bytes_to_u16};

#[inline]
#[must_use]
pub(crate) fn four_to_u32(mut chunk: u32) -> u32 {
    //chunk <<= 32 - length * 8;
    let lower_digits = (chunk & 0x0f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f) * 10;
    chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on a pair of two digits)
    let lower_digits = (chunk & 0x00ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff) * 100;
    chunk = lower_digits + upper_digits;

    chunk
}

const ZERO_COMPLEMENT_U32: u32 = 0x00CF00CF;
const NINE_COMPLEMENT_U32: u32 = 0x00C600C6;
const CHECKER_MASK_U32: u32 = 0xFF00FF00;

#[inline]
#[must_use]
pub(crate) fn check_decimal_bit_u32(chunk: u32) -> bool {
    let lower_upper_check = (((chunk & 0x00FF00FF) + NINE_COMPLEMENT_U32) & CHECKER_MASK_U32) == 0;
    let lower_lower_check = (((0x00FF00FF - (chunk & 0x00FF00FF)) + ZERO_COMPLEMENT_U32) & CHECKER_MASK_U32) != 0;
    
    let upper_upper_check = ((((chunk & 0xFF00FF00) >> 8) + NINE_COMPLEMENT_U32) & CHECKER_MASK_U32) == 0;
    let upper_lower_check = (((0xFF00FF00 - ((chunk & 0xFF00FF00) >> 8 )) + ZERO_COMPLEMENT_U32) & CHECKER_MASK_U32) != 0;

    lower_upper_check && lower_lower_check && upper_upper_check && upper_lower_check
}