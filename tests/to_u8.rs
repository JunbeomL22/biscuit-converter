#[cfg(test)]
mod tests {
    use biscuit_converter::Biscuit;
    use anyhow::Result;

    #[test]
    fn test_back_and_forth() -> Result<()> {
        
        for i in 0..u8::MAX {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = u8::parse_decimal(x_byte);
            assert_eq!(
                val, Ok(i),
                "Failed for {} the string: \"{}\" byte: {:?}", i, x, x_byte,
            );
        }
        Ok(())
    }
}