#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u32("123456789"), 123456789);
        assert_eq!(biscuit_converter.to_u64("123456789"), 123456789);
        assert_eq!(biscuit_converter.to_u128("123456789"), 123456789);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i32("123456789"), 123456789);
        assert_eq!(biscuit_converter.to_i64("123456789"), 123456789);
        assert_eq!(biscuit_converter.to_i128("123456789"), 123456789);
        //
        assert_eq!(biscuit_converter.to_i32("-123456789"), -123456789);
        assert_eq!(biscuit_converter.to_i64("-123456789"), -123456789);
        assert_eq!(biscuit_converter.to_i128("-123456789"), -123456789);
    }


}