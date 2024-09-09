use crate::BiscuitParser;
use crate::little_endian::{
    two_to_u16,
    four_to_u32, 
    eight_to_u64, 
    sixteen_to_u128,
};
use std::ptr::read_unaligned;

impl BiscuitParser {
    #[inline(always)]
    pub fn to_u128<T: AsRef<[u8]>>(self, input: T) -> u128 {
        let u = input.as_ref();
        let length = u.len();
        assert!(length <= 39, "to_u128 works only for numbers up to 32 bytes");
        match length {
            (0..=4) => {
                let chunk: u32 = unsafe { read_unaligned(u.as_ptr() as *const u32) };
                four_to_u32(chunk, length) as u128
            },
            (5..=8) => {
                let chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
                eight_to_u64(chunk, length) as u128
            },
            (9..=16) => {
                let chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
                sixteen_to_u128(chunk, length)
            },
            (17..=32) => {
                let (upper, lower) = u.split_at(16);
                let upper = unsafe { read_unaligned(upper.as_ptr() as *const u128) };
                let lower = unsafe { read_unaligned(lower.as_ptr() as *const u128) };
                sixteen_to_u128(upper, 16) * 10u128.pow((length - 16) as u32) + sixteen_to_u128(lower, length - 16)
            },
            (18..=39) => {
                let (upper, mid) = u.split_at(16);
                let (mid, lower) = mid.split_at(16);
                let upper = unsafe { read_unaligned(upper.as_ptr() as *const u128) };
                let middle = unsafe { read_unaligned(mid.as_ptr() as *const u128) };
                let lower = unsafe { read_unaligned(lower.as_ptr() as *const u128) };
                let upper_val = sixteen_to_u128(upper, 16) * 10u128.pow((length - 32) as u32);
                let mid_val = sixteen_to_u128(middle, 16) * 10u128.pow(16);
                let lower_val = sixteen_to_u128(lower, length - 16);
                upper_val + mid_val + lower_val
            },
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn to_u64<T: AsRef<[u8]>>(self, input: T) -> u64 {
        let u = input.as_ref();
        let length = u.len();
        assert!(length <= 19, "to_u64 works only for numbers up to 19 bytes");
        match length {
            (0..=4) => {
                let chunk: u32 = unsafe { read_unaligned(u.as_ptr() as *const u32) };
                four_to_u32(chunk, length) as u64
            },
            (5..=8) => {
                let chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
                eight_to_u64(chunk, length)
            },
            (9..=16) => {
                let chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
                sixteen_to_u128(chunk, length) as u64
            },
            (17..=19) => {
                let (upper, lower) = u.split_at(16);
                let upper = unsafe { read_unaligned(upper.as_ptr() as *const u128) };
                let lower = unsafe { read_unaligned(lower.as_ptr() as *const u128) };
                (sixteen_to_u128(upper, 16) * 10u128.pow((length - 16) as u32) + sixteen_to_u128(lower, length - 16)) as u64
            },
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn to_u32<T: AsRef<[u8]>>(self, input: T) -> u32 {
        let input = input.as_ref();
        let length = input.len();
        assert!(length <= 9, "to_u32 works only for numbers up to 9 bytes");
        match length {
            (0..=4) => {
                let chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                four_to_u32(chunk, length)
            },
            (5..=8) => {
                let chunk: u64 = unsafe { read_unaligned(input.as_ptr() as *const u64) };
                eight_to_u64(chunk, length) as u32
            },
            9 => {
                let chunk: u128 = unsafe { read_unaligned(input.as_ptr() as *const u128) };
                sixteen_to_u128(chunk, length) as u32
            },
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn to_u16<T: AsRef<[u8]>>(self, input: T) -> u16 {
        let input = input.as_ref();
        let length = input.len();
        assert!(length <= 5, "to_u16 works only for numbers up to 5 bytes");
        match length {
            (0..=2) => {
                let chunk: u16 = unsafe { read_unaligned(input.as_ptr() as *const u16) };
                two_to_u16(chunk, length)
            },
            (3..=4) => {
                let chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                four_to_u32(chunk, length) as u16
            },
            5 => {
                let chunk: u64 = unsafe { read_unaligned(input.as_ptr() as *const u64) };
                eight_to_u64(chunk, length) as u16
            },
            _ => unreachable!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_to_u64() -> Result<()> {
        let flash_parser = BiscuitParser::default();
        assert_eq!(flash_parser.to_u16("1234"), 1234);
        assert_eq!(flash_parser.to_u32("12345"), 12345);
        assert_eq!(flash_parser.to_u32("1234567"), 1234567);
        assert_eq!(flash_parser.to_u32("123456789"), 123456789);
        assert_eq!(flash_parser.to_u64("123456789012"), 123456789012);
        assert_eq!(flash_parser.to_u128("123456789012345678901"), 123456789012345678901);
        Ok(())
    }
}