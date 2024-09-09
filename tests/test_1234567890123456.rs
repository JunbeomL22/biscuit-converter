#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u64("1234567890123456"),  1234567890123456);
        assert_eq!(biscuit_converter.to_u128("1234567890123456"), 1234567890123456);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i64("1234567890123456"), 1234567890123456);
        assert_eq!(biscuit_converter.to_i128("1234567890123456"), 1234567890123456);
        assert_eq!(biscuit_converter.to_i64("-1234567890123456"), -1234567890123456);
        assert_eq!(biscuit_converter.to_i128("-1234567890123456"), -1234567890123456);
    }
}