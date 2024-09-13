#[cfg(test)]
mod tests {
    use biscuit_converter::Biscuit;
    use biscuit_converter::error::ParseIntErr;
    use anyhow::Result;
    const U128_LENGTH_BOUND: usize = 39;
    
    #[test]
    fn test_to_u128_decimal() -> Result<()> {
        

        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            println!("i: {}", i);
            let val = u128::parse_decimal(x);
            assert_eq!(
                val, Ok(0),
                "Failed for {} bytes", i
            );
        }

        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'1'; i];
            let x: &[u8] = &x_vec[..];
            println!("i: {}", i);
            let val = u128::parse_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u128>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'2'; i];
            let x: &[u8] = &x_vec[..];
            println!("i: {}", i);
            let val = u128::parse_decimal(x).unwrap();
            assert_eq!(val, std::str::from_utf8(x)?.parse::<u128>()?);
        }

        for i in 1..U128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'3'; i];
            let x: &[u8] = &x_vec[..];
            println!("i: {}", i);
            let val = u128::parse_decimal(x).unwrap();
            assert_eq!(val, std::str::from_utf8(x)?.parse::<u128>()?);
        }

        Ok(())
    }

    #[test]
    fn test_u128_max_check() -> Result<()> {
        
        let byte_test = u128::MAX.to_string();
        let byte_test = byte_test.as_bytes();
        let x: &[u8] = &byte_test[..];
        let val = u128::parse_decimal(x).unwrap();
        assert_eq!(val, u128::MAX);

        let byte_test_p1 = b"340282366920938463463374607431768211456";
        let byte_test_n1 = b"340282366920938463463374607431768211454";

        let x_p1: &[u8] = &byte_test_p1[..];
        let x_n1: &[u8] = &byte_test_n1[..];

        let val_p1 = u128::parse_decimal(x_p1);
        let val_n1 = u128::parse_decimal(x_n1);

        assert_eq!(val_p1, Err(ParseIntErr::Overflow));
        assert_eq!(val_n1, Ok(340282366920938463463374607431768211454));

        Ok(())
    }

    #[test]
    fn test_leading_zero_u128() -> Result<()> {
        
        let byte_leading_zeros = b"00000000000000000000000123456789012345678901234000000";
        let x_leading_zeros: &[u8] = &byte_leading_zeros[..];
        let val_leading_zeros = u128::parse_decimal(x_leading_zeros).unwrap();
        assert_eq!(val_leading_zeros, 1_234_567_890_123_456_789_012_340_000_00);

        Ok(())
    }

    #[test]
    fn test_back_and_forth() -> Result<()> {
        
        for i in 1..1_000_000 {
            let x = i.to_string();
            let x_bytes = x.as_bytes();
            let val = u128::parse_decimal(x_bytes).unwrap();
            
            assert_eq!(val, i);
        }
        Ok(())
    }

    #[test]
    fn test_base() -> Result<()> {
        let test_vecs = [
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
            "12345678901234567890",
            "123456789012345678901",
            "1234567890123456789012",
            "12345678901234567890123",
            "123456789012345678901234",
            "1234567890123456789012345",
            "12345678901234567890123456",
            "123456789012345678901234567",
            "1234567890123456789012345678",
            "12345678901234567890123456789",
            "123456789012345678901234567890",
            "1234567890123456789012345678901",
            "12345678901234567890123456789012",
            "123456789012345678901234567890123",
            "1234567890123456789012345678901234",
            "12345678901234567890123456789012345",
            "123456789012345678901234567890123456",
            "1234567890123456789012345678901234567",
            "12345678901234567890123456789012345678",
            "123456789012345678901234567890123456789",
        ];

        
        for test in test_vecs.iter() {
            let x = test.as_bytes();
            let val = u128::parse_decimal(x).unwrap();
            assert_eq!(val, test.parse::<u128>()?);
        }
        
        Ok(())
    }

    
}