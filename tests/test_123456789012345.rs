#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u64("123456789012345"), 123456789012345);
        assert_eq!(biscuit_parser.to_u128("123456789012345"), 123456789012345);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i64("123456789012345"), 123456789012345);
        assert_eq!(biscuit_parser.to_i128("123456789012345"), 123456789012345);
        //
        assert_eq!(biscuit_parser.to_i64("-123456789012345"), -123456789012345);
        assert_eq!(biscuit_parser.to_i128("-123456789012345"), -123456789012345);
    }

    #[test]
    fn test_f32() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("123456789012345.") as f64, 123456789012345.0);
        assert_approx_eq!(biscuit_parser.to_f32("12345678901234.5") as f64, 12345678901234.5);
        assert_approx_eq!(biscuit_parser.to_f32("1234567890123.45") as f64, 1234567890123.45);
        assert_approx_eq!(biscuit_parser.to_f32("123456789012.345") as f64, 123456789012.345);
        assert_approx_eq!(biscuit_parser.to_f32("12345678901.2345") as f64, 12345678901.2345);
        assert_approx_eq!(biscuit_parser.to_f32("1234567890.12345") as f64, 1234567890.12345);
        assert_approx_eq!(biscuit_parser.to_f32("123456789.012345") as f64, 123456789.012345);
        assert_approx_eq!(biscuit_parser.to_f32("12345678.9012345") as f64, 12345678.9012345);
        assert_approx_eq!(biscuit_parser.to_f32("1234567.89012345") as f64, 1234567.89012345);
    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("123456789012345."), 123456789012345.0);
        assert_approx_eq!(biscuit_parser.to_f64("12345678901234.5"), 12345678901234.5);
        assert_approx_eq!(biscuit_parser.to_f64("1234567890123.45"), 1234567890123.45);
        assert_approx_eq!(biscuit_parser.to_f64("123456789012.345"), 123456789012.345);
        assert_approx_eq!(biscuit_parser.to_f64("12345678901.2345"), 12345678901.2345);
        assert_approx_eq!(biscuit_parser.to_f64("1234567890.12345"), 1234567890.12345);
        assert_approx_eq!(biscuit_parser.to_f64("123456789.012345"), 123456789.012345);
        assert_approx_eq!(biscuit_parser.to_f64("12345678.9012345"), 12345678.9012345);
        assert_approx_eq!(biscuit_parser.to_f64("1234567.89012345"), 1234567.89012345);
        assert_approx_eq!(biscuit_parser.to_f64("123456.789012345"), 123456.789012345);
        assert_approx_eq!(biscuit_parser.to_f64("12345.6789012345"), 12345.6789012345);
        assert_approx_eq!(biscuit_parser.to_f64("1234.56789012345"), 1234.56789012345);
        assert_approx_eq!(biscuit_parser.to_f64("123.456789012345"), 123.456789012345);
        assert_approx_eq!(biscuit_parser.to_f64("12.3456789012345"), 12.3456789012345);
        assert_approx_eq!(biscuit_parser.to_f64("1.23456789012345"), 1.23456789012345);
        assert_approx_eq!(biscuit_parser.to_f64(".123456789012345"), 0.123456789012345);
    }
}