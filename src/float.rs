use crate::BiscuitParser;
use crate::little_endian::{
    whole_chunk_to_u128,
    four_to_u32,
    eight_to_u64,
    sixteen_to_u128,
};
use memchr::memchr;
use std::ptr::read_unaligned;

fn get_divisor_f64(exponent: usize) -> f64 {
    if exponent == 0 {
        1.0
    } else if exponent == 1 {
        0.1
    } else if exponent == 2 {
        0.01
    } else if exponent == 3 {
        0.001
    } else if exponent == 4 {
        0.0_001
    } else if exponent == 5 {
        0.00_001
    } else if exponent == 6 {
        0.000_001
    } else if exponent == 7 {
        0.0_000_001
    } else if exponent == 8 {
        0.00_000_001
    } else if exponent == 9 {
        0.000_000_001
    } else if exponent == 10 {
        0.0_000_000_001
    } else if exponent == 11 {
        0.00_000_000_001
    } else if exponent == 12 {
        0.000_000_000_001
    } else if exponent == 13 {
        0.0_000_000_000_001
    } else if exponent == 14 {
        0.00_000_000_000_001
    } else if exponent == 15 {
        0.000_000_000_000_001
    } else if exponent == 16 {
        0.0_000_000_000_000_001
    } else if exponent == 17 {
        0.00_000_000_000_000_001
    } else if exponent == 18 {
        0.000_000_000_000_000_001
    } else {
        unreachable!("get_divisor_f64 works only for exponents up to 18")
    }
}

fn get_divisor_f32(exponent: usize) -> f32 {
    if exponent == 0 {
        1.0_f32
    } else if exponent == 1 {
        0.1_f32
    } else if exponent == 2 {
        0.01_f32
    } else if exponent == 3 {
        0.001_f32
    } else if exponent == 4 {
        0.0001_f32
    } else if exponent == 5 {
        0.00001_f32
    } else if exponent == 6 {
        0.000001_f32
    } else if exponent == 7 {
        0.0000001_f32
    } else {
        panic!("Exponent out of range");
    }
}

impl BiscuitParser {
    #[inline]
    #[must_use]
    pub fn to_f64<T: AsRef<[u8]>>(&self, general_input: T) -> f64 {
        let input = general_input.as_ref();
        let length = input.len();
        let fraction_length = if let Some(fraction_length) = self.fraction_length {
            fraction_length + 1
        } else {
            length - memchr(b'.', input).unwrap_or(length)
        };
        let exponent = if fraction_length <= 1 {0} else {fraction_length - 1};
        if length <= 4 {
            self.float_to_i32(input, length, fraction_length) as f64 * get_divisor_f64(exponent)
        } else if length <= 8 {
            self.float_to_i64(input, length, fraction_length) as f64 * get_divisor_f64(exponent)
        } else if length <= 39 {
            self.float_to_i128(input, length, fraction_length) as f64 * get_divisor_f64(exponent)
        } else {
            unreachable!("to_f64 works only for numbers up to 39 bytes")
        }
    }

    #[inline]
    #[must_use]
    pub fn to_f32<T: AsRef<[u8]>>(&self, general_input: T) -> f32 {
        let input = general_input.as_ref();
        let length = input.len();
        let fraction_length = if let Some(fraction_length) = self.fraction_length {
            fraction_length + 1
        } else {
            length - memchr(b'.', input).unwrap_or(length)
        };

        let exponent = if fraction_length <= 1 {0} else {fraction_length - 1};
        if length <= 4 {
            self.float_to_i32(input, length, fraction_length) as f32 * get_divisor_f32(exponent)
        } else if length <= 8 {
            self.float_to_i64(input, length, fraction_length) as i32 as f32 * get_divisor_f32(exponent)
        } else if length <= 39 {
            self.float_to_i128(input, length, fraction_length) as f32 * get_divisor_f32(exponent)
        } else {
            unreachable!("to_f32 works only for numbers up to 39 bytes")
        }
    }

    #[inline]
    pub fn float_to_u128_core(&self, input: &[u8], length: usize, fraction_length: usize) -> u128 {
        match fraction_length {
            0 => {
                if length <= 4 {
                    self.to_u16(input) as u128
                } else if length <= 8 {
                    self.to_u32(input) as u128
                } else if length <= 16 {
                    self.to_u64(input) as u128
                } else {
                    self.to_u128(input)
                }
            },
            1 => {
                if length <= 5 {
                    self.to_u16(&input[..(length - 1)]) as u128
                } else if length <= 9 {
                    self.to_u32(&input[..(length - 1)]) as u128
                } else if length <= 17 {
                    self.to_u64(&input[..(length - 1)]) as u128
                } else {
                    self.to_u128(&input[..(length - 1)])
                }
            },
            _ => {
                if length <= 4 {
                    let mut chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                    chunk <<= 32 - (length * 8);
                    let point_mask = 0xffff_ffff << ((5 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    four_to_u32(chunk, 4) as u128
                } else if length <= 8 {
                    let mut chunk: u64 = unsafe { read_unaligned(input.as_ptr() as *const u64) };
                    chunk <<= 64 - (length * 8);
                    let point_mask = 0xffff_ffff_ffff_ffff << ((9 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    eight_to_u64(chunk, 8) as u128
                } else if length <= 16 {
                    let mut chunk: u128 = unsafe { read_unaligned(input.as_ptr() as *const u128) };
                    chunk <<= 128 - (length * 8);  
                    let point_mask = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << ((17 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    sixteen_to_u128(chunk, 16)
                } else {
                    let point_location = length - fraction_length;
                    let left_point_length = if point_location > 16 { 0 } else { 16 - point_location };
                    let right_point_length = if point_location < 16 { 0 } else { fraction_length };
                    //
                    let (upper, lower) = input.split_at(16);
                    let upper_length = upper.len();
                    let lower_length = lower.len();

                    let mut upper = unsafe { read_unaligned(upper.as_ptr() as *const u128) };
                    let mut lower = unsafe { read_unaligned(lower.as_ptr() as *const u128) };

                    upper <<= 128 - (upper_length * 8);
                    lower <<= 128 - (lower_length * 8);

                    let upper_val = if left_point_length == 0 {
                        whole_chunk_to_u128(upper)
                    } else {
                        let upper_shift = (17 - left_point_length) * 8;
                        let upper_point_mask = if upper_shift >= 128 { 
                            0x0000_0000_0000_0000_0000_0000_0000_0000 
                        } else {
                            0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << upper_shift
                        };
                        let upper_decimal_mask = !upper_point_mask;
                        upper = (upper & upper_point_mask) + ((upper & (upper_decimal_mask >> 8)) << 8);
                        whole_chunk_to_u128(upper)
                    };

                    let lower_val = if right_point_length == 0 {
                        whole_chunk_to_u128(lower)
                    } else {
                        let lower_shift = (17 - right_point_length) * 8;
                        let lower_point_mask = if lower_shift >= 128 {
                            0x0000_0000_0000_0000_0000_0000_0000_0000
                        } else {
                            0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << lower_shift
                        };
                        let lower_decimal_mask = !lower_point_mask;
                        
                        lower = (lower & lower_point_mask) + ((lower & (lower_decimal_mask >> 8)) << 8);
                        whole_chunk_to_u128(lower)
                    };

                    let power_val = if point_location <= 15 {
                        10u128.pow((length - 16) as u32)
                    } else {
                        10u128.pow((length - 17) as u32)
                    };
                    
                    upper_val * power_val + lower_val
                }
            },
        }
    }

    #[inline]
    #[must_use]
    pub fn float_to_u64_core(&self, input: &[u8], length: usize, fraction_length: usize) -> u64 {
        match fraction_length {
            0 => {
                if length <= 4 {
                    self.to_u16(input) as u64
                } else if length <= 8 {
                    self.to_u32(input) as u64
                } else if length <= 16 {
                    self.to_u64(input)
                } else {
                    self.to_u128(input) as u64
                }                
            },
            1 => {
                if length <= 5 {
                    self.to_u16(&input[..(length - 1)]) as u64
                } else if length <= 9 {
                    self.to_u32(&input[..(length - 1)]) as u64
                } else if length <= 17 {
                    self.to_u64(&input[..(length - 1)])
                } else {
                    self.to_u128(&input[..(length - 1)]) as u64
                }
            },
            _ => {
                if length <= 4 {
                    let mut chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                    chunk <<= 32 - (length * 8);
                    
                    let point_mask = 0xffff_ffff << ((5 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    four_to_u32(chunk, 4) as u64
                } else if length <= 8 {
                    let mut chunk: u64 = unsafe { read_unaligned(input.as_ptr() as *const u64) };
                    chunk <<= 64 - (length * 8);  
                    let point_mask = 0xffff_ffff_ffff_ffff << ((9 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    eight_to_u64(chunk, 8)
                } else {
                    let mut chunk: u128 = unsafe { read_unaligned(input.as_ptr() as *const u128) };
                    chunk <<= 128 - (length * 8);  
                    let point_mask = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << ((17 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    sixteen_to_u128(chunk, 16) as u64
                }
            },
        }
    }

    #[inline]
    #[must_use]
    pub fn float_to_u32_core(&self, input: &[u8], length: usize, fraction_length: usize) -> u32 {
        match fraction_length {
            0 => {
                if length <= 4 {
                    self.to_u16(input) as u32
                } else if length <= 8 {
                    self.to_u32(input)
                } else if length <= 10 {
                    self.to_u64(input) as u32
                } else {
                    unreachable!("float_to_u32_core works only for numbers up to 10 bytes")
                }
            },
            1 => {
                if length <= 5 {
                    self.to_u16(&input[..(length - 1)]) as u32
                } else if length <= 9 {
                    self.to_u32(&input[..(length - 1)])
                } else if length <= 11 {
                    self.to_u64(&input[..(length - 1)]) as u32
                } else {
                    unreachable!("float_to_u32_core (with point) works only for numbers up to 11 bytes")
                }
            },
            _ => {
                if length <= 4 {
                    let mut chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                    chunk <<= 32 - (length * 8);
                    let point_mask = 0xffff_ffff << ((5 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    four_to_u32(chunk, length)
                } else {
                    let mut chunk: u64 = unsafe { read_unaligned(input.as_ptr() as *const u64) };
                    chunk <<= 64 - (length * 8);
                    let point_mask = 0xffff_ffff_ffff_ffff << ((9 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    eight_to_u64(chunk, length) as u32
                } 
            },
        }
    }
    #[inline]
    #[must_use]
    pub fn float_to_u16_core(&self, input: &[u8], length: usize, fraction_length: usize) -> u16 {
        match fraction_length {
            0 => {
                if length <= 2 {
                    self.to_u16(input)
                } else {
                    self.to_u32(input) as u16
                }
            },
            1 => {
                if length <= 3 {
                    self.to_u16(&input[..(length - 1)])
                } else {
                    self.to_u32(&input[..(length - 1)]) as u16
                }
            },
            _ => {
                let mut chunk: u32 = unsafe { read_unaligned(input.as_ptr() as *const u32) };
                chunk <<= 32 - (length * 8);
                let point_mask = 0xffff_ffff << ((5 - fraction_length) * 8);
                let decimal_mask = !point_mask;
                chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                four_to_u32(chunk, length) as u16
            },
        }
    }
                
    #[inline]
    #[must_use]
    pub fn float_to_i128(&self, input: &[u8], length: usize, fraction_length: usize) -> i128 {
        if input[0] == b'-' {   
            (!self.float_to_u128_core(&input[1..], length - 1, fraction_length) as i128).wrapping_add(1)
        } else {
            self.float_to_u128_core(&input, length, fraction_length) as i128
        }
    }

    #[inline]
    #[must_use]
    pub fn float_to_i64(&self, input: &[u8], length: usize, fraction_length: usize) -> i64 {
        if input[0] == b'-' {
            (!self.float_to_u64_core(&input.as_ref()[1..], length - 1, fraction_length) as i64).wrapping_add(1)
        } else {
            self.float_to_u64_core(&input, length, fraction_length) as i64
        }
    }

    #[inline]
    #[must_use]
    pub fn float_to_i32(&self, input: &[u8], length: usize, fraction_length: usize) -> i32 {
        if input[0] == b'-' {
            (!self.float_to_u32_core(&input.as_ref()[1..], length - 1, fraction_length) as i32).wrapping_add(1)
        } else {
            self.float_to_u32_core(&input, length, fraction_length) as i32
        }
    }

    #[inline]
    #[must_use]
    pub fn float_to_i16(&self, input: &[u8], length: usize, fraction_length: usize) -> i16 {
        if input[0] == b'-' {
            (!self.float_to_u16_core(&input.as_ref()[1..], length - 1, fraction_length - 1) as i16).wrapping_add(1)
        } else {
            self.float_to_u16_core(&input, length, fraction_length) as i16
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq::assert_approx_eq;

    #[test]
    fn test_f32_4digits() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("123.") as f64, 123.0, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32("12.3") as f64, 12.3, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32("1.23") as f64, 1.23, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32(".123") as f64, 0.123, 0.0001);
    }

    #[test]
    fn test_f32_5digits() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("1234.") as f64, 1234.0, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32("123.4") as f64, 123.4, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32("12.34") as f64, 12.34, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32("1.234") as f64, 1.234, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f32(".1234") as f64, 0.1234, 0.0001);
    }

    #[test]
    fn test_f64_4digits() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("123.") as f64, 123.0, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64("12.3") as f64, 12.3, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64("1.23") as f64, 1.23, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64(".123") as f64, 0.123, 0.0001);
    }

    #[test]
    fn test_f64_5digits() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("1234.") as f64, 1234.0, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64("123.4") as f64, 123.4, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64("12.34") as f64, 12.34, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64("1.234") as f64, 1.234, 0.0001);
        assert_approx_eq!(biscuit_parser.to_f64(".1234") as f64, 0.1234, 0.0001);
    }

}