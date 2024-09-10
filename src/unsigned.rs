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
use crate::exponent::exponent_u128;

impl BiscuitConverter {
    pub fn to_u64<T: AsRef<[u8]>>(self, input: T) -> Option<u64> {
        let u = input.as_ref();
        let length = u.len();
        if length == 0 {
            return None;
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 20, "to_u64 works only for numbers up to 20 bytes");
        match length {
            0 => Some(0),
            1 => Some((u[0] -b'0') as u64),
            2=> Some(two_to_u16(le_bytes_to_u16(u)) as u64),
            3 => {
                let upper = two_to_u16(le_bytes_to_u16(&u[..2])) as u64;
                let lower = (u[2] - b'0') as u64;
                Some(upper * 10 + lower)
            },
            4 => Some(four_to_u32(le_bytes_to_u32(u)) as u64),
            5 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u64;
                let lower = (u[4] - b'0') as u64;
                Some(upper * 10 + lower)
            }
            6 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u64;
                let lower = two_to_u16(le_bytes_to_u16(&u[4..])) as u64;
                Some(upper * 100 + lower)
            },
            7 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u64;
                let mid = two_to_u16(le_bytes_to_u16(&u[4..6])) as u64;
                let lower = (u[6] - b'0') as u64;
                Some(upper * 1_000 + mid * 10 + lower)
            },
            8 => Some(eight_to_u64(le_bytes_to_u64(u))),
            9 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let lower = (u[8] - b'0') as u64;
                Some(upper * 10 + lower)
            },
            10 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let lower = two_to_u16(le_bytes_to_u16(&u[8..])) as u64;
                Some(upper * 100 + lower)
            },
            11 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let mid = two_to_u16(le_bytes_to_u16(&u[8..10])) as u64;
                let lower = (u[10] - b'0') as u64;
                Some(upper * 1_000 + mid * 10 + lower)
            },
            12 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let lower = four_to_u32(le_bytes_to_u32(&u[8..12]));
                Some(upper * 10_000 + lower as u64)
            },
            13 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u64;
                let lower = (u[12] - b'0') as u64;
                Some(upper * 100_000 + mid * 10 + lower)
            },
            14 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u64;
                let lower = two_to_u16(le_bytes_to_u16(&u[12..])) as u64;
                Some(upper * 1_000_000 + mid * 100 + lower)
            },
            15 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u64;
                let lower = two_to_u16(le_bytes_to_u16(&u[12..14])) as u64;
                let tail = (u[14] - b'0') as u64;
                Some(upper * 10_000_000 + mid * 1_000 + lower * 10 + tail)
            },
            16 => Some(sixteen_to_u128(le_bytes_to_u128(u)) as u64),
            17 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16])) as u64;
                let lower = (u[16] - b'0') as u64;
                Some(upper * 10 + lower)
            },
            18 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16])) as u64;
                let lower = two_to_u16(le_bytes_to_u16(&u[16..])) as u64;
                Some(upper * 100 + lower)
            },
            19 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16])) as u64;
                let mid = two_to_u16(le_bytes_to_u16(&u[16..18])) as u64;
                let lower = (u[18] - b'0') as u64;
                Some((upper * 1_000).checked_add(mid * 10 + lower)?)
                        
            }
            20 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16])) as u64;
                let lower = four_to_u32(le_bytes_to_u32(&u[16..])) as u64;
                Some((upper * 10_000).checked_add(lower)?)
            }
            _ => None,
        }
    }

    pub fn to_u128<T: AsRef<[u8]>>(self, input: T) -> Option<u128> {
        let u = input.as_ref();
        let length = u.len();
        if length == 0 {
            return None;
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 39, "to_u128 works only for numbers up to 39 bytes");
        match length {
            0 => Some(0),
            1 => Some((u[0] - b'0') as u128),
            2 => Some(two_to_u16(le_bytes_to_u16(u)) as u128),
            3 => {
                let upper = two_to_u16(le_bytes_to_u16(&u[..2])) as u128;
                let lower = (u[2] - b'0') as u128;
                Some(upper * 10 + lower)
            },
            4 => Some(four_to_u32(le_bytes_to_u32(u)) as u128),
            5 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u128;
                let lower = (u[4] - b'0') as u128;
                Some(upper * 10 + lower)
            },
            6 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[4..])) as u128;
                Some(upper * 100 + lower)
            },
            7 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u128;
                let mid = two_to_u16(le_bytes_to_u16(&u[4..6])) as u128;
                let lower = (u[6] - b'0') as u128;
                Some(upper * 1_000 + mid * 10 + lower)
            },
            8 => Some(eight_to_u64(le_bytes_to_u64(u)) as u128),
            9 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let lower = (u[8] - b'0') as u128;
                Some(upper * 10 + lower)
            },
            10 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[8..])) as u128;
                Some(upper * 100 + lower)
            },
            11 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let mid = two_to_u16(le_bytes_to_u16(&u[8..10])) as u128;
                let lower = (u[10] - b'0') as u128;
                Some(upper * 1_000 + mid * 10 + lower)
            },
            12 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let lower = four_to_u32(le_bytes_to_u32(&u[8..12])) as u128;
                Some(upper * 10_000 + lower)
            },
            13 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u128;
                let lower = (u[12] - b'0') as u128;
                Some(upper * 100_000 + mid * 10 + lower)
            },
            14 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[12..])) as u128;
                Some(upper * 1_000_000 + mid * 100 + lower)
            },
            15 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[12..14])) as u128;
                let tail = (u[14] - b'0') as u128;
                Some(upper * 10_000_000 + mid * 1_000 + lower * 10 + tail)
            },
            16 => Some(sixteen_to_u128(le_bytes_to_u128(u))),
            17 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = (u[16] - b'0') as u128;
                Some(upper * 10 + lower)
            },
            18 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = two_to_u16(le_bytes_to_u16(&u[16..])) as u128;
                Some(upper * 100 + lower)
            },
            19 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = two_to_u16(le_bytes_to_u16(&u[16..18])) as u128;
                let lower = (u[18] - b'0') as u128;
                Some(upper * 1_000 + mid * 10 + lower)
            },
            20 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = four_to_u32(le_bytes_to_u32(&u[16..])) as u128;
                Some(upper * 10_000 + lower)
            },
            21 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = four_to_u32(le_bytes_to_u32(&u[16..20])) as u128;
                let lower = (u[20] - b'0') as u128;
                Some(upper * 100_000 + mid * 10 + lower)
            },
            22 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = four_to_u32(le_bytes_to_u32(&u[16..20])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[20..])) as u128;
                Some(upper * 1_000_000 + mid * 100 + lower)
            },
            23 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = four_to_u32(le_bytes_to_u32(&u[16..20])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[20..22])) as u128;
                let tail = (u[22] - b'0') as u128;
                Some(upper * 10_000_000 + mid * 1_000 + lower * 10 + tail)
            },
            24 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = eight_to_u64(le_bytes_to_u64(&u[16..])) as u128;
                Some(upper * 100_000_000 + lower)
            },
            25 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = (u[24] - b'0') as u128;
                Some(upper * 1_000_000_000 + mid * 10 + lower)
            },
            26 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[24..])) as u128;
                Some(upper * 10_000_000_000 + mid * 100 + lower)
            },
            27 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = two_to_u16(le_bytes_to_u16(&u[24..26])) as u128;
                let tail = (u[26] - b'0') as u128;
                Some(upper * 100_000_000_000 + mid * 1_000 + lower * 10 + tail)
            },
            28 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = four_to_u32(le_bytes_to_u32(&u[24..28])) as u128;
                Some(upper * 1_000_000_000_000 + mid * 10_000 + lower)
            },
            29 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = four_to_u32(le_bytes_to_u32(&u[24..28])) as u128;
                let tail = (u[28] - b'0') as u128;
                Some(upper * 10_000_000_000_000 + mid * 100_000 + lower * 10 + tail)
            },
            30 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = four_to_u32(le_bytes_to_u32(&u[24..28])) as u128;
                let tail = two_to_u16(le_bytes_to_u16(&u[28..])) as u128;
                Some(upper * 100_000_000_000_000 + mid * 1_000_000 + lower * 100 + tail)
            },
            31 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = four_to_u32(le_bytes_to_u32(&u[24..28])) as u128;
                let tail = two_to_u16(le_bytes_to_u16(&u[28..30])) as u128;
                let last = (u[30] - b'0') as u128;
                Some(upper * 1_000_000_000_000_000 + mid * 10_000_000 + lower * 1_000 + tail * 10 + last)
            },
            32 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = sixteen_to_u128(le_bytes_to_u128(&u[16..]));
                Some(upper.wrapping_mul(10_000_000_000_000_000).checked_add(lower)?)
            },
            33 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let last = (u[32] - b'0') as u128;
                Some(upper.wrapping_mul(100_000_000_000_000_000).checked_add(lower * 10)?.checked_add(last)?)
            },
            34 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let mid = two_to_u16(le_bytes_to_u16(&u[32..])) as u128;
                Some(upper.wrapping_mul(1_000_000_000_000_000_000).checked_add(lower * 100)?.checked_add(mid)?)
            },
            35 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let mid = two_to_u16(le_bytes_to_u16(&u[32..34])) as u128;
                let last = (u[34] - b'0') as u128;
                Some(upper.wrapping_mul(10_000_000_000_000_000_000).checked_add(lower * 1_000)?.checked_add(mid * 10)?.checked_add(last)?)
            },
            36 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let lower = four_to_u32(le_bytes_to_u32(&u[32..36])) as u128;
                Some(upper.wrapping_mul(100_000_000_000_000_000_000).checked_add(mid * 10_000)?.checked_add(lower)?)
            },
            37 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let lower = four_to_u32(le_bytes_to_u32(&u[32..36])) as u128;
                let last = (u[36] - b'0') as u128;
                Some(upper.wrapping_mul(1_000_000_000_000_000_000_000).checked_add(mid * 100_000)?.checked_add(lower * 10)?.checked_add(last)?)
            },
            38 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let lower = four_to_u32(le_bytes_to_u32(&u[32..36])) as u128;
                let tail = two_to_u16(le_bytes_to_u16(&u[36..])) as u128;
                Some(upper.wrapping_mul(10_000_000_000_000_000_000_000).checked_add(mid * 1_000_000)?.checked_add(lower * 100)?.checked_add(tail)?)
            },
            39 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let lower = four_to_u32(le_bytes_to_u32(&u[32..36])) as u128;
                let tail = two_to_u16(le_bytes_to_u16(&u[36..38])) as u128;
                let last = (u[38] - b'0') as u128;
                Some(upper.wrapping_mul(100_000_000_000_000_000_000_000).checked_add(mid * 10_000_000)?.checked_add(lower * 1_000)?.checked_add(tail * 10)?.checked_add(last)?)
            },
            _ => None
        }
    }

    pub fn to_u32<T: AsRef<[u8]>>(self, input: T) -> Option<u32> {
        let u = input.as_ref();
        let length = u.len();
        if length == 0 {
            return None;
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 10, "to_u32 works only for numbers up to 10 bytes");
        match length {
            0 => Some(0),
            1 => Some((u[0] - b'0') as u32),
            2 => Some(two_to_u16(le_bytes_to_u16(u)) as u32),
            3 => {
                let upper = two_to_u16(le_bytes_to_u16(&u[..2])) as u32;
                let lower = (u[2] - b'0') as u32;
                Some(upper * 10 + lower)
            },
            4 => Some(four_to_u32(le_bytes_to_u32(u))),
            5 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4]));
                let lower = (u[4] - b'0') as u32;
                Some(upper * 10 + lower)
            },
            6 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4]));
                let lower = two_to_u16(le_bytes_to_u16(&u[4..])) as u32;
                Some(upper * 100 + lower)
            },
            7 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4]));
                let mid = two_to_u16(le_bytes_to_u16(&u[4..6])) as u32;
                let lower = (u[6] - b'0') as u32;
                Some(upper * 1_000 + mid * 10 + lower)
            },
            8 => Some(eight_to_u64(le_bytes_to_u64(u)) as u32),
            9 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u32;
                let lower = (u[8] - b'0') as u32;
                Some(upper.wrapping_mul(10).checked_add(lower)?)
            },
            10 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u32;
                let lower = two_to_u16(le_bytes_to_u16(&u[8..])) as u32;
                Some(upper.wrapping_mul(100).checked_add(lower)?)
            },
            _ => None
        }
    }

    pub fn to_u16<T: AsRef<[u8]>>(self, input: T) -> Option<u16> {
        let u = input.as_ref();
        let length = u.len();
        if length == 0 {
            return None;
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 5, "to_u32 works only for numbers up to 5 bytes");

        match u.len() {
            0 => Some(0),
            1 => Some((u[0] - b'0') as u16),
            2 => Some(two_to_u16(le_bytes_to_u16(u))),
            3 => {
                let upper = two_to_u16(le_bytes_to_u16(&u[..2]));
                let lower = (u[2] - b'0') as u16;
                Some(upper * 10 + lower)
            },
            4 => Some(four_to_u32(le_bytes_to_u32(u)) as u16),
            5 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u16;
                let lower = (u[4] - b'0') as u16;
                Some(upper.wrapping_mul(10).checked_add(lower)?)
            },
            _ => None,
        }
    }
}
