#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u64("12345678901"), 12345678901);
        assert_eq!(biscuit_parser.to_u128("12345678901"), 12345678901);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i64("12345678901"), 12345678901);
        assert_eq!(biscuit_parser.to_i128("12345678901"), 12345678901);
        //
        assert_eq!(biscuit_parser.to_i64("-12345678901"), -12345678901);
        assert_eq!(biscuit_parser.to_i128("-12345678901"), -12345678901);
    }

    #[test]
    fn test_f32() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("12345678901.") as f64, 12345678901.0);
        assert_approx_eq!(biscuit_parser.to_f32("1234567890.1") as f64, 1234567890.1);
        assert_approx_eq!(biscuit_parser.to_f32("123456789.01") as f64, 123456789.01);
        assert_approx_eq!(biscuit_parser.to_f32("12345678.901") as f64, 12345678.901);
        assert_approx_eq!(biscuit_parser.to_f32("1234567.8901") as f64, 1234567.8901);
        assert_approx_eq!(biscuit_parser.to_f32("123456.78901") as f64, 123456.78901);
        assert_approx_eq!(biscuit_parser.to_f32("12345.678901") as f64, 12345.678901);
        assert_approx_eq!(biscuit_parser.to_f32("1234.5678901") as f64, 1234.5678901);
    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("12345678901."), 12345678901.0);
        assert_approx_eq!(biscuit_parser.to_f64("1234567890.1"), 1234567890.1);
        assert_approx_eq!(biscuit_parser.to_f64("123456789.01"), 123456789.01);
        assert_approx_eq!(biscuit_parser.to_f64("12345678.901"), 12345678.901);
        assert_approx_eq!(biscuit_parser.to_f64("1234567.8901"), 1234567.8901);
        assert_approx_eq!(biscuit_parser.to_f64("123456.78901"), 123456.78901);
        assert_approx_eq!(biscuit_parser.to_f64("12345.678901"), 12345.678901);
        assert_approx_eq!(biscuit_parser.to_f64("1234.5678901"), 1234.5678901);
        assert_approx_eq!(biscuit_parser.to_f64("123.45678901"), 123.45678901);
        assert_approx_eq!(biscuit_parser.to_f64("12.345678901"), 12.345678901);
        assert_approx_eq!(biscuit_parser.to_f64("1.2345678901"), 1.2345678901);
        assert_approx_eq!(biscuit_parser.to_f64(".12345678901"), 0.12345678901);
    }
}