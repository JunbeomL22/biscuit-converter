#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_u64("12345678901234567"),  12345678901234567);
        assert_eq!(biscuit_converter.to_u128("12345678901234567"), 12345678901234567);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_converter = BiscuitConverter::default();
        assert_eq!(biscuit_converter.to_i64("12345678901234567"), 12345678901234567);
        assert_eq!(biscuit_converter.to_i128("12345678901234567"), 12345678901234567);
        assert_eq!(biscuit_converter.to_i64("-12345678901234567"), -12345678901234567);
        assert_eq!(biscuit_converter.to_i128("-12345678901234567"), -12345678901234567);
    }

    #[test]
    fn test_f64() {
        let biscuit_converter = BiscuitConverter::default();
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234567."), 12345678901234567.);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123456.7"), 1234567890123456.7);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345.67"), 123456789012345.67);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234.567"), 12345678901234.567);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123.4567"), 1234567890123.4567);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012.34567"), 123456789012.34567);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901.234567"), 12345678901.234567);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890.1234567"), 1234567890.1234567);
        assert_approx_eq!(biscuit_converter.to_f64("123456789.01234567"), 123456789.01234567);
        assert_approx_eq!(biscuit_converter.to_f64("12345678.901234567"), 12345678.901234567);
        assert_approx_eq!(biscuit_converter.to_f64("1234567.8901234567"), 1234567.8901234567);
        assert_approx_eq!(biscuit_converter.to_f64("123456.78901234567"), 123456.78901234567);
        assert_approx_eq!(biscuit_converter.to_f64("12345.678901234567"), 12345.678901234567);
        assert_approx_eq!(biscuit_converter.to_f64("1234.5678901234567"), 1234.5678901234567);
        assert_approx_eq!(biscuit_converter.to_f64("123.45678901234567"), 123.45678901234567);
        assert_approx_eq!(biscuit_converter.to_f64("12.345678901234567"), 12.345678901234567);
        assert_approx_eq!(biscuit_converter.to_f64("1.2345678901234567"), 1.2345678901234567);
        assert_approx_eq!(biscuit_converter.to_f64(".12345678901234567"), 0.12345678901234567);
    }
}