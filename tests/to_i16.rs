#[cfg(test)]
mod tests {
    use biscuit_converter::Biscuit;
    use biscuit_converter::error::ParseIntErr;
    use anyhow::Result;

    #[test]
    fn test_back_and_forth() -> Result<()> {
        for i in i16::MIN..=i16::MAX {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = i16::parse_decimal(x_byte).unwrap();
            assert_eq!(
                val, i,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_i16_extremes() -> Result<()> {
        // Test i16::MAX
        let max_string = i16::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = i16::parse_decimal(max_byte);
        assert_eq!(val, Ok(i16::MAX));
        
        // Test i16::MIN
        let min_string = i16::MIN.to_string();
        let min_byte: &[u8] = min_string.as_bytes();
        let val = i16::parse_decimal(min_byte);
        assert_eq!(val, Ok(i16::MIN));
        
        // Test Overflow
        let byte_test_p1 = b"32768";  // i16::MAX + 1
        let byte_test_n1 = b"-32769";  // i16::MIN - 1
        let val_p1 = i16::parse_decimal(byte_test_p1);
        let val_n1 = i16::parse_decimal(byte_test_n1);
        assert_eq!(val_p1, Err(ParseIntErr::Overflow));
        assert_eq!(val_n1, Err(ParseIntErr::NegOverflow));
        
        Ok(())
    }

    #[test]
    fn test_i16_leading_zeros() -> Result<()> {
        let byte_leading_zeros_pos = b"012345";
        let byte_leading_zeros_neg = b"-012345";
        let val_leading_zeros_pos = i16::parse_decimal(byte_leading_zeros_pos);
        let val_leading_zeros_neg = i16::parse_decimal(byte_leading_zeros_neg);
        assert_eq!(val_leading_zeros_pos, Ok(12345));
        assert_eq!(val_leading_zeros_neg, Ok(-12345));
        Ok(())
    }
}