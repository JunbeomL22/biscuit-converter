use crate::BiscuitConverter;
use crate::little_endian::{
    two_to_u16,
    four_to_u32, 
    eight_to_u64, 
    sixteen_to_u128,
    le_bytes_to_u16,
    le_bytes_to_u32,
    le_bytes_to_u64,
    le_bytes_to_u128,
};
use crate::exponent::{
    exponent_u64,
    exponent_u128,
};

impl BiscuitConverter {
    #[inline]
    pub fn to_u128<T: AsRef<[u8]>>(self, input: T) -> Option<u128> {
        let u = input.as_ref();
        let length = u.len();
        assert!(length <= 39, "to_u128 works only for numbers up to 32 bytes");
        match length {
            (0..=3) => {
                let mut chunk: u32 = le_bytes_to_u32(u);
                chunk <<= 32 - length * 8;
                Some(four_to_u32(chunk) as u128)
            },
            4 => {
                let chunk: u32 = le_bytes_to_u32(u);
                Some(four_to_u32(chunk) as u128)
            }
            (5..=7) => {
                let mut chunk: u64 = le_bytes_to_u64(u);
                chunk <<= 64 - length * 8;
                Some(eight_to_u64(chunk) as u128)
            },
            8 => {
                let chunk: u64 = le_bytes_to_u64(u);
                Some(eight_to_u64(chunk) as u128)
            },
            (9..=15) => {
                let mut chunk: u128 = le_bytes_to_u128(u);
                chunk <<= 128 - length * 8;
                Some(sixteen_to_u128(chunk))
            },
            16 => {
                let chunk: u128 = le_bytes_to_u128(u);
                Some(sixteen_to_u128(chunk))
            },
            (17..=31) => {
                let (upper, lower) = u.split_at(16);
                let upper = le_bytes_to_u128(upper);
                let lower_length = lower.len();
                let mut lower = le_bytes_to_u128(lower);
                lower <<= 128 - lower_length * 8;
                Some(sixteen_to_u128(upper) * exponent_u128(length - 16) + sixteen_to_u128(lower))
            },
            32 => {
                let (upper, lower) = u.split_at(16);
                let upper = le_bytes_to_u128(upper);
                let lower = le_bytes_to_u128(lower);
                Some(sixteen_to_u128(upper) * exponent_u128(16) + sixteen_to_u128(lower))
            }
            (33..=39) => {
                let (upper, mid) = u.split_at(16);
                let (mid, lower) = mid.split_at(16);
                let upper = le_bytes_to_u128(upper);
                let mid = le_bytes_to_u128(mid);
                let lower_length = lower.len();
                let mut lower = le_bytes_to_u128(lower);
                lower <<= 128 - lower_length * 8;
                let upper_val = sixteen_to_u128(upper) * exponent_u128(length - 32);
                let mid_val = sixteen_to_u128(mid) * 10_000_000_000_000_000;
                let lower_val = sixteen_to_u128(lower);
                Some(upper_val + mid_val + lower_val)
            },
            _ => None
        }
    }
}
    /*
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
                two_to_u16(chunk)
            },
            (3..=4) => {
                let chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                four_to_u32(chunk) as u16
            },
            5 => {
                let chunk: u64 = unsafe { read_unaligned(input.as_ptr() as *const u64) };
                eight_to_u64(chunk) as u16
            },
            _ => unreachable!()
        }
    }
}
 */

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    const U128_LENGTH_BOUND: usize = 39;
    const U64_LENGTH_BOUND: usize = 19;
    const U32_LENGTH_BOUND: usize = 9;
    const U16_LENGTH_BOUND: usize = 5;

    #[test]
    fn test_to_u128() -> Result<()> {
        let biscuit = BiscuitConverter::default();

        for i in 0..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u128(x).unwrap();
            assert_eq!(val, 0);
        }

        for i in 0..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'1'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u128(x).unwrap();
            assert_eq!(val, std::str::from_utf8(x)?.parse::<u128>().unwrap());
        }


        Ok(())
    }
}