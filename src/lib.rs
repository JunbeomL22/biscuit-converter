//! # biscuit-converter
//!
//! biscuit-converter is a integer parser for decimal formatted ASCII strings.
//! ## Performance
//!
//! Test machine: Ryzen 7 7700 3.8Ghz, rust 1.79
//!
//! ### `i128` Performance Comparison (in nanoseconds)
//! 
//! | Input                             | biscuit | std    | atoi   |
//! |-----------------------------------|---------|--------|--------|
//! | -1                                | 3.2 ns  | 7.1 ns | 3.4 ns |
//! | -12                               | 3.1 ns  | 8.2 ns | 3.7 ns |
//! | -123                              | 3.2 ns  | 7.5 ns | 4.9 ns |
//! | -1234                             | 3.2 ns  | 7.9 ns | 4.9 ns |
//! | -12345                            | 3.2 ns  | 9.4 ns | 5.7 ns |
//! | -123456                           | 3.4 ns  | 8.9 ns | 6.3 ns |
//! | -1234567                          | 3.6 ns  | 9.5 ns | 7.2 ns |
//! | -12345678                         | 3.2 ns  | 10.6 ns| 8.1 ns |
//! | -123456789                        | 3.4 ns  | 10.2 ns| 9.3 ns |
//! | -1234567890                       | 3.6 ns  | 9.8 ns | 9.6 ns |
//! | -12345678901                      | 3.9 ns  | 11.4 ns| 10.9 ns|
//! | -123456789012                     | 3.8 ns  | 11.6 ns| 12.0 ns|
//! | -1234567890123                    | 4.1 ns  | 13.4 ns| 12.1 ns|
//! | -12345678901234                   | 4.1 ns  | 13.5 ns| 13.5 ns|
//! | -123456789012345                  | 4.6 ns  | 15.6 ns| 14.1 ns|
//! | -1234567890123456                 | 4.1 ns  | 16.7 ns| 15.0 ns|
//! | -12345678901234567                | 4.4 ns  | 17.2 ns| 16.0 ns|
//! | -123456789012345678               | 4.5 ns  | 17.6 ns| 17.3 ns|
//! | -1234567890123456789              | 4.8 ns  | 19.2 ns| 18.4 ns|
//! | -12345678901234567890             | 4.7 ns  | 20.0 ns| 18.9 ns|
//! | -123456789012345678901            | 5.0 ns  | 20.2 ns| 20.0 ns|
//! | -1234567890123456789012           | 5.1 ns  | 21.0 ns| 20.6 ns|
//! | -12345678901234567890123          | 5.3 ns  | 22.6 ns| 21.3 ns|
//! | -123456789012345678901234         | 4.9 ns  | 22.9 ns| 22.4 ns|
//! | -1234567890123456789012345        | 5.1 ns  | 23.6 ns| 23.3 ns|
//! | -12345678901234567890123456       | 5.4 ns  | 25.4 ns| 24.1 ns|
//! | -123456789012345678901234567      | 5.7 ns  | 25.9 ns| 25.1 ns|
//! | -1234567890123456789012345678     | 5.4 ns  | 26.5 ns| 25.9 ns|
//! | -12345678901234567890123456789    | 5.8 ns  | 28.3 ns| 27.0 ns|
//! | -123456789012345678901234567890   | 6.1 ns  | 28.7 ns| 27.9 ns|
//! | -1234567890123456789012345678901  | 6.5 ns  | 28.8 ns| 28.9 ns|
//! | -12345678901234567890123456789012 | 5.9 ns  | 47.5 ns| 30.6 ns|
//! | -123456789012345678901234567890123| 6.7 ns | 53.9 ns| 30.9 ns|
//! | -1234567890123456789012345678901234| 7.1 ns| 55.9 ns| 31.6 ns|
//! | -12345678901234567890123456789012345| 7.0 ns| 53.2 ns| 31.5 ns|
//! | -123456789012345678901234567890123456| 6.9 ns| 54.1 ns| 31.7 ns|
//! | -1234567890123456789012345678901234567| 7.8 ns| 58.4 ns| 32.5 ns|
//! | -12345678901234567890123456789012345678| 8.2 ns| 59.7 ns| 34.3 ns|
//!
//! ### `i64` Performance Comparison (in nanoseconds)
//!
//! | Input                    | biscuit | std    | atoi   |
//! |--------------------------|---------|--------|--------|
//! | -1                       | 1.7 ns  | 3.4 ns | 1.7 ns |
//! | -12                      | 1.9 ns  | 3.5 ns | 2.1 ns |
//! | -123                     | 2.0 ns  | 4.0 ns | 2.4 ns |
//! | -1234                    | 2.0 ns  | 4.1 ns | 2.8 ns |
//! | -12345                   | 2.2 ns  | 4.3 ns | 3.2 ns |
//! | -123456                  | 2.2 ns  | 4.7 ns | 3.8 ns |
//! | -1234567                 | 2.4 ns  | 4.9 ns | 4.1 ns |
//! | -12345678                | 2.2 ns  | 5.1 ns | 4.5 ns |
//! | -123456789               | 2.4 ns  | 5.5 ns | 5.0 ns |
//! | -1234567890              | 2.6 ns  | 5.6 ns | 5.4 ns |
//! | -123456789012            | 2.8 ns  | 6.5 ns | 6.4 ns |
//! | -1234567890123           | 3.0 ns  | 6.9 ns | 6.9 ns |
//! | -12345678901234          | 3.2 ns  | 7.4 ns | 7.4 ns |
//! | -123456789012345         | 3.4 ns  | 7.9 ns | 7.8 ns |
//! | -1234567890123456        | 3.2 ns  | 9.7 ns | 8.2 ns |
//! | -12345678901234567       | 3.2 ns  | 10.4 ns| 9.0 ns |
//! | -123456789012345678      | 3.5 ns  | 11.0 ns| 9.1 ns |
//! | -1234567890123456789     | 3.9 ns  | 11.5 ns| 10.0 ns|
//!
//! ## Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! biscuit-converter = "0.2"
//! ```
//! Then, use it in your Rust code:
//! ```rust
//! use biscuit_converter::BiscuitConverter;
//! 
//! let biscuit_converter = BiscuitConverter::default();
//! // Parsing examples
//! let u64_result: Option<u64> = biscuit_converter.to_u64("123");
//! assert_eq!(u64_result, Ok(123));
//! let i64_result: Option<i64> = biscuit_converter.to_i64("-123");
//! assert_eq!(i64_result, Ok(-123));
//! 
//! ```
//! 
//! # License
//! This project is licensed under either of
//!
//! - Apache License, Version 2.0, <http://www.apache.org/licenses/LICENSE-2.0>
//! - MIT license <http://opensource.org/licenses/MIT>
//!
//! at your option.
//!
//! # Contributions
//!
//! We welcome all kinds of contributions! Whether it's bug reports, feature requests, or code contributions, 
//! your input is valuable and appreciated. Feel free to open issues or submit pull requests on our GitHub repository.
pub mod unsigned_decimal;
pub mod little_endian_decimal;
pub mod integer_decimal;
pub mod error;
pub mod exponent;
/// Parser for decimal notation
/// It can not parse scientific notation
#[derive(Debug, Clone, Copy, Default)]
pub struct BiscuitConverter {}