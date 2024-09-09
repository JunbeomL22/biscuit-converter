#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use approx_eq::assert_approx_eq;
    
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

    #[test]
    fn test_f32() {
        let biscuit_converter = BiscuitConverter::default();
        assert_approx_eq!(biscuit_converter.to_f32("12.") as f64, 12.0);
        assert_approx_eq!(biscuit_converter.to_f32("1.2") as f64, 1.2);
        assert_approx_eq!(biscuit_converter.to_f32("0.12") as f64, 0.12);
    }

    #[test]
    fn test_f64() {
        let biscuit_converter = BiscuitConverter::default();
        assert_approx_eq!(biscuit_converter.to_f64("12.") as f64, 12.0);
        assert_approx_eq!(biscuit_converter.to_f64("1.2") as f64, 1.2);
        assert_approx_eq!(biscuit_converter.to_f64("0.12") as f64, 0.12);
    }
}