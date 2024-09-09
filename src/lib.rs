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
//! 
//! let biscuit_converter = BiscuitConverter {};
//! 
//!
//! // Parsing examples
//! let u64_result: u64 = biscuit_converter.to_u64("123");
//! assert_eq!(u64_result, 123);
//! 
//! let i64_result: i64 = biscuit_converter.to_i64("-123");
//! assert_eq!(i64_result, -123);
//! 
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
//! ### Observations:
//!

//! - `biscuit-converter` consistently outperforms both the standard library and `atoi` for parsing integers.
//! - The performance advantage is particularly significant for smaller numbers.
//! - For larger integers (both signed and unsigned), `biscuit-converter` can be 2-3 times faster than the standard library.
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
//! 2. Little-endian representation is assumed
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
pub mod integer;

/// Parser for decimal notation
/// It can not parse scientific notation
#[derive(Debug, Clone, Copy, Default)]
pub struct BiscuitConverter {}

impl BiscuitConverter {
    pub fn new() -> Self {
        BiscuitConverter {}
    }
}