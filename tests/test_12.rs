#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u16("12"), 12);
        assert_eq!(biscuit_converter.to_u32("12"), 12);
        assert_eq!(biscuit_converter.to_u64("12"), 12);
        assert_eq!(biscuit_converter.to_u128("12"), 12);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i16("12"), 12);
        assert_eq!(biscuit_converter.to_i32("12"), 12);
        assert_eq!(biscuit_converter.to_i64("12"), 12);
        assert_eq!(biscuit_converter.to_i128("12"), 12);
        assert_eq!(biscuit_converter.to_i16("-12"), -12);
        assert_eq!(biscuit_converter.to_i32("-12"), -12);
        assert_eq!(biscuit_converter.to_i64("-12"), -12);
        assert_eq!(biscuit_converter.to_i128("-12"), -12);
    }

}