#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use biscuit_converter::error::{
        CheckError,
        Empty, 
        OverFlow,
        AdditionOverflow,
    };

    use anyhow::Result;
    const U16_LENGTH_BOUND: usize = 5;  // Adjusted for u16

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 1..65535 {  
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_u16_decimal(x_byte).unwrap();
            assert_eq!(
                val, i,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_to_u16_decimal() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 1..U16_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u16_decimal(x);
            assert_eq!(
                val, Ok(0),
                "Failed for {} bytes", i
            );
        }
        for i in 1..U16_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'1'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u16_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u16>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..(U16_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = biscuit.to_u16_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<u16>()?,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_u16_max() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let max_string = u16::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = biscuit.to_u16_decimal(max_byte).expect("to_u16 failed");
        assert_eq!(val, u16::MAX);
        let byte_test_p1 = b"65536";  // u16::MAX + 1
        let byte_test_n1_string = (u16::MAX - 1).to_string();
        let byte_test_n1: &[u8] = byte_test_n1_string.as_bytes();
        let x_p1: &[u8] = &byte_test_p1[..];
        let x_n1: &[u8] = &byte_test_n1[..];
        let val_p1 = biscuit.to_u16_decimal(x_p1);
        let val_n1 = biscuit.to_u16_decimal(x_n1);
        assert_eq!(val_p1, Err(CheckError::AdditionOverflow(AdditionOverflow)));
        assert_eq!(val_n1, Ok(u16::MAX - 1));
        Ok(())
    }

    #[test]
    fn test_u16_leading_zeros() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        let byte_leading_zeros = b"012345";
        let x_leading_zeros: &[u8] = &byte_leading_zeros[..];
        let val_leading_zeros = biscuit.to_u16_decimal(x_leading_zeros);
        assert_eq!(val_leading_zeros, Ok(12345));
        Ok(())
    }
}