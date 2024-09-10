use crate::BiscuitConverter;
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

    #[inline(always)]
    pub fn to_i32<T: AsRef<[u8]>>(self, input: T) -> Option<i32> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return None;
        }
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u32(&u[1..]);   
                unsigned.map(|val| (!(val as i32)).wrapping_add(1))
            },
            _ => self.to_u32(u).map(|val| val as i32),
        }
    }

    pub fn to_i16<T: AsRef<[u8]>>(self, input: T) -> Option<i16> {
        let u = input.as_ref();
        if u.is_empty() || u == b"-" {
            return None;
        }
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u16(&u[1..]);   
                unsigned.map(|val| (!(val as i16)).wrapping_add(1))
            },
            _ => self.to_u16(u).map(|val| val as i16),
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
        assert_eq!(biscuit_parser.to_i16("-1234").unwrap(), -1234);
        assert_eq!(biscuit_parser.to_i32("-123456789").unwrap(), -123456789);
        assert_eq!(biscuit_parser.to_i64("-123456789012345").unwrap(), -123456789012345);
        assert_eq!(biscuit_parser.to_i128("-1234567890123456789012345").unwrap(), -1234567890123456789012345);
        
        Ok(())
    }
}
