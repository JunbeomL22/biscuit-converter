#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use biscuit_converter::error::CheckError;
    use anyhow::Result;
    const I64_LENGTH_BOUND: usize = 20;  // Adjusted for i64, including sign

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in -1_000_000..=1_000_000 {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_i64_decimal(x_byte).unwrap();
            if i >= 0 && (i as u64) <= u64::MAX {
                assert_eq!(
                    val, i as i64,
                    "Failed for positive {}", i
                );
            } else {
                assert_eq!(
                    val, i,
                    "Failed for negative {}", i
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_to_i64() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        
        // Test empty input
        let empty: &[u8] = &[];
        assert_eq!(biscuit.to_i64_decimal(empty), Err(CheckError::Empty), "Failed for empty input");

        // Test single zero
        let single_zero: &[u8] = &[b'0'];
        assert_eq!(biscuit.to_i64_decimal(single_zero), Ok(0), "Failed for single zero");

        for i in 1..I64_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i64_decimal(x);
            assert_eq!(
                val, Ok(0),
                "Failed for {} bytes", i
            );
        }
        for i in 2..I64_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'1'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i64_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i64>()?,
                "Failed for {} bytes", i
            );
        }

        for i in 1..(I64_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i64_decimal(x).unwrap();
            let expected: i64 = std::str::from_utf8(x)?.parse::<u64>()? as i64;
            assert_eq!(
                val, expected,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_i64_extremes() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        
        // Test values near i64::MAX
        let near_max = i64::MAX - 1;
        let near_max_string = near_max.to_string();
        let near_max_byte: &[u8] = near_max_string.as_bytes();
        let val = biscuit.to_i64_decimal(near_max_byte);
        assert_eq!(val, Ok(near_max), "Failed for i64::MAX - 1");

        let max_string = i64::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = biscuit.to_i64_decimal(max_byte);
        assert_eq!(val, Ok(i64::MAX), "Failed for i64::MAX");

        // Test wrapping behavior at i64::MAX + 1
        let byte_test_p1 = b"9223372036854775808";  // i64::MAX + 1
        let val_p1 = biscuit.to_i64_decimal(byte_test_p1);
        assert_eq!(val_p1, Ok(i64::MIN), "Unexpected behavior for i64::MAX + 1");

        // Test values slightly above i64::MAX + 1
        let byte_test_p2 = b"9223372036854775809";  // i64::MAX + 2
        let val_p2 = biscuit.to_i64_decimal(byte_test_p2);
        assert_eq!(val_p2, Ok(i64::MIN + 1), "Unexpected behavior for i64::MAX + 2");

        // Test values near i64::MIN
        let near_min = i64::MIN + 1;
        let near_min_string = near_min.to_string();
        let near_min_byte: &[u8] = near_min_string.as_bytes();
        let val = biscuit.to_i64_decimal(near_min_byte);
        assert_eq!(val, Ok(near_min), "Failed for i64::MIN + 1");

        let min_string = i64::MIN.to_string();
        let min_byte: &[u8] = min_string.as_bytes();
        let val = biscuit.to_i64_decimal(min_byte);
        assert_eq!(val, Ok(i64::MIN), "Failed for i64::MIN");

        // Test wrapping behavior at i64::MIN - 1
        let byte_test_n1 = b"-9223372036854775809";  // i64::MIN - 1
        let val_n1 = biscuit.to_i64_decimal(byte_test_n1);
        assert_eq!(val_n1, Err(CheckError::Overflow), "Unexpected behavior for i64::MIN - 1");
        
        // Test u64::MAX (should be interpreted as -1 in two's complement)
        let u64_max_string = u64::MAX.to_string();
        let u64_max_byte: &[u8] = u64_max_string.as_bytes();
        let val = biscuit.to_i64_decimal(u64_max_byte);
        assert_eq!(val, Err(CheckError::Overflow), "Failed for u64::MAX");
        
        Ok(())
    }
    
    #[test]
    fn test_i64_leading_zeros() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros_pos = b"01234567890123456789";
        let byte_leading_zeros_neg = b"-01234567890123456789";
        let val_leading_zeros_pos = biscuit.to_i64_decimal(byte_leading_zeros_pos);
        let val_leading_zeros_neg = biscuit.to_i64_decimal(byte_leading_zeros_neg);
        assert_eq!(val_leading_zeros_pos, Ok(1234567890123456789));
        assert_eq!(val_leading_zeros_neg, Ok(-1234567890123456789));
        Ok(())
    }

    #[test]
    fn test_base() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let test_vec = vec![
            "-0",
            "0",
            "-1",
            "-12",
            "-123",
            "-1234",
            "-12345",
            "-123456",
            "-1234567",
            "-12345678",
            "-123456789",
            "-1234567890",
            "-12345678901",
            "-123456789012",
            "-1234567890123",
            "-12345678901234",
            "-123456789012345",
            "-1234567890123456",
            "-12345678901234567",
            "-123456789012345678",
        ];
        
        for input_str in test_vec {
            let input = input_str.as_bytes();
            let val = biscuit.to_i64_decimal(input).unwrap();
            assert_eq!(
                val, input_str.parse::<i64>()?,
                "Failed for {}", input_str
            );
        }

        Ok(())
    }
}