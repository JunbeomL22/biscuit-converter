#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use anyhow::Result;
    const U32_LENGTH_BOUND: usize = 10;  // Adjusted for u32

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 1..1_000_000 {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_u32(x_byte).unwrap();
            assert_eq!(
                val, i,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_to_u32() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 1..U32_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u32(x);
            assert_eq!(
                val, Some(0),
                "Failed for {} bytes", i
            );
        }
        for i in 1..U32_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'1'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u32(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u32>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..(U32_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u32(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u32>()?,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_u32_max() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let max_string = u32::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = biscuit.to_u32(max_byte);
        assert_eq!(val, Some(u32::MAX));
        let byte_test_p1 = b"4294967296";  // u32::MAX + 1
        let byte_test_n1_string = (u32::MAX - 1).to_string();
        let byte_test_n1: &[u8] = byte_test_n1_string.as_bytes();
        let x_p1: &[u8] = &byte_test_p1[..];
        let x_n1: &[u8] = &byte_test_n1[..];
        let val_p1 = biscuit.to_u32(x_p1);
        let val_n1 = biscuit.to_u32(x_n1);
        assert_eq!(val_p1, None);
        assert_eq!(val_n1, Some(u32::MAX - 1));
        Ok(())
    }

    #[test]
    fn test_u32_leading_zeros() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros = b"01234567890";
        let x_leading_zeros: &[u8] = &byte_leading_zeros[..];
        let val_leading_zeros = biscuit.to_u32(x_leading_zeros);
        assert_eq!(val_leading_zeros, Some(1234567890));
        Ok(())
    }
}