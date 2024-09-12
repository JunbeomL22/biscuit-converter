use crate::BiscuitConverter;
use crate::error::CheckError;

use crate::little_endian_decimal::{
    checked_conversion_u8,
    checked_conversion_u16,
    checked_conversion_u32,
    checked_conversion_u64,
    checked_conversion_u128,
};

// to handle integer converter overflow
const I128_MAX_AS_U128: u128 = 170141183460469231731687303715884105727;
const I128_MIN_ABS_AS_U128: u128 = 170141183460469231731687303715884105728;
const I64_MAX_AS_U64: u64 = 9223372036854775807;
const I64_MIN_ABS_AS_U64: u64 = 9223372036854775808;
const I32_MAX_AS_U32: u32 = 2147483647;
const I32_MIN_ABS_AS_U32: u32 = 2147483648;
const I16_MAX_AS_U16: u16 = 32767;
const I16_MIN_ABS_AS_U16: u16 = 32768;
const I8_MAX_AS_U8: u8 = 127;
const I8_MIN_ABS_AS_U8: u8 = 128;

impl BiscuitConverter {
    #[inline]
    pub fn to_u128_decimal(self, u: &[u8]) -> Result<u128, CheckError> {
        self.to_u128_decimal_core(u, false, false)
    }

    pub fn to_u128_decimal_core(self, u: &[u8], neg_max_check: bool, pos_max_check: bool) -> Result<u128, CheckError> {
        let mut length = u.len();
        if length == 0 {
            return Err(CheckError::Empty)
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        length -= start;

        match length {
            0 => Ok(0),
            1 => checked_conversion_u8(u).map(|val| val as u128),
            2 => checked_conversion_u16(u).map(|val| val as u128),
            3 => {
                let upper_chunk = checked_conversion_u16(&u[..2])? as u128;
                let lower_chunk = checked_conversion_u8(&u[2..])? as u128;
                Ok(upper_chunk * 10 + lower_chunk)
            }
            4 => checked_conversion_u32(u).map(|val| val as u128),
            5 => {
                let upper_chunk = checked_conversion_u32(&u[..4])? as u128;
                let lower_chunk = checked_conversion_u8(&u[4..])? as u128;
                Ok(upper_chunk * 10 + lower_chunk)
            },
            6 => {
                let upper_chunk = checked_conversion_u32(&u[..4])? as u128;
                let lower_chunk = checked_conversion_u16(&u[4..])? as u128;
                Ok(upper_chunk * 100 + lower_chunk)
            },
            7 => {
                let upper_chunk = checked_conversion_u32(&u[..4])? as u128;
                let mid_chunk = checked_conversion_u16(&u[4..6])? as u128;
                let lower_chunk = checked_conversion_u8(&u[6..])? as u128;
                Ok(upper_chunk * 1_000 + mid_chunk * 10 + lower_chunk)
            },
            8 => checked_conversion_u64(u).map(|val| val as u128),
            9 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let lower_chunk = checked_conversion_u8(&u[8..])? as u128;
                Ok(upper_chunk * 10 + lower_chunk)
            },
            10 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let lower_chunk = checked_conversion_u16(&u[8..])? as u128;
                Ok(upper_chunk * 100 + lower_chunk)
            },
            11 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let mid_chunk = checked_conversion_u16(&u[8..10])? as u128;
                let lower_chunk = checked_conversion_u8(&u[10..])? as u128;
                Ok(upper_chunk * 1_000 + mid_chunk * 10 + lower_chunk)
            },
            12 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let lower_chunk = checked_conversion_u32(&u[8..])? as u128;
                Ok(upper_chunk * 10_000 + lower_chunk)
            },
            13 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let mid_chunk = checked_conversion_u32(&u[8..12])? as u128;
                let lower_chunk = checked_conversion_u8(&u[12..])? as u128;
                Ok(upper_chunk * 100_000 + mid_chunk * 10 + lower_chunk)
            },
            14 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let mid_chunk = checked_conversion_u32(&u[8..12])? as u128;
                let lower_chunk = checked_conversion_u16(&u[12..])? as u128;
                Ok(upper_chunk * 1_000_000 + mid_chunk * 100 + lower_chunk)
            },
            15 => {
                let upper_chunk = checked_conversion_u64(&u[..8])? as u128;
                let mid_chunk = checked_conversion_u32(&u[8..12])? as u128;
                let lower_chunk = checked_conversion_u16(&u[12..14])? as u128;
                let last_chunk = checked_conversion_u8(&u[14..])? as u128;
                let upper = upper_chunk * 10_000_000;
                let mid = mid_chunk * 1_000;
                let lower = lower_chunk * 10;
                Ok(upper + mid + lower + last_chunk)
            },
            16 => checked_conversion_u128(u),
            17 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u8(&u[16..])? as u128;
                Ok(upper_chunk * 10 + lower_chunk)
            },
            18 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u16(&u[16..])? as u128;
                Ok(upper_chunk * 100 + lower_chunk)
            },
            19 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u16(&u[16..18])? as u128;
                let lower_chunk = checked_conversion_u8(&u[18..])? as u128;
                let upper = upper_chunk * 1_000;
                let mid = mid_chunk * 10;
                let lower = lower_chunk;
                Ok(upper + mid + lower)
            },
            20 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u32(&u[16..])? as u128;
                let upper = upper_chunk * 10_000;
                Ok(upper + lower_chunk)
            },
            21 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u32(&u[16..20])? as u128;
                let lower_chunk = checked_conversion_u8(&u[20..])? as u128;
                let upper = upper_chunk * 100_000;
                let mid = mid_chunk * 10;
                let lower = lower_chunk;
                Ok(upper + mid + lower)
            },
            22 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u32(&u[16..20])? as u128;
                let lower_chunk = checked_conversion_u16(&u[20..])? as u128;
                let upper = upper_chunk * 1_000_000;
                let mid = mid_chunk * 100;
                let lower = lower_chunk;
                Ok(upper + mid + lower)
            },
            23 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u32(&u[16..20])? as u128;
                let lower_chunk = checked_conversion_u16(&u[20..22])? as u128;
                let last_chunk = checked_conversion_u8(&u[22..])? as u128;
                let upper = upper_chunk * 10_000_000;
                let mid = mid_chunk * 1_000;
                let lower = lower_chunk * 10;
                Ok(upper + mid + lower + last_chunk)
            },
            24 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let upper = upper_chunk * 100_000_000;
                Ok(upper + lower_chunk)
            },
            25 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u8(&u[24..])? as u128;
                let upper = upper_chunk * 1_000_000_000;
                let mid = mid_chunk * 10;
                Ok(upper + mid + lower_chunk)
            },
            26 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u16(&u[24..])? as u128;
                let upper = upper_chunk * 10_000_000_000;
                let mid = mid_chunk * 100;
                Ok(upper + mid + lower_chunk)
            },
            27 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u16(&u[24..26])? as u128;
                let last_chunk = checked_conversion_u8(&u[26..])? as u128;
                let upper = upper_chunk * 100_000_000_000;
                let mid = mid_chunk * 1_000;
                let lower = lower_chunk * 10;
                Ok(upper + mid + lower + last_chunk)
            },
            28 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u32(&u[24..])? as u128;
                let upper = upper_chunk * 1_000_000_000_000;
                let mid = mid_chunk * 10_000;
                Ok(upper + mid + lower_chunk)
            },
            29 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u32(&u[24..28])? as u128;
                let last_chunk = checked_conversion_u8(&u[28..])? as u128;
                let upper = upper_chunk * 10_000_000_000_000;
                let mid = mid_chunk * 100_000;
                let lower = lower_chunk * 10;
                Ok(upper + mid + lower + last_chunk)
            },
            30 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u32(&u[24..28])? as u128;
                let last_chunk = checked_conversion_u16(&u[28..30])? as u128;
                let upper = upper_chunk * 100_000_000_000_000;
                let mid = mid_chunk * 1_000_000;
                let lower = lower_chunk * 100;
                Ok(upper + mid + lower + last_chunk)
            },
            31 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u64(&u[16..24])? as u128;
                let lower_chunk = checked_conversion_u32(&u[24..28])? as u128;
                let tail_chunk = checked_conversion_u16(&u[28..30])? as u128;
                let last = checked_conversion_u8(&u[30..])? as u128;
                let upper = upper_chunk * 1_000_000_000_000_000;
                let mid = mid_chunk * 10_000_000;
                let lower = lower_chunk * 1_000;
                let tail = tail_chunk * 10;
                
                Ok(upper + mid + lower + tail + last)
            },
            32 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u128(&u[16..])? as u128;
                let upper = upper_chunk * 10_000_000_000_000_000;
                Ok(upper + lower_chunk)
            },
            33 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u128(&u[16..32])?;
                let lower_chunk = checked_conversion_u8(&u[32..])? as u128;
                let upper = upper_chunk.wrapping_mul(100_000_000_000_000_000);
                let mid = mid_chunk * 10;
                Ok(mid + lower_chunk + upper)
            },
            34 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u128(&u[16..32])?;
                let lower_chunk = checked_conversion_u16(&u[32..])? as u128;
                let upper = upper_chunk.wrapping_mul(1_000_000_000_000_000_000);
                let mid = mid_chunk * 100;
                let res = mid + lower_chunk;
                if let Some(res) = upper.checked_add(res) {
                    Ok(res)
                } else {
                    Err(CheckError::Overflow)
                }
            },
            35 => {
                let upper_chunk = checked_conversion_u128(&u[..16])? as u128;
                let mid_chunk = checked_conversion_u128(&u[16..32])? as u128;
                let lower_chunk = checked_conversion_u16(&u[32..34])? as u128;
                let last_chunk = checked_conversion_u8(&u[34..])? as u128;
                let upper = upper_chunk.wrapping_mul(10_000_000_000_000_000_000);
                let mid = mid_chunk * 1_000;
                let lower = lower_chunk * 10;
                Ok(upper + mid + lower + last_chunk)
            },
            36 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u128(&u[16..32])?;
                let lower_chunk = checked_conversion_u32(&u[32..])? as u128;
                let upper = upper_chunk.wrapping_mul(100_000_000_000_000_000_000);
                let mid = mid_chunk * 10_000;
                let res = mid + lower_chunk;
                if let Some(res) = upper.checked_add(res) {
                    Ok(res)
                } else {
                    Err(CheckError::Overflow)
                }
            },
            37 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u128(&u[16..32])?;
                let lower_chunk = checked_conversion_u32(&u[32..36])? as u128;
                let last_chunk = checked_conversion_u8(&u[36..])? as u128;
                let upper = upper_chunk.wrapping_mul(1_000_000_000_000_000_000_000);
                let mid = mid_chunk * 100_000;
                let lower = lower_chunk * 10;
                Ok(upper + mid + lower + last_chunk)
            },
            38 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u128(&u[16..32])?;
                let lower_chunk = checked_conversion_u32(&u[32..36])? as u128;
                let last_chunk = checked_conversion_u16(&u[36..])? as u128;
                let upper = upper_chunk.wrapping_mul(10_000_000_000_000_000_000_000);
                let mid = mid_chunk * 1_000_000;
                let lower = lower_chunk * 100;
                let res = mid + lower + last_chunk;
                if let Some(res) = upper.checked_add(res) {
                    Ok(res)
                } else {
                    Err(CheckError::Overflow)
                }
            },
            39 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u128(&u[16..32])?;
                let lower_chunk = checked_conversion_u32(&u[32..36])? as u128;
                let tail_chunk = checked_conversion_u16(&u[36..38])? as u128;
                let last = checked_conversion_u8(&u[38..])? as u128;
                let upper = upper_chunk.wrapping_mul(100_000_000_000_000_000_000_000);
                let mid = mid_chunk * 10_000_000;
                let lower = lower_chunk * 1000;
                let tail = tail_chunk * 10;
                let res = mid + lower + tail + last;
                if let Some(res) = upper.checked_add(res) {
                    if neg_max_check && res > I128_MIN_ABS_AS_U128 {
                        Err(CheckError::Overflow)
                    } else if pos_max_check && res > I128_MAX_AS_U128 {
                        Err(CheckError::Overflow)
                    } else {
                        Ok(res)
                    }
                } else {
                    Err(CheckError::Overflow)
                }
            },
            _ => Err(CheckError::Overflow)
        }        
 
    }

    #[inline]
    pub fn to_u64_decimal(self, u: &[u8]) -> Result<u64, CheckError> {
        self.to_u64_decimal_core(u, false, false)
    }

    pub fn to_u64_decimal_core(self, u: &[u8], neg_max_check: bool, pos_max_check: bool) -> Result<u64, CheckError> {
        let mut length = u.len();
        if length == 0 {
            return Err(CheckError::Empty)
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        length -= start;

        match length {
            0 => Ok(0),
            1 => checked_conversion_u8(u).map(|val| val as u64),
            2 => checked_conversion_u16(u).map(|val| val as u64),
            3 => {
                let upper_chunk = checked_conversion_u16(&u[..2])?;
                let lower_chunk = checked_conversion_u8(&u[2..])?;
                Ok((upper_chunk as u64) * 10 + lower_chunk as u64)
            }
            4 => checked_conversion_u32(u).map(|val| val as u64),
            5 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let lower_chunk = checked_conversion_u8(&u[4..])?;
                Ok((upper_chunk) as u64 * 10 + lower_chunk as u64)
            },
            6 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let lower_chunk = checked_conversion_u16(&u[4..])?;
                Ok(upper_chunk as u64 * 100 + lower_chunk as u64)
            },
            7 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let mid_chunk = checked_conversion_u16(&u[4..6])?;
                let lower_chunk = checked_conversion_u8(&u[6..])?;
                Ok(upper_chunk as u64 * 1_000 + mid_chunk as u64 * 10 + lower_chunk as u64)
            },
            8 => checked_conversion_u64(u),
            9 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let lower_chunk = checked_conversion_u8(&u[8..])?;
                Ok(upper_chunk as u64 * 10 + lower_chunk as u64)
            },
            10 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let lower_chunk = checked_conversion_u16(&u[8..])?;
                Ok(upper_chunk as u64 * 100 + lower_chunk as u64)
            },
            11 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let mid_chunk = checked_conversion_u16(&u[8..10])?;
                let lower_chunk = checked_conversion_u8(&u[10..])?;
                Ok(upper_chunk as u64 * 1_000 + mid_chunk as u64 * 10 + lower_chunk as u64)
            },
            12 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let lower_chunk = checked_conversion_u32(&u[8..])?;
                Ok(upper_chunk as u64 * 10_000 + lower_chunk as u64)
            },
            13 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let mid_chunk = checked_conversion_u32(&u[8..12])?;
                let lower_chunk = checked_conversion_u8(&u[12..])?;
                Ok(upper_chunk as u64 * 100_000 + mid_chunk as u64 * 10 + lower_chunk as u64)
            },
            14 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let mid_chunk = checked_conversion_u32(&u[8..12])?;
                let lower_chunk = checked_conversion_u16(&u[12..])?;
                Ok(upper_chunk as u64 * 1_000_000 + mid_chunk as u64 * 100 + lower_chunk as u64)
            },
            15 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let mid_chunk = checked_conversion_u32(&u[8..12])?;
                let lower_chunk = checked_conversion_u16(&u[12..14])?;
                let last_chunk = checked_conversion_u8(&u[14..])?;
                
                let upper = (upper_chunk as u64).wrapping_mul(10_000_000);
                let mid = mid_chunk as u64 * 1_000;
                let lower = lower_chunk as u64 * 10;
                let last = last_chunk as u64;
                let res = last + lower + mid;
                if let Some(res) = upper.checked_add(res) {
                    Ok(res)
                } else {
                    Err(CheckError::Overflow)
                }

            },
            16 => checked_conversion_u128(u).map(|val| val as u64),
            17 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u8(&u[16..])?;
                Ok(upper_chunk as u64 * 10 + lower_chunk as u64)
            },
            18 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u16(&u[16..])?;
                Ok(upper_chunk as u64 * 100 + lower_chunk as u64)
            },
            19 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let mid_chunk = checked_conversion_u16(&u[16..18])?;
                let lower_chunk = checked_conversion_u8(&u[18..])?;
                let upper = upper_chunk as u64 * 1_000;
                let mid = mid_chunk as u64 * 10;
                let lower = lower_chunk as u64;
                let res = lower + mid;
                if let Some(res) = upper.checked_add(res) {
                    if neg_max_check && res > I64_MIN_ABS_AS_U64 {
                        Err(CheckError::Overflow)
                    } else if pos_max_check && res > I64_MAX_AS_U64 {
                        Err(CheckError::Overflow)
                    } else {
                        Ok(res)
                    }
                } else {
                    Err(CheckError::Overflow)
                }
            },
            20 => {
                let upper_chunk = checked_conversion_u128(&u[..16])?;
                let lower_chunk = checked_conversion_u32(&u[16..])?;
                let upper = (upper_chunk as u64).wrapping_mul(10_000);
                if let Some(res) = upper.checked_add(lower_chunk as u64) {
                    if neg_max_check && res > I64_MIN_ABS_AS_U64 {
                        Err(CheckError::Overflow)
                    } else if pos_max_check && res > I64_MAX_AS_U64 {
                        Err(CheckError::Overflow)
                    } else {
                        Ok(res)
                    }
                } else {
                    Err(CheckError::Overflow)
                }
            },
            _ => Err(CheckError::Overflow)
        }
    }

    #[inline]
    pub fn to_u32_decimal(self, u: &[u8]) -> Result<u32, CheckError> {
        self.to_u32_decimal_core(u, false, false)
    }

    pub fn to_u32_decimal_core(self, u: &[u8], neg_max_check: bool, pos_max_check: bool) -> Result<u32, CheckError> {
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty)
        }

        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;
        
        match length {
            0 => Ok(0),
            1 => checked_conversion_u8(u).map(|val| val as u32),
            2 => checked_conversion_u16(u).map(|val| val as u32),
            3 => {
                let upper_chunk = checked_conversion_u16(&u[..2])?;
                let lower_chunk = checked_conversion_u8(&u[2..])?;
                Ok((upper_chunk as u32) * 10 + lower_chunk as u32)
            }
            4 => checked_conversion_u32(u),
            5 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let lower_chunk = checked_conversion_u8(&u[4..])?;
                Ok((upper_chunk) as u32 * 10 + lower_chunk as u32)
            },
            6 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let lower_chunk = checked_conversion_u16(&u[4..])?;
                Ok(upper_chunk as u32 * 100 + lower_chunk as u32)
            },
            7 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let mid_chunk = checked_conversion_u16(&u[4..6])?;
                let lower_chunk = checked_conversion_u8(&u[6..])?;
                Ok(upper_chunk as u32 * 1_000 + mid_chunk as u32 * 10 + lower_chunk as u32)
            },
            8 => checked_conversion_u64(u).map(|val| val as u32),
            9 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let lower_chunk = checked_conversion_u8(&u[8..])?;
                Ok(upper_chunk as u32 * 10 + lower_chunk as u32)
            },
            10 => {
                let upper_chunk = checked_conversion_u64(&u[..8])?;
                let lower_chunk = checked_conversion_u16(&u[8..])?;
                let upper = (upper_chunk as u32).wrapping_mul(100);
                if let Some(res) = upper.checked_add(lower_chunk as u32) {
                    if neg_max_check && res > I32_MIN_ABS_AS_U32 {
                        Err(CheckError::Overflow)
                    } else if pos_max_check && res > I32_MAX_AS_U32 {
                        Err(CheckError::Overflow)
                    } else {
                        Ok(res)
                    }
                } else {
                    Err(CheckError::Overflow)
                }
            },
            _ => Err(CheckError::Overflow)
        }
    }
    
    #[inline]
    pub fn to_u16_decimal(self, u: &[u8]) -> Result<u16, CheckError> {
        self.to_u16_decimal_core(u, false, false)
    }

    pub fn to_u16_decimal_core(self, u: &[u8], neg_max_check: bool, pos_max_check: bool) -> Result<u16, CheckError> {
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty)
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;

        match length {
            0 => Ok(0),
            1 => checked_conversion_u8(u).map(|val| val as u16),
            2 => checked_conversion_u16(u),
            3 => {
                let upper_chunk = checked_conversion_u16(&u[..2])?;
                let lower_chunk = checked_conversion_u8(&u[2..])?;
                Ok((upper_chunk as u16) * 10 + lower_chunk as u16)
            }
            4 => checked_conversion_u32(u).map(|val| val as u16),
            5 => {
                let upper_chunk = checked_conversion_u32(&u[..4])?;
                let lower_chunk = checked_conversion_u8(&u[4..])?;
                let upper_chunk = (upper_chunk as u16).wrapping_mul(10);
                if let Some(res) = upper_chunk.checked_add(lower_chunk as u16) {
                    if neg_max_check && res > I16_MIN_ABS_AS_U16 {
                        Err(CheckError::Overflow)
                    } else if pos_max_check && res > I16_MAX_AS_U16 {
                        Err(CheckError::Overflow)
                    } else {
                        Ok(res)
                    }
                } else {
                    Err(CheckError::Overflow)
                }
            },
            _ => Err(CheckError::Overflow)
        }
    }

    #[inline]
    pub fn to_u8_decimal(self, u: &[u8]) -> Result<u8, CheckError> {
        self.to_u8_decimal_core(u, false, false)
    }

    pub fn to_u8_decimal_core(self, u: &[u8], neg_max_check: bool, pos_max_check: bool) -> Result<u8, CheckError> {
        let length = u.len();
        if length == 0 {
            return Err(CheckError::Empty)
        }
        let mut start = 0;
        while start < length && u[start] == b'0' {
            start += 1;
        }
        let u = &u[start..];
        let length = length - start;

        match length {
            0 => Ok(0),
            1 => checked_conversion_u8(u),
            2 => checked_conversion_u16(u).map(|val| val as u8),
            3 => {
                let upper_chunk = checked_conversion_u16(&u[..2])?;
                let lower_chunk = checked_conversion_u8(&u[2..])?;
                let upper_chunk = (upper_chunk as u8).wrapping_mul(10);
                if let Some(res) = upper_chunk.checked_add(lower_chunk as u8) {
                    if neg_max_check && res > I8_MIN_ABS_AS_U8 {
                        Err(CheckError::Overflow)
                    } else if pos_max_check && res > I8_MAX_AS_U8 {
                        Err(CheckError::Overflow)
                    } else {
                        Ok(res)
                    }
                } else {
                    Err(CheckError::Overflow)
                }
            }
            _ => Err(CheckError::Overflow),
        }
    }
}

