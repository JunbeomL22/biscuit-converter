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
pub fn two_to_u16(chunk: u16) -> u16 {
    ((chunk & 0x0f00) >> 8) + (chunk & 0x000f) * 10
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_to_u16() {
        let u = b"12";
        let x: &[u8] = &u[..];
        let x = le_bytes_to_u16(x);
        assert_eq!(two_to_u16(x), 12);
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

