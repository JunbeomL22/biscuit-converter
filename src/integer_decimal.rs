use crate::BiscuitConverter;
use crate::error::{
    CheckError, Empty, NonDecimal
};

impl BiscuitConverter {
    #[inline]
    pub fn to_i128<T: AsRef<[u8]>>(self, input: T) -> Option<i128> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return None;
        }
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u128(&u[1..]);   
                unsigned.map(|val| (!(val as i128)).wrapping_add(1))
            },
            _ => self.to_u128(u).map(|val| val as i128),
        }
    }

    #[inline]
    pub fn to_i64<T: AsRef<[u8]>>(self, input: T) -> Option<i64> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return None;
        }
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u64(&u[1..]);
                unsigned.map(|val| (!(val as i64)).wrapping_add(1))   
            },
            _ => self.to_u64(u).map(|val| val as i64),
        }
    }

    #[inline]
    pub fn to_i32_decimal<T: AsRef<[u8]>>(self, input: T) -> Result<i32, CheckError> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return Err(CheckError::Empty(Empty));
        }
        match u[0] == b'-' {
            true => {
                match self.to_u32_decimal(&u[1..]) {
                    Ok(res) => Ok((!(res as i32)).wrapping_add(1)),
                    Err(e) => Err(e),
                }
            },
            _ => self.to_u32_decimal(u).map(|val| val as i32),
        }
    }

    /// # Safety
    /// The caller must ensure that the input is a valid decimal number
    pub unsafe fn to_i16_unchecked<T: AsRef<[u8]>>(self, input: T) -> Result<i16, CheckError> {
        let u = input.as_ref();
        match u[0] == b'-' {
            true => {
                if let Ok(unsigned) = unsafe {self.to_u16_unchecked(&u[1..]) } {
                    Ok((!(unsigned as i16)).wrapping_add(1))
                } else {
                    Err(CheckError::NonDecimal(NonDecimal))
                }
            },
            _ => unsafe {self.to_u16_unchecked(u).map(|val| val as i16)},
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
        assert_eq!(unsafe {biscuit_parser.to_i16_unchecked("-1234").unwrap() }, -1234);
        assert_eq!(biscuit_parser.to_i32_decimal("-123456789").unwrap(), -123456789);
        assert_eq!(biscuit_parser.to_i64("-123456789012345").unwrap(), -123456789012345);
        assert_eq!(biscuit_parser.to_i128("-1234567890123456789012345").unwrap(), -1234567890123456789012345);
        
        Ok(())
    }
}
