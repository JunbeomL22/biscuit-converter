#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use anyhow::Result;
    const I128_LENGTH_BOUND: usize = 40;  // Adjusted for i128, including sign

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in -1_000_000..=1_000_000 {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_i128(x_byte).unwrap();
            if i >= 0 && (i as u128) <= u128::MAX {
                assert_eq!(
                    val, i as i128,
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
    fn test_to_i128() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        
        // Test empty input
        let empty: &[u8] = &[];
        assert_eq!(biscuit.to_i128(empty), None, "Failed for empty input");

        // Test single zero
        let single_zero: &[u8] = &[b'0'];
        assert_eq!(biscuit.to_i128(single_zero), Some(0), "Failed for single zero");

        for i in 1..I128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i128(x);
            assert_eq!(
                val, Some(0),
                "Failed for {} bytes", i
            );
        }
        for i in 2..I128_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'1'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i128(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i128>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..(I128_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_i128(x).unwrap();
            let expected: i128 = std::str::from_utf8(x)?.parse::<u128>()? as i128;
            assert_eq!(
                val, expected,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_i128_extremes() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        
        // Test values near i128::MAX
        let near_max = i128::MAX - 1;
        let near_max_string = near_max.to_string();
        let near_max_byte: &[u8] = near_max_string.as_bytes();
        let val = biscuit.to_i128(near_max_byte);
        assert_eq!(val, Some(near_max), "Failed for i128::MAX - 1");

        let max_string = i128::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = biscuit.to_i128(max_byte);
        assert_eq!(val, Some(i128::MAX), "Failed for i128::MAX");

        // Test wrapping behavior at i128::MAX + 1
        let byte_test_p1 = b"170141183460469231731687303715884105728";  // i128::MAX + 1
        let val_p1 = biscuit.to_i128(byte_test_p1);
        assert_eq!(val_p1, Some(i128::MIN), "Unexpected behavior for i128::MAX + 1");

        // Test values slightly above i128::MAX + 1
        let byte_test_p2 = b"170141183460469231731687303715884105729";  // i128::MAX + 2
        let val_p2 = biscuit.to_i128(byte_test_p2);
        assert_eq!(val_p2, Some(i128::MIN + 1), "Unexpected behavior for i128::MAX + 2");

        // Test values near i128::MIN
        let near_min = i128::MIN + 1;
        let near_min_string = near_min.to_string();
        let near_min_byte: &[u8] = near_min_string.as_bytes();
        let val = biscuit.to_i128(near_min_byte);
        assert_eq!(val, Some(near_min), "Failed for i128::MIN + 1");

        let min_string = i128::MIN.to_string();
        let min_byte: &[u8] = min_string.as_bytes();
        let val = biscuit.to_i128(min_byte);
        assert_eq!(val, Some(i128::MIN), "Failed for i128::MIN");

        // Test wrapping behavior at i128::MIN - 1
        let byte_test_n1 = b"-170141183460469231731687303715884105729";  // i128::MIN - 1
        let val_n1 = biscuit.to_i128(byte_test_n1);
        assert_eq!(val_n1, Some(i128::MAX), "Unexpected behavior for i128::MIN - 1");
        
        // Test u128::MAX (should be interpreted as -1 in two's complement)
        let u128_max_string = u128::MAX.to_string();
        let u128_max_byte: &[u8] = u128_max_string.as_bytes();
        let val = biscuit.to_i128(u128_max_byte);
        assert_eq!(val, Some(-1), "Failed for u128::MAX");
        
        Ok(())
    }
    
    #[test]
    fn test_i128_leading_zeros() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros_pos = b"0000000890123456789012345678901234567890";
        let byte_leading_zeros_neg = b"-00000000890123456789012345678901234567890";
        let val_leading_zeros_pos = biscuit.to_i128(byte_leading_zeros_pos);
        let val_leading_zeros_neg = biscuit.to_i128(byte_leading_zeros_neg);
        assert_eq!(val_leading_zeros_pos, Some(890123456789012345678901234567890));
        assert_eq!(val_leading_zeros_neg, Some(-890123456789012345678901234567890));
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
            "-1234567890123456789",
            "-12345678901234567890",
            "-123456789012345678901",
            "-1234567890123456789012",
            "-12345678901234567890123",
            "-123456789012345678901234",
            "-1234567890123456789012345",
            "-12345678901234567890123456",
            "-123456789012345678901234567",
            "-1234567890123456789012345678",
            "-12345678901234567890123456789",
            "-123456789012345678901234567890",
            "-1234567890123456789012345678901",
            "-12345678901234567890123456789012",
            "-123456789012345678901234567890123",
            "-1234567890123456789012345678901234",
            "-12345678901234567890123456789012345",
            "-123456789012345678901234567890123456",
            "-1234567890123456789012345678901234567",
            "-12345678901234567890123456789012345678",
        ];
        
        for test in test_vec.iter() {
            let x = test.as_bytes();
            let val = biscuit.to_i128(x).unwrap();
            assert_eq!(val, test.parse::<i128>()?);
        }

        Ok(())
    }
}