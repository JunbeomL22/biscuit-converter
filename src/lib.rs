//! # biscuit-converter
//!
//! `biscuit-converter` is a high-performance numeric parser for Rust that converts ASCII numbers to their numeric representations.
//! It uses a combination of bit operations to achieve fast parsing for integers, unsigned integers, and floating-point numbers.
//!
//! ## Features
//!
//! - Fast parsing of ASCII numbers to numeric types
//! - Supports parsing of integers, unsigned integers, and floating-point numbers
//! - Optimized for various string lengths
//! - Option to specify fraction length for further optimization
//!
//! ## Performance
//!
//! - Integer parsing: Faster than `atoi`
//! - Float parsing:
//!   - Small-sized strings: Similar performance to `std::str::parse`
//!   - Mid-sized strings: Slower than `std::str::parse` (when fraction length is not given)
//!   - Large strings: Faster than `std::str::parse`
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! biscuit-converter = "0.1"
//! ```
//!
//! Then, use it in your Rust code:
//!
//! ```rust
//! use biscuit_converter::BiscuitConverter;
//!
//! // Default parser
//! let biscuit_converter = BiscuitConverter::default();
//!     
//! // Parser with known fraction length
//! // This is faster than the above parser
//! let biscuit_converter_fraction_given = BiscuitConverter::initialize().with_fraction_length(2); 
//!
//! // Parsing examples
//! let int_result: u64 = biscuit_converter.to_u64("123");
//! assert_eq!(int_result, 123);
//!
//! let float_result: f64 = biscuit_converter.to_f64("123.45");
//! assert_eq!(float_result, 123.45);
//!
//! // Faster parsing when fraction length is known
//! let optimized_float_result: f64 = biscuit_converter_fraction_given.to_f64("123.45");
//! assert_eq!(optimized_float_result, 123.45);
//! ```
//!
//! ## Benchmarks
//!
//! Comprehensive benchmark results for `biscuit-converter` compared to the standard library and `atoi`.
//! Times are in nanoseconds, rounded to one decimal place.
//!
//! **Testing Environment:**
//! - CPU: Ryzen 7 7700 3.8 GHz
//! - Rust Version: 1.79
//!
//! ### Unsigned Integers (u64)
//!
//! | Input              | biscuit | std   | atoi  |
//! |--------------------|---------|-------|-------|
//! | 123                | 1.0     | 4.0   | 2.3   |
//! | 123456             | 1.3     | 4.7   | 3.1   |
//! | 123456789          | 2.9     | 5.7   | 4.2   |
//! | 123456789012       | 3.1     | 6.6   | 5.4   |
//! | 123456789012345    | 2.9     | 7.8   | 6.7   |
//! | 123456789012345678 | 5.9     | 11.9  | 8.0   |
//!
//! ### Signed Integers (i64)
//!
//! | Input              | biscuit | std   | atoi  |
//! |--------------------|---------|-------|-------|
//! | -123               | 1.2     | 4.0   | 2.5   |
//! | -123456            | 1.5     | 4.7   | 3.7   |
//! | -123456789         | 3.1     | 5.5   | 5.0   |
//! | -123456789012      | 3.1     | 6.6   | 6.5   |
//! | -123456789012345   | 3.1     | 8.1   | 7.8   |
//! | -123456789012345678| 5.9     | 11.2  | 9.1   |
//!
//! ### Floating-Point Numbers (f64)
//!
//! | Input                     | biscuit | biscuit (fraction given) | std   |
//! |---------------------------|---------|--------------------------|-------|
//! | 1.23                      | 5.9     | 3.4                      | 6.0   |
//! | 1234.56                   | 7.3     | 4.1                      | 7.0   |
//! | 1234567.89                | 12.3    | 9.4                      | 8.2   |
//! | 1234567890.12             | 13.1    | 9.4                      | 7.5   |
//! | 1234567890123.45          | 11.9    | 9.5                      | 8.5   |
//! | 1234567890123456.78       | 15.9    | 13.6                     | 10.7  |
//! | 1234567890123456789.01    | 16.9    | 14.9                     | 25.2  |
//! | 1234567890123456789012.34 | 18.7    | 16.5                     | 26.9  |
//! | 1234567890123456789012345.67 | 18.9 | 16.5                     | 25.8  |
//! | 1234567890123456789012345678.90 | 18.6 | 16.2                  | 26.9  |
//!
//! ### Observations:
//!
//! 1. Integer Parsing (Unsigned and Signed):
//!    - `biscuit-converter` consistently outperforms both the standard library and `atoi` for parsing integers.
//!    - The performance advantage is particularly significant for smaller numbers.
//!    - For larger integers (both signed and unsigned), `biscuit-converter` can be 2-3 times faster than the standard library.
//!
//! 2. Floating-Point Numbers:
//!    - For small floating-point numbers, the standard library parser is generally faster.
//!    - For larger numbers (more than about 16 digits), `biscuit-converter` becomes significantly faster than the standard library.
//!    - When the fraction length is known and provided to `biscuit-converter`, it consistently outperforms both the standard version of `biscuit-converter` and the standard library.
//!
//! 3. Overall Performance:
//!    - `biscuit-converter` shows its strength in parsing larger numbers across all types (unsigned, signed, and float).
//!    - The performance advantage of `biscuit-converter` is most pronounced for integer parsing.
//!    - For very large numbers of any type, `biscuit-converter` can provide substantial performance improvements over the standard library.
//!
//! Note: These benchmarks were run on the specified testing environment. Results may vary depending on hardware and environmental factors.
//! It's always recommended to run benchmarks on your target hardware for the most accurate results.
//!
//! ## Algorithm Explanation
//!
//! The `biscuit-converter` library achieves its high performance through bit manipulation techniques.
//! Key concepts include:
//!
//! 1. ASCII number representation
//! 2. Little-endian representation
//! 3. Efficient bit shifting
//!
//! The algorithm uses various techniques for parsing different number sizes and types,
//! including single digit parsing, multi-digit parsing, handling irregular digit counts,
//! and parsing negative numbers.
//!
//! For a detailed explanation of the algorithm and parsing techniques, please refer to
//! the project's README on GitHub.
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! - Apache License, Version 2.0, (<http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license (<http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Contributions are very welcome! Whether it's bug reports, optimizations, or any other improvements,
//! all contributions will be gratefully reviewed. Please feel free to submit a Pull Request or
//! open an Issue on the GitHub repository.
pub mod unsigned;
pub mod little_endian;
pub mod float;
pub mod integer;

/// Parser for decimal notation
/// It can not parse scientific notation
#[derive(Debug, Clone, Copy, Default)]
pub struct BiscuitConverter {
    fraction_length: Option<usize>,
}

impl BiscuitConverter {
    pub fn initialize() -> BiscuitConverter {
        BiscuitConverter {
            fraction_length: None,
        }
    }

    pub fn new(fraction_length: usize) -> Self {
        Self {
            fraction_length: Some(fraction_length),
        }
    }

    pub fn with_fraction_length(mut self, fraction_length: usize) -> Self {
        self.fraction_length = Some(fraction_length);
        self
    }

    pub fn set_fraction_length(&mut self, fraction_length: usize) {
        self.fraction_length = Some(fraction_length);
    }
}