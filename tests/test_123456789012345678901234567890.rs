#[cfg(test)]
mod tests {
    use biscuit_converter::BiscuitConverter;
    use approx_eq::assert_approx_eq;
    
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

    #[test]
    fn test_f64() {
        let biscuit_converter = BiscuitConverter::default();
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345678901234567890."), 123456789012345678901234567890.0);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234567890123456789.0"), 12345678901234567890123456789.0);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123456789012345678.90"), 1234567890123456789012345678.90);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345678901234567.890"), 123456789012345678901234567.890);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234567890123456.7890"), 12345678901234567890123456.7890);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123456789012345.67890"), 1234567890123456789012345.67890);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345678901234.567890"), 123456789012345678901234.567890);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234567890123.4567890"), 12345678901234567890123.4567890);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123456789012.34567890"), 1234567890123456789012.34567890);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345678901.234567890"), 123456789012345678901.234567890);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234567890.1234567890"), 12345678901234567890.1234567890);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123456789.01234567890"), 1234567890123456789.01234567890);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345678.901234567890"), 123456789012345678.901234567890);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234567.8901234567890"), 12345678901234567.8901234567890);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123456.78901234567890"), 1234567890123456.78901234567890);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012345.678901234567890"), 123456789012345.678901234567890);
        assert_approx_eq!(biscuit_converter.to_f64("12345678901234.5678901234567890"), 12345678901234.5678901234567890);
        assert_approx_eq!(biscuit_converter.to_f64("1234567890123.45678901234567890"), 1234567890123.45678901234567890);
        assert_approx_eq!(biscuit_converter.to_f64("123456789012.345678901234567890"), 123456789012.345678901234567890);
    }
}