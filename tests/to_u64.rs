#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use anyhow::Result;

    const U64_LENGTH_BOUND: usize = 20;
    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();

        for i in 1..1_000_000 {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_u64(x_byte).unwrap();
            assert_eq!(
                val, i,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }
    #[test]
    fn test_to_u64() -> Result<()> {
        let biscuit = BiscuitConverter::default();

        for i in 1..U64_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u64(x);
            assert_eq!(
                val, Some(0),
                "Failed for {} bytes", i
            );
        }

        for i in 1..U64_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'1'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u64(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u64>()?,
                "Failed for {} bytes", i
            );
        }

        for i in 1..(U64_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u64(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u64>()?,
                "Failed for {} bytes", i
            );
        }

        Ok(())
    }

    #[test]
    fn test_u64_max() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let max_string = u64::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = biscuit.to_u64(max_byte);
        assert_eq!(val, Some(u64::MAX));

        let byte_test_p1 = b"18446744073709551616";
        let byte_test_n1_string = (u64::MAX - 1).to_string();
        let byte_test_n1: &[u8] = byte_test_n1_string.as_bytes();

        let x_p1: &[u8] = &byte_test_p1[..];
        let x_n1: &[u8] = &byte_test_n1[..];

        let val_p1 = biscuit.to_u64(x_p1);
        let val_n1 = biscuit.to_u64(x_n1);

        assert_eq!(val_p1, None);
        assert_eq!(val_n1, Some(u64::MAX - 1));

        Ok(())
    }

    #[test]
    fn test_u64_leading_zeros() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros = b"01234567890123456780";
        let x_leading_zeros: &[u8] = &byte_leading_zeros[..];
        let val_leading_zeros = biscuit.to_u64(x_leading_zeros);
        assert_eq!(val_leading_zeros, Some(1234567890123456780));

        Ok(())
    }

    #[test]
    fn test_base() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let test_vec = vec![
            "0",
            "1",
            "12",
            "123",
            "1234",
            "12345",
            "123456",
            "1234567",
            "12345678",
            "123456789",
            "1234567890",
            "12345678901",
            "123456789012",
            "1234567890123",
            "12345678901234",
            "123456789012345",
            "1234567890123456",
            "12345678901234567",
            "123456789012345678",
            "1234567890123456789",
        ];

        for input_str in test_vec {
            let input = input_str.as_bytes();
            let val = biscuit.to_u64(input).unwrap();
            assert_eq!(
                val, input_str.parse::<u64>()?,
                "Failed for {}", input_str
            );
        }

        Ok(())
    }

}