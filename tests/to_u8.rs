#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use anyhow::Result;

    #[test]
    fn test_back_and_forth() -> Result<()> {
        let biscuit = BiscuitConverter::default();
        for i in 0..u8::MAX {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = biscuit.to_u8_decimal(x_byte);
            assert_eq!(
                val, Ok(i),
                "Failed for {} the string: \"{}\" byte: {:?}", i, x, x_byte,
            );
        }
        Ok(())
    }
}