#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use anyhow::Result;
    const I32_LENGTH_BOUND: usize = 11;  // Adjusted for i32, including sign

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in (-1_000_000..1_000_000).step_by(1000) {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_i32(x_byte).unwrap();
            assert_eq!(
                val, i,
                "Failed for {}", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_to_i32() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 2..I32_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'0'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i32(x);
            assert_eq!(
                val, Some(0),
                "Failed for {} bytes", i
            );
        }
        for i in 2..I32_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'1'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i32(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i32>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..(I32_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i32(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i32>()?,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_i32_extremes() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        
        // Test i32::MAX
        let max_string = i32::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = biscuit.to_i32(max_byte);
        assert_eq!(val, Some(i32::MAX));
        
        // Test i32::MIN
        let min_string = i32::MIN.to_string();
        let min_byte: &[u8] = min_string.as_bytes();
        let val = biscuit.to_i32(min_byte);
        assert_eq!(val, Some(i32::MIN));
        
        // Test overflow
        let byte_test_p1 = b"2147483648";  // i32::MAX + 1
        let byte_test_n1 = b"-2147483649";  // i32::MIN - 1
        let val_p1 = biscuit.to_i32(byte_test_p1);
        let val_n1 = biscuit.to_i32(byte_test_n1);
        assert_eq!(val_p1.unwrap(), -2147483648);
        assert_eq!(val_n1.unwrap(), 2147483647);
        
        Ok(())
    }

    #[test]
    fn test_i32_leading_zeros() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros_pos = b"01234567890";
        let byte_leading_zeros_neg = b"-01234567890";
        let val_leading_zeros_pos = biscuit.to_i32(byte_leading_zeros_pos);
        let val_leading_zeros_neg = biscuit.to_i32(byte_leading_zeros_neg);
        assert_eq!(val_leading_zeros_pos, Some(1234567890));
        assert_eq!(val_leading_zeros_neg, Some(-1234567890));
        Ok(())
    }
}