#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u64("1234567890123456"),  1234567890123456);
        assert_eq!(biscuit_parser.to_u128("1234567890123456"), 1234567890123456);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i64("1234567890123456"), 1234567890123456);
        assert_eq!(biscuit_parser.to_i128("1234567890123456"), 1234567890123456);
        assert_eq!(biscuit_parser.to_i64("-1234567890123456"), -1234567890123456);
        assert_eq!(biscuit_parser.to_i128("-1234567890123456"), -1234567890123456);
    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("1234567890123456."), 1234567890123456.);
        assert_approx_eq!(biscuit_parser.to_f64("123456789012345.6"), 123456789012345.6);
        assert_approx_eq!(biscuit_parser.to_f64("12345678901234.56"), 12345678901234.56);
        assert_approx_eq!(biscuit_parser.to_f64("1234567890123.456"), 1234567890123.456);
        assert_approx_eq!(biscuit_parser.to_f64("123456789012.3456"), 123456789012.3456);
        assert_approx_eq!(biscuit_parser.to_f64("12345678901.23456"), 12345678901.23456);
        assert_approx_eq!(biscuit_parser.to_f64("1234567890.123456"), 1234567890.123456);
        assert_approx_eq!(biscuit_parser.to_f64("123456789.0123456"), 123456789.0123456);
        assert_approx_eq!(biscuit_parser.to_f64("12345678.90123456"), 12345678.90123456);
        assert_approx_eq!(biscuit_parser.to_f64("1234567.890123456"), 1234567.890123456);
        assert_approx_eq!(biscuit_parser.to_f64("123456.7890123456"), 123456.7890123456);
        assert_approx_eq!(biscuit_parser.to_f64("12345.67890123456"), 12345.67890123456);
        assert_approx_eq!(biscuit_parser.to_f64("1234.567890123456"), 1234.567890123456);
        assert_approx_eq!(biscuit_parser.to_f64("123.4567890123456"), 123.4567890123456);
        assert_approx_eq!(biscuit_parser.to_f64("12.34567890123456"), 12.34567890123456);
        assert_approx_eq!(biscuit_parser.to_f64("1.234567890123456"), 1.234567890123456);
        assert_approx_eq!(biscuit_parser.to_f64(".1234567890123456"), 0.1234567890123456);
    }
}