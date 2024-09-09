#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u16("123"), 123);
        assert_eq!(biscuit_converter.to_u32("123"), 123);
        assert_eq!(biscuit_converter.to_u64("123"), 123);
        assert_eq!(biscuit_converter.to_u128("123"), 123);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i16("123"), 123);
        assert_eq!(biscuit_converter.to_i32("123"), 123);
        assert_eq!(biscuit_converter.to_i64("123"), 123);
        assert_eq!(biscuit_converter.to_i128("123"), 123);
        assert_eq!(biscuit_converter.to_i16("-123"), -123);
        assert_eq!(biscuit_converter.to_i32("-123"), -123);
        assert_eq!(biscuit_converter.to_i64("-123"), -123);
        assert_eq!(biscuit_converter.to_i128("-123"), -123);
    }

    #[test]
    fn test_f32() {
        let biscuit_converter = BiscuitConverter::default();
        assert_approx_eq!(biscuit_converter.to_f32("123.") as f64, 123.0);
        assert_approx_eq!(biscuit_converter.to_f32("12.3") as f64, 12.3);
        assert_approx_eq!(biscuit_converter.to_f32("1.23") as f64, 1.23);
        assert_approx_eq!(biscuit_converter.to_f32(".123") as f64, 0.123);
    }

    #[test]
    fn test_f64() {
        let biscuit_converter = BiscuitConverter::default();
        assert_approx_eq!(biscuit_converter.to_f64("123."), 123.0);
        assert_approx_eq!(biscuit_converter.to_f64("12.3"), 12.3);
        assert_approx_eq!(biscuit_converter.to_f64("1.23"), 1.23);
        assert_approx_eq!(biscuit_converter.to_f64(".123"), 0.123);
    }
}