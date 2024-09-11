// from_le_bytes version for u128 (up to 16 bytes)
#[inline(always)]
#[must_use]
pub fn le_bytes_to_u128(input: &[u8]) -> u128 {
    let mut bytes = [0u8; 16];
    let start = 16 - input.len();
    bytes[start..].copy_from_slice(input);
    u128::from_le_bytes(bytes)
}

// from_le_bytes version for u64 (up to 8 bytes)
#[inline(always)]
#[must_use]
pub fn le_bytes_to_u64(input: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    let start = 8 - input.len();
    bytes[start..].copy_from_slice(input);
    u64::from_le_bytes(bytes)
}

// from_le_bytes version for u32 (up to 4 bytes)
#[inline(always)]
#[must_use]
pub fn le_bytes_to_u32(input: &[u8]) -> u32 {
    let mut bytes = [0u8; 4];
    let start = 4 - input.len();
    bytes[start..].copy_from_slice(input);
    u32::from_le_bytes(bytes)
}

// from_le_bytes version for u16 (up to 2 bytes)
#[inline(always)]
#[must_use]
pub fn le_bytes_to_u16(input: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    let start = 2 - input.len();
    bytes[start..].copy_from_slice(input);
    u16::from_le_bytes(bytes)
}

#[inline(always)]
#[must_use]
pub fn two_to_u16_decimal(chunk: u16) -> u16 {
    ((chunk & 0x0f00) >> 8) + (chunk & 0x000f) * 10
}

#[inline(always)]
#[must_use]
pub fn one_to_u8(chunk: u8) -> u8 {
    chunk - 0x30    
}

#[inline(always)]
#[must_use]
pub fn four_to_u32(mut chunk: u32) -> u32 {
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

#[inline(always)]
#[must_use]
pub fn eight_to_u64(mut chunk: u64) -> u64 {
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

#[inline(always)]
#[must_use]
pub fn sixteen_to_u128(mut chunk: u128) -> u128 {
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

pub fn check_decimal(input: &[u8]) -> bool {
    input.iter().all(|&x| x >= b'0' && x <= b'9')
}

#[inline]
#[must_use]
pub fn check_decimal_bit_u8(chunk: u8) -> bool {
    chunk >= 0x30 && chunk <= 0x39
}

const ZERO_COMPLEMENT_U16: u16 = 0x00CF;
const NINE_COMPLEMENT_U16: u16 = 0x00C6;
const CHECKER_MASK_U16: u16 = 0xFF00;

#[inline]
#[must_use]
pub fn check_decimal_bit_u16(chunk: u16) -> bool {
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
pub fn check_decimal_bit_u32(chunk: u32) -> bool {
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
pub fn check_decimal_bit_u64(chunk: u64) -> bool {
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
pub fn check_decimal_bit_u128(chunk: u128) -> bool {
    let lower_upper_check = (((chunk & 0x00FF00FF00FF00FF00FF00FF00FF00FF) + NINE_COMPLEMENT_U128) & CHECKER_MASK_U128) == 0;
    let lower_lower_check = (((0x00FF00FF00FF00FF00FF00FF00FF00FF - (chunk & 0x00FF00FF00FF00FF00FF00FF00FF00FF)) + ZERO_COMPLEMENT_U128) & CHECKER_MASK_U128) != 0;
    
    let upper_upper_check = ((((chunk & 0xFF00FF00FF00FF00FF00FF00FF00FF00) >> 8) + NINE_COMPLEMENT_U128) & CHECKER_MASK_U128) == 0;
    let upper_lower_check = (((0xFF00FF00FF00FF00FF00FF00FF00FF00 - ((chunk & 0xFF00FF00FF00FF00FF00FF00FF00FF00) >> 8 )) + ZERO_COMPLEMENT_U128) & CHECKER_MASK_U128) != 0;

    lower_upper_check && lower_lower_check && upper_upper_check && upper_lower_check
}

#[cfg(test)]
mod tests {
    use super::*;

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

