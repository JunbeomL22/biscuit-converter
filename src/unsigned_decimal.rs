use crate::{exponent, BiscuitConverter};
use crate::error::{
    AdditionOverflow, 
    CheckError, 
    Empty, 
    OverFlow, 
    NonDecimal,
};
use crate::exponent::{
    exponent_u64,
    exponent_u128,
};

use crate::little_endian_decimal::{
    one_to_u8,
    two_to_u16_decimal,
    four_to_u32, 
    eight_to_u64, 
    sixteen_to_u128,
    le_bytes_to_u16,
    le_bytes_to_u32,
    le_bytes_to_u64,
    le_bytes_to_u128,
    check_decimal_bit_u128,
    check_decimal_bit_u64,
    check_decimal_bit_u32,
    check_decimal_bit_u16,
    check_decimal_bit_u8,
};

impl BiscuitConverter {
    pub fn to_u128_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<u128, CheckError> {
        let u = input.as_ref();
        let mut length = u.len();
        if length == 0 {
            return Err(CheckError::Empty(Empty));
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        length = length - start;
        
        if length > 39 {
            return Err(CheckError::OverFlow(OverFlow));
        }
        
        let mut result: u128 = 0;
        let mut upper = u;
        let mut lower = u;
        
        if length >= 32 {
            // Process first 16 bytes
            upper = &u[..16];
            let chunk_u128 = le_bytes_to_u128(upper);
            if !check_decimal_bit_u128(chunk_u128) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result = sixteen_to_u128(chunk_u128) * exponent_u128(length - 16);
            
            // Process next 16 bytes
            upper = &u[16..32];
            let chunk_u128 = le_bytes_to_u128(upper);
            if !check_decimal_bit_u128(chunk_u128) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            if let Some(res) = result.checked_add(sixteen_to_u128(chunk_u128) * exponent_u128(length - 32)) {
                result = res;
            } else {
                return Err(CheckError::AdditionOverflow(AdditionOverflow));
            }
            
            length -= 32;
            if length > 0 {
                lower = &u[32..];
            }
        } else if length >= 16 {
            upper = &lower[..16];
            let chunk_u128 = le_bytes_to_u128(upper);
            if !check_decimal_bit_u128(chunk_u128) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result += sixteen_to_u128(chunk_u128) * exponent_u128(length - 16);
            length -= 16;
            if length > 0 {
                lower = &lower[16..];
            }
        } else if length >= 8 {
            upper = &lower[..8];
            let chunk_u64 = le_bytes_to_u64(upper);
            if !check_decimal_bit_u64(chunk_u64) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result += eight_to_u64(chunk_u64) as u128 * exponent_u128(length - 8);
            length -= 8;
            if length > 0 {
                lower = &lower[8..];
            }
        } else if length >= 4 {
            upper = &lower[..4];
            let chunk_u32 = le_bytes_to_u32(upper);
            if !check_decimal_bit_u32(chunk_u32) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result += (four_to_u32(chunk_u32) as u128) * exponent_u128(length - 4);
            length -= 4;
            if length > 0 {
                lower = &lower[4..];
            }
        } else if length >= 2 {
            upper = &upper[..2];
            let chunk_u16 = le_bytes_to_u16(upper);
            if !check_decimal_bit_u16(chunk_u16) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            if let Some(res) = result.checked_add(two_to_u16_decimal(chunk_u16) as u128 * exponent_u128(length - 2)) {
                result = res;
            } else {
                return Err(CheckError::AdditionOverflow(AdditionOverflow));
            }
            length -= 2;
            if length > 0 {
                lower = &lower[2..];
            }
        } else if length == 1 {
            upper = &lower[..1];
            if !check_decimal_bit_u8(upper[0]) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            if let Some(res) = result.checked_add(one_to_u8(upper[0]) as u128) {
                result = res;
            } else {
                return Err(CheckError::AdditionOverflow(AdditionOverflow));
            }
        } else {
            // it has been checked that the length is not zero
            return Ok(0);
        }
        
        Ok(result)
    }

    pub fn to_u64_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<u64, CheckError> {
        let u = input.as_ref();
        let mut length = u.len();
        if length == 0 {
            return Err(CheckError::Empty(Empty));
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        length = length - start;
        
        if length > 20 {
            return Err(CheckError::OverFlow(OverFlow));
        }
        
        let mut result= 0;
        let mut upper = u;
        let mut lower = u;
        
        if length >= 16 {
            upper = &u[..16];
            let chunk_u128 = le_bytes_to_u128(upper);
            if !check_decimal_bit_u128(chunk_u128) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result = sixteen_to_u128(chunk_u128) as u64 * exponent_u64(length - 16);
            length -= 16;
            if length > 0 {
                lower = &u[16..];
            }
        } else if length >= 8 {
            upper = &lower[..8];
            let chunk_u64 = le_bytes_to_u64(upper);
            if !check_decimal_bit_u64(chunk_u64) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result += eight_to_u64(chunk_u64) * exponent_u64(length - 8);
            length -= 8;
            if length > 0 {
                lower = &lower[8..];
            }
        } else if length >= 4 {
            upper = &lower[..4];
            let chunk_u32 = le_bytes_to_u32(upper);
            if !check_decimal_bit_u32(chunk_u32) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            if let Some(res) = result.checked_add(four_to_u32(chunk_u32) as u64 * exponent_u64(length - 4)) {
                result = res;
            } else {
                return Err(CheckError::AdditionOverflow(AdditionOverflow));
            }
            length -= 4;
            if length > 0 {
                lower = &lower[4..];
            }
        } else if length >= 2 {
            upper = &upper[..2];
            let chunk_u16 = le_bytes_to_u16(upper);
            if !check_decimal_bit_u16(chunk_u16) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result += two_to_u16_decimal(chunk_u16) as u64 * exponent_u64(length - 2);
            length -= 2;
            if length > 0 {
                lower = &lower[2..];
            }
        } else if length == 1 {
            upper = &lower[..1];
            if !check_decimal_bit_u8(upper[0]) {
                return Err(CheckError::NonDecimal(NonDecimal));
            }
            result += one_to_u8(upper[0]) as u64;
        } else {
            // it has been checked that the length is not zero
            return Ok(0);
        }
        
        Ok(result)
    }


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
            2=> Some(two_to_u16_decimal(le_bytes_to_u16(u)) as u64),
            3 => {
                let upper = two_to_u16_decimal(le_bytes_to_u16(&u[..2])) as u64;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[4..])) as u64;
                Some(upper * 100 + lower)
            },
            7 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u64;
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[4..6])) as u64;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[8..])) as u64;
                Some(upper * 100 + lower)
            },
            11 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[8..10])) as u64;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[12..])) as u64;
                Some(upper * 1_000_000 + mid * 100 + lower)
            },
            15 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8]));
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u64;
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[12..14])) as u64;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[16..])) as u64;
                Some(upper * 100 + lower)
            },
            19 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16])) as u64;
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[16..18])) as u64;
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
            2 => Some(two_to_u16_decimal(le_bytes_to_u16(u)) as u128),
            3 => {
                let upper = two_to_u16_decimal(le_bytes_to_u16(&u[..2])) as u128;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[4..])) as u128;
                Some(upper * 100 + lower)
            },
            7 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u128;
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[4..6])) as u128;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[8..])) as u128;
                Some(upper * 100 + lower)
            },
            11 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[8..10])) as u128;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[12..])) as u128;
                Some(upper * 1_000_000 + mid * 100 + lower)
            },
            15 => {
                let upper = eight_to_u64(le_bytes_to_u64(&u[..8])) as u128;
                let mid = four_to_u32(le_bytes_to_u32(&u[8..12])) as u128;
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[12..14])) as u128;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[16..])) as u128;
                Some(upper * 100 + lower)
            },
            19 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[16..18])) as u128;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[20..])) as u128;
                Some(upper * 1_000_000 + mid * 100 + lower)
            },
            23 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = four_to_u32(le_bytes_to_u32(&u[16..20])) as u128;
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[20..22])) as u128;
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
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[24..])) as u128;
                Some(upper * 10_000_000_000 + mid * 100 + lower)
            },
            27 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = two_to_u16_decimal(le_bytes_to_u16(&u[24..26])) as u128;
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
                let tail = two_to_u16_decimal(le_bytes_to_u16(&u[28..])) as u128;
                Some(upper * 100_000_000_000_000 + mid * 1_000_000 + lower * 100 + tail)
            },
            31 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = eight_to_u64(le_bytes_to_u64(&u[16..24])) as u128;
                let lower = four_to_u32(le_bytes_to_u32(&u[24..28])) as u128;
                let tail = two_to_u16_decimal(le_bytes_to_u16(&u[28..30])) as u128;
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
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[32..])) as u128;
                Some(upper.wrapping_mul(1_000_000_000_000_000_000).checked_add(lower * 100)?.checked_add(mid)?)
            },
            35 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let lower = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let mid = two_to_u16_decimal(le_bytes_to_u16(&u[32..34])) as u128;
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
                let tail = two_to_u16_decimal(le_bytes_to_u16(&u[36..])) as u128;
                Some(upper.wrapping_mul(10_000_000_000_000_000_000_000).checked_add(mid * 1_000_000)?.checked_add(lower * 100)?.checked_add(tail)?)
            },
            39 => {
                let upper = sixteen_to_u128(le_bytes_to_u128(&u[..16]));
                let mid = sixteen_to_u128(le_bytes_to_u128(&u[16..32]));
                let lower = four_to_u32(le_bytes_to_u32(&u[32..36])) as u128;
                let tail = two_to_u16_decimal(le_bytes_to_u16(&u[36..38])) as u128;
                let last = (u[38] - b'0') as u128;
                Some(upper.wrapping_mul(100_000_000_000_000_000_000_000).checked_add(mid * 10_000_000)?.checked_add(lower * 1_000)?.checked_add(tail * 10)?.checked_add(last)?)
            },
            _ => None
        }
    }

    pub fn to_u32_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<u32, CheckError> {
        let u: &[u8] = input.as_ref();
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty(Empty));
        }

        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 10, "to_u32 works only for numbers up to 10 bytes");
        match length {
            0 => {
                // We checked empty above, so this means all zeros
                Ok(0)
            }
            1 => {
                if u[0] >= b'0' && u[0] <= b'9' {
                    Ok((u[0] - b'0') as u32)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            2 => {
                let chunk = le_bytes_to_u16(u);
                if check_decimal_bit_u16(chunk) {
                    Ok(two_to_u16_decimal(chunk) as u32)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            }
            3 => {
                let upper = le_bytes_to_u16(&u[..2]);
                if !(check_decimal_bit_u16(upper) && check_decimal_bit_u8(u[2])) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = two_to_u16_decimal(upper) as u32;
                    let lower = (u[2] - b'0') as u32;
                    Ok(upper * 10 + lower)
                }
            },
            4 => {
                let chunk = le_bytes_to_u32(u);
                if check_decimal_bit_u32(chunk) {
                    Ok(four_to_u32(chunk) as u32)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            }
            5 => {
                let upper = le_bytes_to_u32(&u[..4]);
                if !(check_decimal_bit_u32(upper) && check_decimal_bit_u8(u[4])) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = four_to_u32(upper) as u32;
                    let lower = (u[4] - b'0') as u32;
                    if let Some(res) = (upper * 10).checked_add(lower) {
                        Ok(res)
                    } else {
                        Err(CheckError::AdditionOverflow(AdditionOverflow))
                    }
                }
            },
            6 => {
                let upper_chunk = le_bytes_to_u32(&u[..4]);
                let lower_chunk = le_bytes_to_u16(&u[4..]);
                if !check_decimal_bit_u32(upper_chunk) || !check_decimal_bit_u16(lower_chunk) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = four_to_u32(upper_chunk) as u32;
                    let lower = two_to_u16_decimal(lower_chunk) as u32;
                    Ok(upper * 100 + lower)
                }
            },
            7 => {
                let upper_chunk = le_bytes_to_u32(&u[..4]);
                let mid_chunk = le_bytes_to_u16(&u[4..6]);
                let lower_chunk = u[6];
                if !check_decimal_bit_u32(upper_chunk) || !check_decimal_bit_u16(mid_chunk) || !check_decimal_bit_u8(lower_chunk) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = four_to_u32(upper_chunk) as u32;
                    let mid = two_to_u16_decimal(mid_chunk) as u32;
                    let lower = (lower_chunk - b'0') as u32;
                    Ok(upper * 1_000 + mid * 10 + lower)
                }
            },
            8 => {
                let chunk = le_bytes_to_u64(u);
                if check_decimal_bit_u64(chunk) {
                    Ok(eight_to_u64(chunk) as u32)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            9 => {
                let upper_chunk = le_bytes_to_u64(&u[..8]);
                let lower_chunk = u[8];
                if !check_decimal_bit_u64(upper_chunk) || !check_decimal_bit_u8(lower_chunk) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = eight_to_u64(upper_chunk) as u32;
                    let lower = (lower_chunk - b'0') as u32;
                    Ok(upper * 10 + lower)
                }
            },
            10 => {
                let upper_chunk = le_bytes_to_u64(&u[..8]);
                let lower_chunk = le_bytes_to_u16(&u[8..]);
                if !check_decimal_bit_u64(upper_chunk) || !check_decimal_bit_u16(lower_chunk) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = eight_to_u64(upper_chunk) as u32;
                    let lower = two_to_u16_decimal(lower_chunk) as u32;
                    if let Some(res) = (upper * 100).checked_add(lower) {
                        Ok(res)
                    } else {
                        Err(CheckError::AdditionOverflow(AdditionOverflow))
                    }
                }
            },
            _ => Err(CheckError::OverFlow(OverFlow)),
        }
    }

    /// # Safety
    /// This function is unsafe because it doesn't check the input for validity.
    pub unsafe fn to_u16_unchecked<T: AsRef<[u8]>>(self, input: T) -> Result<u16, CheckError> {
        let u = input.as_ref();
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty(Empty));
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 5, "to_u32 works only for numbers up to 5 bytes");

        match u.len() {
            0 => Ok(0),
            1 => Ok((u[0] - b'0') as u16),
            2 => Ok(two_to_u16_decimal(le_bytes_to_u16(u))),
            3 => {
                let upper = two_to_u16_decimal(le_bytes_to_u16(&u[..2]));
                let lower = (u[2] - b'0') as u16;
                Ok(upper * 10 + lower)
            },
            4 => Ok(four_to_u32(le_bytes_to_u32(u)) as u16),
            5 => {
                let upper = four_to_u32(le_bytes_to_u32(&u[..4])) as u16;
                let lower = (u[4] - b'0') as u16;
                if let Some(res) = upper.wrapping_mul(10).checked_add(lower) {
                    Ok(res)
                } else {
                    Err(CheckError::AdditionOverflow(AdditionOverflow))
                }
            },
            _ => Err(CheckError::OverFlow(OverFlow)),
        }
    }

    pub fn to_u16_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<u16, CheckError> {
        let u: &[u8] = input.as_ref();
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty(Empty));
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 5, "to_u32 works only for numbers up to 5 bytes");

        match u.len() {
            0 => {
                // We checked empty above, so this means all zeros
                Ok(0)
            }
            1 => {
                if u[0] >= b'0' && u[0] <= b'9' {
                    Ok((u[0] - b'0') as u16)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            2 => {
                let chunk = le_bytes_to_u16(u);
                if check_decimal_bit_u16(chunk) {
                    Ok(two_to_u16_decimal(chunk))
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            3 => {
                let upper = le_bytes_to_u16(&u[..2]);
                if !(check_decimal_bit_u16(upper) && check_decimal_bit_u8(u[2])) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = two_to_u16_decimal(upper);
                    let lower = (u[2] - b'0') as u16;
                    Ok(upper * 10 + lower)
                }
            },
            4 => {
                let chunk = le_bytes_to_u32(u);
                if check_decimal_bit_u32(chunk) {
                    Ok(four_to_u32(chunk) as u16)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            5 => {
                let upper = le_bytes_to_u32(&u[..4]);
                if !(check_decimal_bit_u32(upper) && check_decimal_bit_u8(u[4])) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = four_to_u32(upper) as u16;
                    let lower = (u[4] - b'0') as u16;
                    if let Some(res) = (upper * 10).checked_add(lower) {
                        Ok(res)
                    } else {
                        Err(CheckError::AdditionOverflow(AdditionOverflow))
                    }
                }
            },
            _ => Err(CheckError::OverFlow(OverFlow)),
        }
    }

    pub fn to_u8_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<u8, CheckError> {
        let u: &[u8] = input.as_ref();
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty(Empty));
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        assert!(length <= 3, "to_u32 works only for numbers up to 3 bytes");

        match u.len() {
            0 => {
                // We checked empty above, so this means all zeros
                Ok(0)
            }
            1 => {
                if u[0] >= b'0' && u[0] <= b'9' {
                    Ok((u[0] - b'0') as u8)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            2 => {
                let chunk = le_bytes_to_u16(u);
                if check_decimal_bit_u16(chunk) {
                    Ok(two_to_u16_decimal(chunk) as u8)
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            3 => {
                let upper = le_bytes_to_u16(&u[..2]);
                if !(check_decimal_bit_u16(upper) && check_decimal_bit_u8(u[2])) {
                    Err(CheckError::NonDecimal(NonDecimal))
                } else {
                    let upper = two_to_u16_decimal(upper) as u8;
                    let lower = (u[2] - b'0') as u8;
                    Ok(upper * 10 + lower)
                }
            },
            _ => Err(CheckError::OverFlow(OverFlow)),
        }
    }
}

