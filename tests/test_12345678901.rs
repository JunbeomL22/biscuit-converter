#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u64("12345678901"), 12345678901);
        assert_eq!(biscuit_converter.to_u128("12345678901"), 12345678901);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i64("12345678901"), 12345678901);
        assert_eq!(biscuit_converter.to_i128("12345678901"), 12345678901);
        //
        assert_eq!(biscuit_converter.to_i64("-12345678901"), -12345678901);
        assert_eq!(biscuit_converter.to_i128("-12345678901"), -12345678901);
    }


}