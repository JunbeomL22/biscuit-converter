use crate::error::CheckError;
use crate::utils::{
    le_bytes_to_u16,
    le_bytes_to_u32,
    le_bytes_to_u64,
    le_bytes_to_u128,
};

#[inline]
pub(crate) fn checked_conversion_u8(input: &[u8]) -> Result<u8, CheckError> {
    if input[0] >= 0x30 && input[0] <= 0x39 {
        Ok(input[0] - 0x30)
    } else {
        Err(CheckError::NonDecimal)
    }
}

#[inline]
pub(crate) fn checked_conversion_u16(input: &[u8]) -> Result<u16, CheckError> {
    let chunk = le_bytes_to_u16(input);
    if check_decimal_bit_u16(chunk) {
        Ok(two_to_u16_decimal(chunk))
    } else {
        Err(CheckError::NonDecimal)
    }
}

#[inline]
pub(crate) fn checked_conversion_u32(input: &[u8]) -> Result<u32, CheckError> {
    let chunk = le_bytes_to_u32(input);
    if check_decimal_bit_u32(chunk) {
        Ok(four_to_u32(chunk))
    } else {
        Err(CheckError::NonDecimal)
    }
}

#[inline]
pub(crate) fn checked_conversion_u64(input: &[u8]) -> Result<u64, CheckError> {
    let chunk = le_bytes_to_u64(input);
    if check_decimal_bit_u64(chunk) {
        Ok(eight_to_u64(chunk))
    } else {
        Err(CheckError::NonDecimal)
    }
    
}

#[inline]
pub(crate) fn checked_conversion_u128(input: &[u8]) -> Result<u128, CheckError> {
    let chunk = le_bytes_to_u128(input);
    if check_decimal_bit_u128(chunk) {
        Ok(sixteen_to_u128(chunk))
    } else {
        Err(CheckError::NonDecimal)
    }
}


#[inline]
#[must_use]
pub(crate) fn two_to_u16_decimal(chunk: u16) -> u16 {
    ((chunk & 0x0f00) >> 8) + (chunk & 0x000f) * 10
}


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

#[inline]
#[must_use]
pub(crate) fn eight_to_u64(mut chunk: u64) -> u64 {
    //chunk <<= 64 - length * 8;

    let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f) * 10;
    chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on 2 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff) * 100;
    chunk = lower_digits + upper_digits;

    // 4-byte mask trick (works on a pair of four digits)
    let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff) * 10000;
    chunk = lower_digits + upper_digits;

    chunk
}

#[inline]
#[must_use]
pub(crate) fn sixteen_to_u128(mut chunk: u128) -> u128 {
    let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
    chunk = lower_digits + upper_digits;
    // 2-byte mask trick (works on 4 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
    chunk = lower_digits + upper_digits;
    // 4-byte mask trick (works on 2 pairs of four digits)
    let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
    chunk = lower_digits + upper_digits;
    // 8-byte mask trick (works on a pair of eight digits)
    let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
    let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100_000_000;
    chunk = lower_digits + upper_digits;
    //
    chunk
}

const ZERO_COMPLEMENT_U16: u16 = 0x00CF;
const NINE_COMPLEMENT_U16: u16 = 0x00C6;
const CHECKER_MASK_U16: u16 = 0xFF00;

#[inline]
#[must_use]
pub(crate) fn check_decimal_bit_u16(chunk: u16) -> bool {
    let lower_upper_check = (((chunk & 0x00FF) + NINE_COMPLEMENT_U16) & CHECKER_MASK_U16) == 0;
    let lower_lower_check = (((0x00FF - (chunk & 0x00FF)) + ZERO_COMPLEMENT_U16) & CHECKER_MASK_U16) != 0;
    
    let upper_upper_check = ((((chunk & 0xFF00) >> 8) + NINE_COMPLEMENT_U16) & CHECKER_MASK_U16) == 0;
    let upper_lower_check = (((0xFF00 - ((chunk & 0xFF00) >> 8 )) + ZERO_COMPLEMENT_U16) & CHECKER_MASK_U16) != 0;

    lower_upper_check && lower_lower_check && upper_upper_check && upper_lower_check
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

const ZERO_COMPLEMENT_U64: u64 = 0x00CF00CF00CF00CF;
const NINE_COMPLEMENT_U64: u64 = 0x00C600C600C600C6;
const CHECKER_MASK_U64: u64 = 0xFF00FF00FF00FF00;

#[inline]
#[must_use]
pub(crate) fn check_decimal_bit_u64(chunk: u64) -> bool {
    let lower_upper_check = (((chunk & 0x00FF00FF00FF00FF) + NINE_COMPLEMENT_U64) & CHECKER_MASK_U64) == 0;
    let lower_lower_check = (((0x00FF00FF00FF00FF - (chunk & 0x00FF00FF00FF00FF)) + ZERO_COMPLEMENT_U64) & CHECKER_MASK_U64) != 0;
    
    let upper_upper_check = ((((chunk & 0xFF00FF00FF00FF00) >> 8) + NINE_COMPLEMENT_U64) & CHECKER_MASK_U64) == 0;
    let upper_lower_check = (((0xFF00FF00FF00FF00 - ((chunk & 0xFF00FF00FF00FF00) >> 8 )) + ZERO_COMPLEMENT_U64) & CHECKER_MASK_U64) != 0;

    lower_upper_check && lower_lower_check && upper_upper_check && upper_lower_check
}

const ZERO_COMPLEMENT_U128: u128 = 0x00CF00CF00CF00CF00CF00CF00CF00CF;
const NINE_COMPLEMENT_U128: u128 = 0x00C600C600C600C600C600C600C600C6;
const CHECKER_MASK_U128: u128 = 0xFF00FF00FF00FF00FF00FF00FF00FF00;

#[inline]
#[must_use]
pub(crate) fn check_decimal_bit_u128(chunk: u128) -> bool {
    let lower_upper_check = (((chunk & 0x00FF00FF00FF00FF00FF00FF00FF00FF) + NINE_COMPLEMENT_U128) & CHECKER_MASK_U128) == 0;
    let lower_lower_check = (((0x00FF00FF00FF00FF00FF00FF00FF00FF - (chunk & 0x00FF00FF00FF00FF00FF00FF00FF00FF)) + ZERO_COMPLEMENT_U128) & CHECKER_MASK_U128) != 0;
    
    let upper_upper_check = ((((chunk & 0xFF00FF00FF00FF00FF00FF00FF00FF00) >> 8) + NINE_COMPLEMENT_U128) & CHECKER_MASK_U128) == 0;
    let upper_lower_check = (((0xFF00FF00FF00FF00FF00FF00FF00FF00 - ((chunk & 0xFF00FF00FF00FF00FF00FF00FF00FF00) >> 8 )) + ZERO_COMPLEMENT_U128) & CHECKER_MASK_U128) != 0;

    lower_upper_check && lower_lower_check && upper_upper_check && upper_lower_check
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn check_decimal(input: &[u8]) -> bool {
        input.iter().all(|&x| (b'0'..=b'9').contains(&x))
    }
    
    #[inline]
    #[must_use]
    pub(crate) fn check_decimal_bit_u8(chunk: u8) -> bool {
        (0x30..=0x39).contains(&chunk)
    }

    #[inline]
    #[must_use]
    pub(crate) fn one_to_u8(chunk: u8) -> u8 {
        if chunk < 0x30 || chunk > 0x39 {
            return 0;
        }
        chunk - 0x30    
    }

    #[test]
    fn test_check_decimal() {
        for i in 0..100 {
            let u = format!("{}", i);
            let u = u.as_bytes();
            let chunk = le_bytes_to_u16(u);
            assert_eq!(check_decimal_bit_u16(chunk), check_decimal(u));
        }

        let u_vec = [b"1x", b"x1", b"ab", b"zy"];
        for u in u_vec.iter() {
            let chunk = le_bytes_to_u16(*u);
            assert_eq!(check_decimal_bit_u16(chunk), check_decimal(*u));
        }
    }

    #[test]
    fn test_check_decimal_u32() {
        for i in 0..10000 {
            let u = format!("{}", i);
            let u = u.as_bytes();
            let chunk = le_bytes_to_u32(u);
            assert_eq!(check_decimal_bit_u32(chunk), check_decimal(u));
        }

        let u_vec = [b"1x1x", b"x11a", b"ab11", b"zyab"];
        for u in u_vec.iter() {
            let chunk = le_bytes_to_u32(*u);
            assert_eq!(check_decimal_bit_u32(chunk), check_decimal(*u));
        }
    }

    #[test]
    fn test_check_decimal_u64() {
        for i in 0..10000 {
            let u = format!("{}", i);
            let u = u.as_bytes();
            let chunk = le_bytes_to_u64(u);
            assert_eq!(check_decimal_bit_u64(chunk), check_decimal(u));
        }

        let u_vec = [b"1x1x1x1x", b"x11a11ax", b"ab11ab11", b"yabzyabz"];
        for u in u_vec.iter() {
            let chunk = le_bytes_to_u64(*u);
            assert_eq!(check_decimal_bit_u64(chunk), check_decimal(*u));
        }

        let test_vec = vec![
            b"12345678",
            b"8765b321",
            b"zyxwvuts",
            b"00005678",
        ];

        for u in test_vec.iter() {
            let chunk = le_bytes_to_u64(*u);
            assert_eq!(check_decimal_bit_u64(chunk), check_decimal(*u));
        }
    }

    #[test]
    fn test_check_decimal_u128() {
        for i in 0..10000 {
            let u = format!("{}", i);
            let u = u.as_bytes();
            let chunk = le_bytes_to_u128(u);
            assert_eq!(check_decimal_bit_u128(chunk), check_decimal(u));
        }

        let u_vec = [b"1x1x1x1x1x1x1x1x", b"11111a11a1x1x1x1", b"ab11ab11a1x1x1x1", b"zyabzyabz1x1x1x1"];
        for u in u_vec.iter() {
            let chunk = le_bytes_to_u128(*u);
            assert_eq!(check_decimal_bit_u128(chunk), check_decimal(*u));
        }
    }

    #[test]
    fn test_two_to_u16() {
        let u = b"12";
        let x: &[u8] = &u[..];
        let x = le_bytes_to_u16(x);
        assert_eq!(two_to_u16_decimal(x), 12);
    }

    #[test]
    fn test_four_to_u32() {
        let u = b"1234";
        let x: &[u8] = &u[..];
        let x = le_bytes_to_u32(x);
        assert_eq!(four_to_u32(x), 1234);
    }

    #[test]
    fn test_eight_to_u64() {
        let u = b"12345678";
        let x: &[u8] = &u[..];
        let x = le_bytes_to_u64(x);
        assert_eq!(eight_to_u64(x), 12345678);
    }

    #[test]
    fn test_sixteen_to_u128() {
        let u = b"1234567890123456";
        let x: &[u8] = &u[..];
        let x = le_bytes_to_u128(x);
        assert_eq!(sixteen_to_u128(x), 1234567890123456);
    }
}

