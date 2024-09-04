#[inline(always)]
pub fn two_to_u16(mut chunk: u16, length: usize) -> u16 {
    chunk <<= 16 - length * 8;
    chunk & 0x0f0f + (chunk & 0xf0f0) * 10
}

#[inline(always)]
pub fn four_to_u32(mut chunk: u32, length: usize) -> u32 {
    chunk <<= 32 - length * 8;

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
pub fn eight_to_u64(mut chunk: u64, length: usize) -> u64 {
    chunk <<= 64 - length * 8;

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
pub fn sixteen_to_u128(mut chunk: u128, length: usize) -> u128 {
    chunk <<= 128 - length * 8;

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

#[inline(always)]
pub fn whole_chunk_to_u128(mut chunk: u128) -> u128 {
    let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
    chunk = lower_digits + upper_digits;

    let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
    chunk = lower_digits + upper_digits;
    
    let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
    chunk = lower_digits + upper_digits;
    
    let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
    let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100_000_000;
    chunk = lower_digits + upper_digits;
    
    chunk
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::ptr::read_unaligned;

    #[test]
    fn test_four_to_u32() -> Result<()> {
        let u = b"1234";
        let x: &[u8] = &u[..];
        assert_eq!(four_to_u32(unsafe { read_unaligned(x.as_ptr() as *const u32) }, 4), 1234);
        Ok(())
    }

    #[test]
    fn test_eight_to_u64() -> Result<()> {
        let u = b"12345678";
        let x: &[u8] = &u[..];
        assert_eq!(eight_to_u64(unsafe { read_unaligned(x.as_ptr() as *const u64) }, 8), 12345678);
        Ok(())
    }

    #[test]
    fn test_sixteen_to_u128() -> Result<()> {
        let u = b"1234567890123456";
        let x: &[u8] = &u[..];
        assert_eq!(sixteen_to_u128(unsafe { read_unaligned(x.as_ptr() as *const u128) }, 16), 1234567890123456);
        Ok(())
    }
}