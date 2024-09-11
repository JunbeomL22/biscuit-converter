use crate::BiscuitConverter;
use crate::error::{
    CheckError, Empty, NonDecimal
};

impl BiscuitConverter {
    #[inline]
    pub fn to_i128_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<i128, CheckError> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return Err(CheckError::Empty(Empty));
        }
        match u[0] {
            b'0' => {
                if let Ok(val) = self.to_u128_decimal(&u[1..]) {
                    Ok((!(val as i128)).wrapping_add(1))
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            _ => self.to_u128_decimal(u).map(|val| val as i128),
        }
    }

    #[inline]
    pub fn to_i64_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<i64, CheckError> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return Err(CheckError::Empty(Empty));
        }
        match u[0] {
            b'0' => {
                if let Ok(val) = self.to_u64_decimal(&u[1..]) {
                    Ok((!(val as i64)).wrapping_add(1))
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            _ => self.to_u64_decimal(u).map(|val| val as i64),
        }
    }

    #[inline]
    pub fn to_i32_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<i32, CheckError> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return Err(CheckError::Empty(Empty));
        }
        match u[0] {
            b'-' => {
                match self.to_u32_decimal(&u[1..]) {
                    Ok(res) => Ok((!(res as i32)).wrapping_add(1)),
                    Err(e) => Err(e),
                }
            },
            _ => self.to_u32_decimal(u).map(|val| val as i32),
        }
    }

    pub fn to_i16_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<i16, CheckError> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return Err(CheckError::Empty(Empty));
        }
        match u[0] == b'-' {
            true => {
                if let Ok(unsigned) = self.to_u16_decimal(&u[1..]) {
                    Ok((!(unsigned as i16)).wrapping_add(1))
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            _ => self.to_u16_decimal(u).map(|val| val as i16),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_conversion() -> Result<()> {
        let biscuit_parser = BiscuitConverter::default();
        assert_eq!(biscuit_parser.to_i16_decimal("-1234"), Ok(-1234));
        assert_eq!(biscuit_parser.to_i32_decimal("-123456789"), Ok(-123456789));
        assert_eq!(biscuit_parser.to_i64_decimal("-123456789012345"), Ok(-123456789012345));
        assert_eq!(biscuit_parser.to_i128_decimal("-1234567890123456789012345").unwrap(), -1234567890123456789012345);
        
        Ok(())
    }
}
