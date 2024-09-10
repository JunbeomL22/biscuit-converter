#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use anyhow::Result;
    const U128_LENGTH_BOUND: usize = 39;
    
    #[test]
    fn test_to_u128() -> Result<()> {
        let biscuit = BiscuitConverter::default();

        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u128(x);
            assert_eq!(
                val, Some(0),
                "Failed for {} bytes", i
            );
        }

        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'1'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u128(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u128>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'2'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u128(x).unwrap();
            assert_eq!(val, std::str::from_utf8(x)?.parse::<u128>()?);
        }

        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'3'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u128(x).unwrap();
            assert_eq!(val, std::str::from_utf8(x)?.parse::<u128>()?);
        }

        Ok(())
    }

    #[test]
    fn test_u128_max_check() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_test = b"340282366920938463463374607431768211455";
        let x: &[u8] = &byte_test[..];
        let val = biscuit.to_u128(x).unwrap();
        assert_eq!(val, u128::MAX);

        let byte_test_p1 = b"340282366920938463463374607431768211456";
        let byte_test_n1 = b"340282366920938463463374607431768211454";

        let x_p1: &[u8] = &byte_test_p1[..];
        let x_n1: &[u8] = &byte_test_n1[..];

        let val_p1 = biscuit.to_u128(x_p1);
        let val_n1 = biscuit.to_u128(x_n1);

        assert_eq!(val_p1, None);
        assert_eq!(val_n1, Some(340282366920938463463374607431768211454));

        Ok(())
    }

    #[test]
    fn test_leading_zero_u128() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros = b"00000000000000000000000123456789012345678901234000000";
        let x_leading_zeros: &[u8] = &byte_leading_zeros[..];
        let val_leading_zeros = biscuit.to_u128(x_leading_zeros).unwrap();
        assert_eq!(val_leading_zeros, 1_234_567_890_123_456_789_012_340_000_00);

        Ok(())
    }

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 1..1_000_000 {
            let x = i.to_string();
            let x_bytes = x.as_bytes();
            let val = biscuit.to_u128(x_bytes).unwrap();
            
            assert_eq!(val, i);
        }
        Ok(())
    }

    
}