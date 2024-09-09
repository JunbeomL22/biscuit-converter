#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u64("1234567890123"), 1234567890123);
        assert_eq!(biscuit_parser.to_u128("1234567890123"), 1234567890123);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i64("1234567890123"), 1234567890123);
        assert_eq!(biscuit_parser.to_i128("1234567890123"), 1234567890123);
        //
        assert_eq!(biscuit_parser.to_i64("-1234567890123"), -1234567890123);
        assert_eq!(biscuit_parser.to_i128("-1234567890123"), -1234567890123);
    }

    #[test]
    fn test_f32() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("1234567890123.") as f64, 1234567890123.0);
        assert_approx_eq!(biscuit_parser.to_f32("123456789012.3") as f64, 123456789012.3);
        assert_approx_eq!(biscuit_parser.to_f32("12345678901.23") as f64, 12345678901.23);
        assert_approx_eq!(biscuit_parser.to_f32("1234567890.123") as f64, 1234567890.123);
        assert_approx_eq!(biscuit_parser.to_f32("123456789.0123") as f64, 123456789.0123);
        assert_approx_eq!(biscuit_parser.to_f32("12345678.90123") as f64, 12345678.90123);
        assert_approx_eq!(biscuit_parser.to_f32("1234567.890123") as f64, 1234567.890123);
        assert_approx_eq!(biscuit_parser.to_f32("123456.7890123") as f64, 123456.7890123);

    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("1234567890123."), 1234567890123.0);
        assert_approx_eq!(biscuit_parser.to_f64("123456789012.3"), 123456789012.3);
        assert_approx_eq!(biscuit_parser.to_f64("12345678901.23"), 12345678901.23);
        assert_approx_eq!(biscuit_parser.to_f64("1234567890.123"), 1234567890.123);
        assert_approx_eq!(biscuit_parser.to_f64("123456789.0123"), 123456789.0123);
        assert_approx_eq!(biscuit_parser.to_f64("12345678.90123"), 12345678.90123);
        assert_approx_eq!(biscuit_parser.to_f64("1234567.890123"), 1234567.890123);
        assert_approx_eq!(biscuit_parser.to_f64("123456.7890123"), 123456.7890123);
        assert_approx_eq!(biscuit_parser.to_f64("12345.67890123"), 12345.67890123);
        assert_approx_eq!(biscuit_parser.to_f64("1234.567890123"), 1234.567890123);
        assert_approx_eq!(biscuit_parser.to_f64("123.4567890123"), 123.4567890123);
        assert_approx_eq!(biscuit_parser.to_f64("12.34567890123"), 12.34567890123);
        assert_approx_eq!(biscuit_parser.to_f64("1.234567890123"), 1.234567890123);
        assert_approx_eq!(biscuit_parser.to_f64(".1234567890123"), 0.1234567890123);

    }
}