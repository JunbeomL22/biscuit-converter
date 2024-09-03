use crate::FlashParser;

impl FlashParser {
    #[inline(always)]
    pub fn to_i128<T: AsRef<[u8]>>(self, input: T) -> i128 {
        let u = input.as_ref();
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u128(&u[1..]);   
                (!unsigned).wrapping_add(1) as i128
            },
            _ => self.to_u128(u) as i128,
        }
    }

    #[inline(always)]
    pub fn to_i64<T: AsRef<[u8]>>(self, input: T) -> i64 {
        let u = input.as_ref();
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u64(&u[1..]);   
                (!unsigned).wrapping_add(1) as i64
            },
            _ => self.to_u64(u) as i64,
        }
    }

    #[inline(always)]
    pub fn to_i32<T: AsRef<[u8]>>(self, input: T) -> i32 {
        let u = input.as_ref();
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u32(&u[1..]);   
                (!unsigned).wrapping_add(1) as i32
            },
            _ => self.to_u32(u) as i32,
        }
    }

    pub fn to_i16<T: AsRef<[u8]>>(self, input: T) -> i16 {
        let u = input.as_ref();
        match u[0] == b'-' {
            true => {
                let unsigned = self.to_u16(&u[1..]);   
                (!unsigned).wrapping_add(1) as i16
            },
            _ => self.to_u16(u) as i16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_conversion() -> Result<()> {
        let flash_parser = FlashParser::default();
        assert_eq!(flash_parser.to_i16("-1234"), -1234);
        assert_eq!(flash_parser.to_i32("-123456789"), -123456789);
        assert_eq!(flash_parser.to_i64("-123456789012345"), -123456789012345);
        assert_eq!(flash_parser.to_i128("-1234567890123456789012345"), -1234567890123456789012345);
        
        Ok(())
    }
}
