use crate::Biscuit;

impl Biscuit for i128 {
    #[inline]
    fn parse_decimal(u: &[u8]) -> Result<Self, crate::error::ParseIntErr> {
        if !u.is_empty() && u[0] == b'-' {
            u128::unsinged_decimal_core(&u[1..], true, false).map(|val| (!(val as i128)).wrapping_add(1))
        } else {
            u128::unsinged_decimal_core(u, false, true).map(|val| val as i128)
        }
    }
}
impl Biscuit for i64 {
    #[inline]
    fn parse_decimal(u: &[u8]) -> Result<Self, crate::error::ParseIntErr> {
        if !u.is_empty() && u[0] == b'-' {
            u64::unsinged_decimal_core(&u[1..], true, false).map(|val| (!(val as i64)).wrapping_add(1))
        } else {
            u64::unsinged_decimal_core(u, false, true).map(|val| val as i64)
        }
    }
}

impl Biscuit for i32 {
    #[inline]
    fn parse_decimal(u: &[u8]) -> Result<Self, crate::error::ParseIntErr> {
        if !u.is_empty() && u[0] == b'-' {
            u32::unsinged_decimal_core(&u[1..], true, false).map(|val| (!(val as i32)).wrapping_add(1))
        } else {
            u32::unsinged_decimal_core(u, false, true).map(|val| val as i32)
        }
    }
}

impl Biscuit for i16 {
    #[inline]
    fn parse_decimal(u: &[u8]) -> Result<Self, crate::error::ParseIntErr> {
        if !u.is_empty() && u[0] == b'-' {
            u16::unsinged_decimal_core(&u[1..], true, false).map(|val| (!(val as i16)).wrapping_add(1))
        } else {
            u16::unsinged_decimal_core(u, false, true).map(|val| val as i16)
        }
    }
}

impl Biscuit for i8 {
    #[inline]
    fn parse_decimal(u: &[u8]) -> Result<Self, crate::error::ParseIntErr> {
        if !u.is_empty() && u[0] == b'-' {
            u8::unsinged_decimal_core(u, true, false).map(|val| (!(val as i8)).wrapping_add(1))
        } else {
            u8::unsinged_decimal_core(u, false, true).map(|val| val as i8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_conversion() -> Result<()> {
        assert_eq!(i16::parse_decimal(b"-1234"), Ok(-1234));
        assert_eq!(i32::parse_decimal(b"-123456789"), Ok(-123456789));
        assert_eq!(i64::parse_decimal(b"-123456789012345"), Ok(-123456789012345));
        assert_eq!(i128::parse_decimal(b"-1234567890123456789012345"), Ok(-1234567890123456789012345));
        
        Ok(())
    }
}
