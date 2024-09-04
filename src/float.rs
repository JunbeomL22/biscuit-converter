use crate::FlashParser;
use crate::little_endian::whole_chunk_to_u128;
use memchr::memchr;
use std::ptr::read_unaligned;

impl FlashParser {
    #[inline(always)]
    pub fn is_negative<T: AsRef<[u8]>>(&self, input: T) -> bool {
        let u = input.as_ref();
        u[0] == b'-'
    }

    #[inline(always)]
    pub fn to_f64<T: AsRef<[u8]>>(&self, input: T) -> f64 {
        let u = input.as_ref();
        let length = u.len();
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', u),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };
        // 0 -> 0 // 1 -> 0 // 2 -> 2 // 3 -> 3 // 4 -> 4 // 5 -> 5 // 6 -> 6 // 7 -> 7 // 8 -> 8 // 9 -> 9
        let exponent = if fraction_length <= 1 {0} else {fraction_length};
        self.float_to_i128(u) as f64 / 10i128.pow(exponent as u32) as f64
    }

    #[inline(always)]
    #[must_use]
    pub fn float_to_i128<T: AsRef<[u8]>>(&self, input: T) -> i128 {
        let length = input.as_ref().len();
        assert!(length <= 32, "The length of the input is too long. The maximum length is 32.");
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', input.as_ref()),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };

        if self.is_negative(&input) {
            // Negate the unsigned integer and add 1 to convert to negative
            (!self.float_to_u128_core(&input.as_ref()[1..], length-1, fraction_length) as i128).wrapping_add(1)
            //(!self.float_to_u128_core(&input.as_ref()[1..], length, fraction_length) as i128).wrapping_add(1)
        } else {
            self.float_to_u128_core(&input, length, fraction_length) as i128
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn float_to_u128<T: AsRef<[u8]>>(&self, input: T) -> u128 {
        let length = input.as_ref().len();
        assert!(length <= 32, "The length of the input is too long. The maximum length is 32.");
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', input.as_ref()),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };

        self.float_to_u128_core(&input, length, fraction_length)
    }

    pub fn float_to_u128_core<T: AsRef<[u8]>>(&self, input: T, length: usize, fraction_length: usize) -> u128 {
        let u = input.as_ref();
        match fraction_length {
            0 => self.to_u128(u),
            1 => self.to_u128(&u[..(length - 1)]),
            _ => {
                if length <= 16 {
                    let mut chunk: u128 = unsafe { read_unaligned(u.as_ptr() as *const u128) };
                    chunk <<= 128 - (length * 8);  
                    let point_mask = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff << ((17 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    whole_chunk_to_u128(chunk)
                } else {
                    let point_location = length - fraction_length;
                    let left_point_length = if point_location > 16 { 0 } else { 16 - point_location };
                    let right_point_length = if point_location < 16 { 0 } else { fraction_length };
                    //
                    let (upper, lower) = u.split_at(16);
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
    pub fn float_to_i64<T: AsRef<[u8]>>(&self, input: T) -> i64 {
        let length = input.as_ref().len();
        assert!(length <= 19, "The length of the input is too long. The maximum length is 19.");
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', input.as_ref()),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };

        if self.is_negative(&input) {
            (!self.float_to_u64_core(&input.as_ref()[1..], length, fraction_length) as i64).wrapping_add(1)
        } else {
            self.float_to_u64_core(&input, length - 1, fraction_length) as i64
        }
    }

    #[inline]
    #[must_use]
    pub fn float_to_f32<T: AsRef<[u8]>>(&self, input: T) -> f32 {
        let u = input.as_ref();
        let length = u.len();
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', u),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };

        let exponent = if fraction_length <= 1 {0} else {fraction_length};
        self.to_i64(u) as f32 / 10u64.pow(exponent as u32) as f32
    }

    #[inline(always)]
    #[must_use]
    pub fn float_to_u64<T: AsRef<[u8]>>(&self, input: T) -> u64 {
        let length = input.as_ref().len();
        assert!(length <= 19, "The length of the input is too long. The maximum length is 19.");
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', input.as_ref()),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };

        self.float_to_u64_core(&input, length, fraction_length)
    }

    #[inline]
    pub fn float_to_u32_core<T: AsRef<[u8]>>(&self, input: T, length: usize, fraction_length: usize) -> u32 {
       unimplemented!()
    }

    #[inline]
    pub fn float_to_u32<T: AsRef<[u8]>>(&self, input: T) -> u32 {
        unimplemented!()
    }

    #[must_use]
    pub fn float_to_u64_core<T: AsRef<[u8]>>(&self, input: T, length: usize, fraction_length: usize) -> u64 {
        let u = input.as_ref();
        match fraction_length {
            0 => self.to_u64(u),
            1 => self.to_u64(&u[..(length - 1)]),
            _ => {
                if length <= 9 {
                    // ex) u = "123.45", length = 6, point_length = 3,
                    // "123.45" => "??123.45"
                    let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
                    // "??123.45" => "123.4500"
                    chunk <<= 64 - (length * 8);
                    // "123.4500" => "12345000"
                    let point_mask = 0xffff_ffff_ffff_ffff << ((9 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    chunk
                } else {
                    let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
                    chunk <<= 64 - (length * 8);  
                    let point_mask = 0xffff_ffff_ffff_ffff << ((17 - fraction_length) * 8);
                    let decimal_mask = !point_mask;
                    chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
                    chunk as u64
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.float_to_u128("1234567890123456789012345678901"),  1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123456789012345678901."), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012345678901234567890.1"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901234567890123456789.01"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123456789012345678.901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012345678901234567.8901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901234567890123456.78901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123456789012345.678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012345678901234.5678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901234567890123.45678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123456789012.345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012345678901.2345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901234567890.12345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123456789.012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012345678.9012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901234567.89012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123456.789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012345.6789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901234.56789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890123.456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789012.3456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678901.23456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567890.123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456789.0123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345678.90123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234567.890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123456.7890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12345.67890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1234.567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("123.4567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("12.34567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128("1.234567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.float_to_u128(".1234567890123456789012345678901"), 1234567890123456789012345678901);
        //
        assert_eq!(flash_parser.float_to_u128("0.123456789012345678901234567890"), 123456789012345678901234567890);
    }

    #[test]
    fn test_15digit() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.float_to_u128("123456789012345"),  123456789012345);
        assert_eq!(flash_parser.float_to_u128("123456789012345."), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("12345678901234.5"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("1234567890123.45"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("123456789012.345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("12345678901.2345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("1234567890.12345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("123456789.012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("12345678.9012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("1234567.89012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("123456.789012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("12345.6789012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("1234.56789012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("123.456789012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("12.3456789012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128("1.23456789012345"), 123456789012345);
        assert_eq!(flash_parser.float_to_u128(".123456789012345"), 123456789012345);
        //
        assert_eq!(flash_parser.float_to_u128("0.123456789012345"), 123456789012345);
    }

    #[test]
    fn test_16digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.float_to_u128("1234567890123456"),  1234567890123456);
        assert_eq!(flash_parser.float_to_u128("1234567890123456."), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("123456789012345.6"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("12345678901234.56"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("1234567890123.456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("123456789012.3456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("12345678901.23456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("1234567890.123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("123456789.0123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("12345678.90123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("1234567.890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("123456.7890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("12345.67890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("1234.567890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("123.4567890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("12.34567890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128("1.234567890123456"), 1234567890123456);
        assert_eq!(flash_parser.float_to_u128(".1234567890123456"), 1234567890123456);
        //
        assert_eq!(flash_parser.float_to_u128("0.1234567890123456"), 1234567890123456);
    }

    #[test]
    fn test_17digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.float_to_u128("12345678901234567"),  12345678901234567);
        assert_eq!(flash_parser.float_to_u128("12345678901234567."), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("1234567890123456.7"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("123456789012345.67"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("12345678901234.567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("1234567890123.4567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("123456789012.34567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("12345678901.234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("1234567890.1234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("123456789.01234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("12345678.901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("1234567.8901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("123456.78901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("12345.678901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("1234.5678901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("123.45678901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("12.345678901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128("1.2345678901234567"), 12345678901234567);
        assert_eq!(flash_parser.float_to_u128(".12345678901234567"), 12345678901234567);    
        //
        assert_eq!(flash_parser.float_to_u128("0.12345678901234567"), 12345678901234567);
    }

    #[test]
    fn test_18digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.float_to_u128("123456789012345678"),  123456789012345678);
        assert_eq!(flash_parser.float_to_u128("123456789012345678."), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("12345678901234567.8"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("1234567890123456.78"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("123456789012345.678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("12345678901234.5678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("1234567890123.45678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("123456789012.345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("12345678901.2345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("1234567890.12345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("123456789.012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("12345678.9012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("1234567.89012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("123456.789012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("12345.6789012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("1234.56789012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("123.456789012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("12.3456789012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128("1.23456789012345678"), 123456789012345678);
        assert_eq!(flash_parser.float_to_u128(".123456789012345678"), 123456789012345678);
        //
        assert_eq!(flash_parser.float_to_u128("0.123456789012345678"), 123456789012345678);
    }

    #[test]
    fn test_negative_17digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.float_to_i128("-12345678901234567"),  -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-12345678901234567."), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-1234567890123456.7"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-123456789012345.67"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-12345678901234.567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-1234567890123.4567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-123456789012.34567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-12345678901.234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-1234567890.1234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-123456789.01234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-12345678.901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-1234567.8901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-123456.78901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-12345.678901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-1234.5678901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-123.45678901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-12.345678901234567"), -12345678901234567);
        assert_eq!(flash_parser.float_to_i128("-1.2345678901234567"), -12345678901234567);
    }

    #[test]
    fn test_float() {
        let flash_parser = FlashParser::default();
        let val = flash_parser.to_f64("1234567.123");
        assert!(
            (val - 1234567.123).abs() < 0.0000001,
            "val: {}, expected: 1234567.123",
            val,
        );


    }
}