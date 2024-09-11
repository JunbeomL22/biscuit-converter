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

