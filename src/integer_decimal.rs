use crate::BiscuitConverter;
use crate::error::CheckError;

impl BiscuitConverter {
    #[inline]
    pub fn to_i128_decimal(self, u: &[u8]) -> Result<i128, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        match u[0] {
            b'-' => self.to_u128_decimal_core(&u[1..], true, false).map(|val| (!(val as i128)).wrapping_add(1)),
            _ => self.to_u128_decimal_core(u, false, true).map(|val| val as i128),
        }
    }

    #[inline]
    pub fn to_i64_decimal(self, u: &[u8]) -> Result<i64, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
    
        match u[0] {
            b'-' => self.to_u64_decimal_core(&u[1..], true, false).map(|val| (!(val as i64)).wrapping_add(1)),
            _ => self.to_u64_decimal_core(u, false, true).map(|val| val as i64),
        }
    }


    #[inline]
    pub fn to_i32_decimal(self, u: &[u8]) -> Result<i32, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        
        match u[0] {
            b'-' => self.to_u32_decimal_core(&u[1..], true, false).map(|val| (!(val as i32)).wrapping_add(1)),
            _ => self.to_u32_decimal_core(u, false, true).map(|val| val as i32),
        }
    }

    #[inline]
    pub fn to_i16_decimal(self, u: &[u8]) -> Result<i16, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        match u[0] {
            b'-' => self.to_u16_decimal_core(&u[1..], true, false).map(|val| (!(val as i16)).wrapping_add(1)),
            _ => self.to_u16_decimal_core(u, false, true).map(|val| val as i16),
        }
    }

    #[inline]
    pub fn to_i8_decimal(self, u: &[u8]) -> Result<i8, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
            
        match u[0] {
            b'-' => self.to_u8_decimal_core(&u[1..], true, false).map(|val| (!(val as i8)).wrapping_add(1)),
            _ => self.to_u8_decimal_core(u, false, true).map(|val| val as i8),
        }
        

    }
}

/*

    #[inline]
    pub fn to_i64_decimal(self, u: &[u8]) -> Result<i64, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        match u[0] {
            b'-' => self.to_u64_decimal(&u[1..]).map(|val| (!(val as i64)).wrapping_add(1)),
            _ => self.to_u64_decimal(u).map(|val| val as i64),
        }
    }

    #[inline]
    pub fn to_i32_decimal(self, u: &[u8]) -> Result<i32, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        match u[0] {
            b'-' => self.to_u32_decimal(&u[1..]).map(|val| (!(val as i32)).wrapping_add(1)),
            _ => self.to_u32_decimal(u).map(|val| val as i32),
        }
    }

    #[inline]
    pub fn to_i16_decimal(self, u: &[u8]) -> Result<i16, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        match u[0] {
            b'-' => self.to_u16_decimal(&u[1..]).map(|val| (!(val as i16)).wrapping_add(1)),
            _ => self.to_u16_decimal(u).map(|val| val as i16),
        }
    }

    #[inline]
    pub fn to_i8_decimal(self, u: &[u8]) -> Result<i8, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        match u[0] {
            b'-' => self.to_u8_decimal(&u[1..]).map(|val| (!(val as i8)).wrapping_add(1)),
            _ => self.to_u8_decimal(u).map(|val| val as i8),
        }
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_conversion() -> Result<()> {
        let biscuit_parser = BiscuitConverter::default();
        assert_eq!(biscuit_parser.to_i16_decimal(b"-1234"), Ok(-1234));
        assert_eq!(biscuit_parser.to_i32_decimal(b"-123456789"), Ok(-123456789));
        assert_eq!(biscuit_parser.to_i64_decimal(b"-123456789012345"), Ok(-123456789012345));
        assert_eq!(biscuit_parser.to_i128_decimal(b"-1234567890123456789012345"), Ok(-1234567890123456789012345));
        
        Ok(())
    }
}

 