#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u32("123456789"), 123456789);
        assert_eq!(biscuit_parser.to_u64("123456789"), 123456789);
        assert_eq!(biscuit_parser.to_u128("123456789"), 123456789);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i32("123456789"), 123456789);
        assert_eq!(biscuit_parser.to_i64("123456789"), 123456789);
        assert_eq!(biscuit_parser.to_i128("123456789"), 123456789);
        //
        assert_eq!(biscuit_parser.to_i32("-123456789"), -123456789);
        assert_eq!(biscuit_parser.to_i64("-123456789"), -123456789);
        assert_eq!(biscuit_parser.to_i128("-123456789"), -123456789);
    }

    #[test]
    fn test_f32() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("123456789.") as f64, 123456789.0);
        assert_approx_eq!(biscuit_parser.to_f32("12345678.9") as f64, 12345678.9);
        assert_approx_eq!(biscuit_parser.to_f32("1234567.89") as f64, 1234567.89);
        assert_approx_eq!(biscuit_parser.to_f32("123456.789") as f64, 123456.789);
        assert_approx_eq!(biscuit_parser.to_f32("12345.6789") as f64, 12345.6789);
        assert_approx_eq!(biscuit_parser.to_f32("1234.56789") as f64, 1234.56789);
        assert_approx_eq!(biscuit_parser.to_f32("123.456789") as f64, 123.456789);
        assert_approx_eq!(biscuit_parser.to_f32("12.3456789") as f64, 12.3456789);
        assert_approx_eq!(biscuit_parser.to_f32("1.23456789") as f64, 1.23456789);
    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("123456789."), 123456789.0);
        assert_approx_eq!(biscuit_parser.to_f64("12345678.9"), 12345678.9);
        assert_approx_eq!(biscuit_parser.to_f64("1234567.89"), 1234567.89);
        assert_approx_eq!(biscuit_parser.to_f64("123456.789"), 123456.789);
        assert_approx_eq!(biscuit_parser.to_f64("12345.6789"), 12345.6789);
        assert_approx_eq!(biscuit_parser.to_f64("1234.56789"), 1234.56789);
        assert_approx_eq!(biscuit_parser.to_f64("123.456789"), 123.456789);
        assert_approx_eq!(biscuit_parser.to_f64("12.3456789"), 12.3456789);
        assert_approx_eq!(biscuit_parser.to_f64("1.23456789"), 1.23456789);
        assert_approx_eq!(biscuit_parser.to_f64(".123456789"), 0.123456789);
    }
}