#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u32("1234567"), 1234567);
        assert_eq!(biscuit_parser.to_u64("1234567"), 1234567);
        assert_eq!(biscuit_parser.to_u128("1234567"), 1234567);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i32("1234567"), 1234567);
        assert_eq!(biscuit_parser.to_i64("1234567"), 1234567);
        assert_eq!(biscuit_parser.to_i128("1234567"), 1234567);
        //
        assert_eq!(biscuit_parser.to_i32("-1234567"), -1234567);
        assert_eq!(biscuit_parser.to_i64("-1234567"), -1234567);
        assert_eq!(biscuit_parser.to_i128("-1234567"), -1234567);
    }

    #[test]
    fn test_f32() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("1234567.") as f64, 1234567.0);
        assert_approx_eq!(biscuit_parser.to_f32("123456.7") as f64, 123456.7);
        assert_approx_eq!(biscuit_parser.to_f32("12345.67") as f64, 12345.67);
        assert_approx_eq!(biscuit_parser.to_f32("1234.567") as f64, 1234.567);
        assert_approx_eq!(biscuit_parser.to_f32("123.4567") as f64, 123.4567);
        assert_approx_eq!(biscuit_parser.to_f32("12.34567") as f64, 12.34567);
        assert_approx_eq!(biscuit_parser.to_f32("1.234567") as f64, 1.234567);
        assert_approx_eq!(biscuit_parser.to_f32(".1234567") as f64, 0.1234567);
    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("1234567."), 1234567.0);
        assert_approx_eq!(biscuit_parser.to_f64("123456.7"), 123456.7);
        assert_approx_eq!(biscuit_parser.to_f64("12345.67"), 12345.67);
        assert_approx_eq!(biscuit_parser.to_f64("1234.567"), 1234.567);
        assert_approx_eq!(biscuit_parser.to_f64("123.4567"), 123.4567);
        assert_approx_eq!(biscuit_parser.to_f64("12.34567"), 12.34567);
        assert_approx_eq!(biscuit_parser.to_f64("1.234567"), 1.234567);
        assert_approx_eq!(biscuit_parser.to_f64(".1234567"), 0.1234567);
    }
}