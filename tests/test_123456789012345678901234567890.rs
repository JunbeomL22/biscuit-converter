#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u128("123456789012345678901234567890"), 123456789012345678901234567890);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i128("123456789012345678901234567890"), 123456789012345678901234567890);
        //
        assert_eq!(biscuit_converter.to_i128("-123456789012345678901234567890"), -123456789012345678901234567890);
    }

}