//! # biscuit-converter
//! 
//! `biscuit-converter` is a decimal integer parser using bitwise operations.
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! biscuit-converter = "0.3"
//! ```
//! 
//! Then, use it in your Rust code:
//! 
//! ```rust
//! use biscuit_converter::{Biscuit, error::ParseIntErr};
//! 
//! let val = i32::parse_decimal(b"1234");
//! assert_eq!(val, Ok(1234));
//!
//! let val = i32::parse_decimal(b"1234a");
//! assert_eq!(val, Err(ParseIntErr::NonDecimal));
//! 
//! let val = i32::parse_decimal(b"a1234");
//! assert_eq!(val, Err(ParseIntErr::NonDecimal));
//! 
//! let val = i32::parse_decimal(b"");
//! assert_eq!(val, Err(ParseIntErr::Empty));
//! 
//! let val = i32::parse_decimal(b" ");
//! assert_eq!(val, Err(ParseIntErr::NonDecimal));
//! 
//! let val = i32::parse_decimal(b"2147483647"); // i32::MAX
//! assert_eq!(val, Ok(2147483647));
//! 
//! let val = i32::parse_decimal(b"2147483648"); // i32::MAX + 1
//! assert_eq!(val, Err(ParseIntErr::Overflow));
//! 
//! let val = i32::parse_decimal(b"-2147483648"); // i32::MIN
//! assert_eq!(val, Ok(-2147483648));
//! 
//! let val = i32::parse_decimal(b"-2147483649"); // i32::MIN - 1
//! assert_eq!(val, Err(ParseIntErr::NegOverflow));
//! 
//! let reading = i32::parse_decimal(b"0000000000000000000000000000000000000123");
//! assert_eq!(reading, Ok(123));
//! ```
//! 
//! ## License
//! 
//! This project is licensed under either of
//! - Apache License, Version 2.0, <http://www.apache.org/licenses/LICENSE-2.0>
//! - MIT license <http://opensource.org/licenses/MIT>
//! at your option.
//! 
//! ## Contributions
//! 
//! Contributions are welcome! Feel free to open issues or submit pull requests on our GitHub repository.

// Rest of your library code goes here...
pub mod unsigned_decimal;
pub mod little_endian_decimal;
pub mod integer_decimal;
pub mod error;
pub mod utils;
/// Parser for decimal notation
/// It can not parse scientific notation
pub trait Biscuit: Sized {
    #[inline]
    fn parse_decimal(u: &[u8]) -> Result<Self, error::ParseIntErr> {
        Self::unsinged_decimal_core(u, false, false)
    }

    fn unsinged_decimal_core(_u: &[u8], _neg_max_check: bool, _pos_max_check: bool) -> Result<Self, error::ParseIntErr> {
        unimplemented!("This function should be implemented in the child struct")
    }
}

#[cfg(test)]
mod tests {
    use crate::Biscuit;
    use crate::error::ParseIntErr;
    use anyhow::Result;
    use atoi::atoi;

    #[test]
    fn test_base() -> Result<()> {
        let err_overflow = i32::parse_decimal(b"1234567890123");
        assert_eq!(err_overflow, Err(ParseIntErr::Overflow));

        let val_i64 = i64::parse_decimal(b"1234567890123");
        assert_eq!(val_i64, Ok(1234567890123));

        let nondecimal = i32::parse_decimal(b"123a");
        assert_eq!(nondecimal, Err(ParseIntErr::NonDecimal));

        let nondecimal = i32::parse_decimal(b"a123");
        assert_eq!(nondecimal, Err(ParseIntErr::NonDecimal));

        let err_empty = i32::parse_decimal(b"");
        assert_eq!(err_empty, Err(ParseIntErr::Empty));

        let u128_max_str = b"340282366920938463463374607431768211455";
        let val_u128_max = u128::parse_decimal(u128_max_str)?;
        assert_eq!(val_u128_max, u128::MAX);

        let u128_overflow_str = b"340282366920938463463374607431768211456";
        let err_u128_overflow = u128::parse_decimal(u128_overflow_str);
        assert_eq!(err_u128_overflow, Err(ParseIntErr::Overflow));
        
        let i128_max_str = b"170141183460469231731687303715884105727";
        let val_i128_max = i128::parse_decimal(i128_max_str)?;
        assert_eq!(val_i128_max, i128::MAX);

        let i128_overflow_str = b"170141183460469231731687303715884105728";
        let i128_overflowed_value = i128::parse_decimal(i128_overflow_str);
        // it is under the maximum of u128 
        assert_eq!(i128_overflowed_value, Err(ParseIntErr::Overflow));

        let i128_leading_zero_str = b"00000000000000000000000000000000000000000000001234";
        let val_i128_leading_zero = i128::parse_decimal(i128_leading_zero_str)?;
        assert_eq!(val_i128_leading_zero, 1234);
        
        Ok(())
    }

    #[test]
    fn test_atoi() -> Result<()> {
        let err_overflow = atoi::<i32>(b"1234567890123");
        assert_eq!(err_overflow, None);

        let val_i64 = atoi::<i64>(b"1234567890123");
        assert_eq!(val_i64, Some(1234567890123));

        let nondecimal = atoi::<i64>(b"123a");
        assert_eq!(nondecimal, Some(123));

        let nondecimal = atoi::<i64>(b"a123");
        assert_eq!(nondecimal, None);

        let err_empty = atoi::<i64>(b"");
        assert_eq!(err_empty, None);

        let u128_max_str = b"340282366920938463463374607431768211455";
        let val_u128_max = atoi::<u128>(u128_max_str);
        assert_eq!(val_u128_max, Some(u128::MAX));

        let u128_overflow_str = b"340282366920938463463374607431768211456";
        let err_u128_overflow = atoi::<u128>(u128_overflow_str);
        assert_eq!(err_u128_overflow, None);
        
        let i128_max_str = b"170141183460469231731687303715884105727";
        let val_i128_max = atoi::<i128>(i128_max_str);
        assert_eq!(val_i128_max, Some(i128::MAX));

        let i128_overflow_str = b"170141183460469231731687303715884105728";
        let i128_overflowed_value = atoi::<i128>(i128_overflow_str);
        // it is under the maximum of u128 
        assert_eq!(i128_overflowed_value, None);

        let i128_leading_zero_str = b"000000000000000000000000000000000000000000000000001234";
        let val_i128_leading_zero = atoi::<i128>(i128_leading_zero_str);
        assert_eq!(val_i128_leading_zero, Some(1234));
        
        Ok(())
    }

    #[test]
    fn test_std() -> Result<()> {
        let err_overflow = "1234567890123".parse::<i32>();
        match err_overflow {
            Ok(_) => panic!("Should have failed to parse"),
            Err(e) => assert_eq!(*e.kind(), std::num::IntErrorKind::PosOverflow),
        }

        let val_i64 = "1234567890123".parse::<i64>();
        assert_eq!(val_i64, Ok(1234567890123));

        let nondecimal = "123a".parse::<i64>();
        match nondecimal {
            Ok(_) => panic!("Should have failed to parse"),
            Err(e) => assert_eq!(*e.kind(), std::num::IntErrorKind::InvalidDigit),
        }

        let nondecimal = "a123".parse::<i64>();
        match nondecimal {
            Ok(_) => panic!("Should have failed to parse"),
            Err(e) => assert_eq!(*e.kind(), std::num::IntErrorKind::InvalidDigit),
        }

        let err_empty = "".parse::<i64>();
        match err_empty {
            Ok(_) => panic!("Should have failed to parse"),
            Err(e) => assert_eq!(*e.kind(), std::num::IntErrorKind::Empty),
        }

        let u128_max_str = "340282366920938463463374607431768211455".parse::<u128>();
        assert_eq!(u128_max_str, Ok(u128::MAX));

        let u128_overflow_str = "340282366920938463463374607431768211456".parse::<u128>();
        match u128_overflow_str {
            Ok(_) => panic!("Should have failed to parse"),
            Err(e) => assert_eq!(*e.kind(), std::num::IntErrorKind::PosOverflow),
        }

        let i128_max_str = "170141183460469231731687303715884105727".parse::<i128>();
        assert_eq!(i128_max_str, Ok(i128::MAX));

        let i128_overflow_str = "170141183460469231731687303715884105728".parse::<i128>();
        match i128_overflow_str {
            Ok(_) => panic!("Should have failed to parse"),
            Err(e) => assert_eq!(*e.kind(), std::num::IntErrorKind::PosOverflow),
        }

        let i128_leading_zero_str = "000000000000000000000000000000000000000000000000001234".parse::<i128>();
        assert_eq!(i128_leading_zero_str, Ok(1234));

        Ok(())
    }
}