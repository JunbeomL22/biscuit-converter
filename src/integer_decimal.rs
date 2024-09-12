use crate::BiscuitConverter;
use crate::error::CheckError;

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
    pub fn to_i128_decimal(self, u: &[u8]) -> Result<i128, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        if u.len() == 39 {
            match u[0] {
                b'-' => {
                    match self.to_u128_decimal(&u[1..]) {
                        Ok(val) => {
                            if val > I128_MIN_ABS_AS_U128 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok((!(val as i128)).wrapping_add(1))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    match self.to_u128_decimal(u) {
                        Ok(val) => {
                            if val > I128_MAX_AS_U128 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok(val as i128)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            match u[0] {
                b'-' => self.to_u128_decimal(&u[1..]).map(|val| (!(val as i128)).wrapping_add(1)),
                _ => self.to_u128_decimal(u).map(|val| val as i128),
            }
        }

    }

    #[inline]
    pub fn to_i64_decimal(self, u: &[u8]) -> Result<i64, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        if u.len() == 20 {
            match u[0] {
                b'-' => {
                    match self.to_u64_decimal(&u[1..]) {
                        Ok(val) => {
                            if val > I64_MIN_ABS_AS_U64 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok((!(val as i64)).wrapping_add(1))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    match self.to_u64_decimal(u) {
                        Ok(val) => {
                            if val > I64_MAX_AS_U64 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok(val as i64)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            match u[0] {
                b'-' => self.to_u64_decimal(&u[1..]).map(|val| (!(val as i64)).wrapping_add(1)),
                _ => self.to_u64_decimal(u).map(|val| val as i64),
            }
        }        
    }


    #[inline]
    pub fn to_i32_decimal(self, u: &[u8]) -> Result<i32, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        if u.len() == 10 {
            match u[0] {
                b'-' => {
                    match self.to_u32_decimal(&u[1..]) {
                        Ok(val) => {
                            if val > I32_MIN_ABS_AS_U32 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok((!(val as i32)).wrapping_add(1))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    match self.to_u32_decimal(u) {
                        Ok(val) => {
                            if val > I32_MAX_AS_U32 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok(val as i32)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            match u[0] {
                b'-' => self.to_u32_decimal(&u[1..]).map(|val| (!(val as i32)).wrapping_add(1)),
                _ => self.to_u32_decimal(u).map(|val| val as i32),
            }
        }
    }

    #[inline]
    pub fn to_i16_decimal(self, u: &[u8]) -> Result<i16, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        if u.len() == 5 {
            match u[0] {
                b'-' => {
                    match self.to_u16_decimal(&u[1..]) {
                        Ok(val) => {
                            if val > I16_MIN_ABS_AS_U16 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok((!(val as i16)).wrapping_add(1))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    match self.to_u16_decimal(u) {
                        Ok(val) => {
                            if val > I16_MAX_AS_U16 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok(val as i16)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            match u[0] {
                b'-' => self.to_u16_decimal(&u[1..]).map(|val| (!(val as i16)).wrapping_add(1)),
                _ => self.to_u16_decimal(u).map(|val| val as i16),
            }
        }
    }

    #[inline]
    pub fn to_i8_decimal(self, u: &[u8]) -> Result<i8, CheckError> {
        if u.is_empty() { return Err(CheckError::Empty); }
        if u.len() == 3 {
            match u[0] {
                b'-' => {
                    match self.to_u8_decimal(&u[1..]) {
                        Ok(val) => {
                            if val > I8_MIN_ABS_AS_U8 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok((!(val as i8)).wrapping_add(1))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    match self.to_u8_decimal(u) {
                        Ok(val) => {
                            if val > I8_MAX_AS_U8 {
                                Err(CheckError::Overflow)
                            } else {
                                Ok(val as i8)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        } else {
            match u[0] {
                b'-' => self.to_u8_decimal(&u[1..]).map(|val| (!(val as i8)).wrapping_add(1)),
                _ => self.to_u8_decimal(u).map(|val| val as i8),
            }
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

 