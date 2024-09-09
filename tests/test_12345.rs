#[cfg(test)]
mod tests {
    use biscuit_parser::BiscuitParser;
    use approx_eq::assert_approx_eq;
    
    #[test]
    fn test_to_unsigned() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_u32("12345"), 12345);
        assert_eq!(biscuit_parser.to_u64("12345"), 12345);
        assert_eq!(biscuit_parser.to_u128("12345"), 12345);
    }

    #[test]
    fn test_to_signed() {
        let biscuit_parser = BiscuitParser::default();
        assert_eq!(biscuit_parser.to_i32("12345"), 12345);
        assert_eq!(biscuit_parser.to_i64("12345"), 12345);
        assert_eq!(biscuit_parser.to_i128("12345"), 12345);
        //
        assert_eq!(biscuit_parser.to_i32("-12345"), -12345);
        assert_eq!(biscuit_parser.to_i64("-12345"), -12345);
        assert_eq!(biscuit_parser.to_i128("-12345"), -12345);
    }

    #[test]
    fn test_f32() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f32("12345.") as f64, 12345.0);
        assert_approx_eq!(biscuit_parser.to_f32("1234.5") as f64, 1234.5);
        assert_approx_eq!(biscuit_parser.to_f32("123.45") as f64, 123.45);
        assert_approx_eq!(biscuit_parser.to_f32("12.345") as f64, 12.345);
        assert_approx_eq!(biscuit_parser.to_f32("1.2345") as f64, 1.2345);
        assert_approx_eq!(biscuit_parser.to_f32(".12345") as f64, 0.12345);
    }

    #[test]
    fn test_f64() {
        let biscuit_parser = BiscuitParser::default();
        assert_approx_eq!(biscuit_parser.to_f64("12345."), 12345.0);
        assert_approx_eq!(biscuit_parser.to_f64("1234.5"), 1234.5);
        assert_approx_eq!(biscuit_parser.to_f64("123.45"), 123.45);
        assert_approx_eq!(biscuit_parser.to_f64("12.345"), 12.345);
        assert_approx_eq!(biscuit_parser.to_f64("1.2345"), 1.2345);
        assert_approx_eq!(biscuit_parser.to_f64(".12345"), 0.12345);
    }
}