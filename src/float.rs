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
    #[must_use]
    pub fn float_to_i128<T: AsRef<[u8]>>(&self, input: T) -> i128 {
        if self.is_negative(&input) {
            (!self.nonnegative_float_to_u128(&input.as_ref()[1..]) as i128).wrapping_add(1)
        } else {
            self.nonnegative_float_to_u128(&input) as i128
        }
    }

    pub fn nonnegative_float_to_u128<T: AsRef<[u8]>>(&self, input: T) -> u128 {
        let u = input.as_ref();
        let length = u.len();
        assert!(length <= 32, "The length of the input is too long. The maximum length is 32.");
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => Some(fraction_length),
            None => memchr(b'.', u),
        };

        let fraction_length = match fraction_length {
            Some(fraction_length) => length - fraction_length,
            None => 0,
        };
     
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456789012345678901"),  1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456789012345678901."), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678901234567890.1"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567890123456789.01"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456789012345678.901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678901234567.8901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567890123456.78901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456789012345.678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678901234.5678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567890123.45678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456789012.345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678901.2345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567890.12345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456789.012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678.9012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567.89012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456.789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345.6789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234.56789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123.456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012.3456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901.23456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890.123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789.0123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678.90123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567.890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456.7890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345.67890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234.567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123.4567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12.34567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1.234567890123456789012345678901"), 1234567890123456789012345678901);
        assert_eq!(flash_parser.nonnegative_float_to_u128(".1234567890123456789012345678901"), 1234567890123456789012345678901);
        //
        assert_eq!(flash_parser.nonnegative_float_to_u128("0.123456789012345678901234567890"), 123456789012345678901234567890);
    }

    #[test]
    fn test_15digit() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345"),  123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345."), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234.5"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123.45"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012.345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901.2345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890.12345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789.012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678.9012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567.89012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456.789012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345.6789012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234.56789012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123.456789012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12.3456789012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1.23456789012345"), 123456789012345);
        assert_eq!(flash_parser.nonnegative_float_to_u128(".123456789012345"), 123456789012345);
        //
        assert_eq!(flash_parser.nonnegative_float_to_u128("0.123456789012345"), 123456789012345);
    }

    #[test]
    fn test_16digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456"),  1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456."), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345.6"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234.56"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123.456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012.3456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901.23456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890.123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789.0123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678.90123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567.890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456.7890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345.67890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234.567890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123.4567890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12.34567890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1.234567890123456"), 1234567890123456);
        assert_eq!(flash_parser.nonnegative_float_to_u128(".1234567890123456"), 1234567890123456);
        //
        assert_eq!(flash_parser.nonnegative_float_to_u128("0.1234567890123456"), 1234567890123456);
    }

    #[test]
    fn test_17digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567"),  12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567."), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456.7"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345.67"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234.567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123.4567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012.34567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901.234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890.1234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789.01234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678.901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567.8901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456.78901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345.678901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234.5678901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123.45678901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12.345678901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1.2345678901234567"), 12345678901234567);
        assert_eq!(flash_parser.nonnegative_float_to_u128(".12345678901234567"), 12345678901234567);    
        //
        assert_eq!(flash_parser.nonnegative_float_to_u128("0.12345678901234567"), 12345678901234567);
    }

    #[test]
    fn test_18digits() {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678"),  123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345678."), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234567.8"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123456.78"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012345.678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901234.5678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890123.45678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789012.345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678901.2345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567890.12345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456789.012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345678.9012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234567.89012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123456.789012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12345.6789012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1234.56789012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("123.456789012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("12.3456789012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128("1.23456789012345678"), 123456789012345678);
        assert_eq!(flash_parser.nonnegative_float_to_u128(".123456789012345678"), 123456789012345678);
        //
        assert_eq!(flash_parser.nonnegative_float_to_u128("0.123456789012345678"), 123456789012345678);
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
}