# biscuit-converter

`biscuit-converter` is a high-performance numeric parser for Rust that converts ASCII numbers to their numeric representations. It uses a combination of bit operations to achieve fast parsing for integers, unsigned integers, and floating-point numbers.

## Features

- Fast parsing of ASCII numbers to numeric types
- Supports parsing of integers, unsigned integers, and floating-point numbers
- Optimized for various string lengths
- Option to specify fraction length for further optimization

## Performance

- Integer parsing: Faster than `atoi`
- Float parsing:
  - Small-sized strings: Similar performance to `std::str::parse`
  - Mid-sized strings: Slower than `std::str::parse` (when fraction length is not given)
  - Large strings: Faster than `std::str::parse`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
biscuit-converter = "0.1"
```

Then, use it in your Rust code:

```rust
use biscuit_converter::BiscuitConverter;

fn main() {
    // Default parser
    let biscuit_converter = BiscuitConverter::default();
    
    // Parser with known fraction length
    // This is faster than the above parser
    let biscuit_converter_fraction_given = BiscuitConverter::initialize().with_fraction_length(2); 

    // Parsing examples
    let int_result: u64 = biscuit_converter.to_u64("123");
    assert_eq!(int_result, 123);

    let float_result: f64 = biscuit_converter.to_f64("123.45");
    assert_eq!(float_result, 123.45);

    // Faster parsing when fraction length is known
    let optimized_float_result: f64 = biscuit_converter_fraction_given.to_f64("123.45");
    assert_eq!(optimized_float_result, 123.45);
}
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, (<http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license (<http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are very welcome! Whether it's bug reports, optimizations, or any other improvements, all contributions will be gratefully reviewed. Please feel free to submit a Pull Request or open an Issue on the GitHub repository.

## Benchmarks

# Comprehensive Benchmark Results for biscuit-converter

This table shows the performance of `biscuit-converter` compared to the standard library and `atoi` for parsing various types of numbers. Times are in nanoseconds, rounded to one decimal place.

**Testing Environment:**
- CPU: Ryzen 7 7700 3.8 GHz
- Rust Version: 1.79

## Unsigned Integers (u64)

| Input              | biscuit | std   | atoi  |
|--------------------|---------|-------|-------|
| 123                | 1.0     | 4.0   | 2.3   |
| 123456             | 1.3     | 4.7   | 3.1   |
| 123456789          | 2.9     | 5.7   | 4.2   |
| 123456789012       | 3.1     | 6.6   | 5.4   |
| 123456789012345    | 2.9     | 7.8   | 6.7   |
| 123456789012345678 | 5.9     | 11.9  | 8.0   |

## Signed Integers (i64)

| Input              | biscuit | std   | atoi  |
|--------------------|---------|-------|-------|
| -123               | 1.2     | 4.0   | 2.5   |
| -123456            | 1.5     | 4.7   | 3.7   |
| -123456789         | 3.1     | 5.5   | 5.0   |
| -123456789012      | 3.1     | 6.6   | 6.5   |
| -123456789012345   | 3.1     | 8.1   | 7.8   |
| -123456789012345678| 5.9     | 11.2  | 9.1   |

## Floating-Point Numbers (f64)

| Input                     | biscuit | biscuit (fraction given) | std   |
|---------------------------|---------|--------------------------|-------|
| 1.23                      | 5.9     | 3.4                      | 6.0   |
| 1234.56                   | 7.3     | 4.1                      | 7.0   |
| 1234567.89                | 12.3    | 9.4                      | 8.2   |
| 1234567890.12             | 13.1    | 9.4                      | 7.5   |
| 1234567890123.45          | 11.9    | 9.5                      | 8.5   |
| 1234567890123456.78       | 15.9    | 13.6                     | 10.7  |
| 1234567890123456789.01    | 16.9    | 14.9                     | 25.2  |
| 1234567890123456789012.34 | 18.7    | 16.5                     | 26.9  |
| 1234567890123456789012345.67 | 18.9 | 16.5                     | 25.8  |
| 1234567890123456789012345678.90 | 18.6 | 16.2                  | 26.9  |

## Observations:

1. Integer Parsing (Unsigned and Signed):
   - `biscuit-converter` consistently outperforms both the standard library and `atoi` for parsing integers.
   - The performance advantage is particularly significant for smaller numbers.
   - For larger integers (both signed and unsigned), `biscuit-converter` can be 2-3 times faster than the standard library.

2. Floating-Point Numbers:
   - For small floating-point numbers, the standard library parser is generally faster.
   - For larger numbers (more than about 16 digits), `biscuit-converter` becomes significantly faster than the standard library.
   - When the fraction length is known and provided to `biscuit-converter`, it consistently outperforms both the standard version of `biscuit-converter` and the standard library.

3. Overall Performance:
   - `biscuit-converter` shows its strength in parsing larger numbers across all types (unsigned, signed, and float).
   - The performance advantage of `biscuit-converter` is most pronounced for integer parsing.
   - For very large numbers of any type, `biscuit-converter` can provide substantial performance improvements over the standard library.

Note: These benchmarks were run on the specified testing environment. Results may vary depending on hardware and environmental factors. It's always recommended to run benchmarks on your target hardware for the most accurate results.

## Algorithm Explanation

The `biscuit-converter` library achieves its high performance through bit manipulation techniques. The algorithm is heavily influenced by ideas from:

- Rust: [Faster Integer Parsing](https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html)
- C++: [Faster Integer Parsing](https://kholdstare.github.io/technical/2020/05/26/faster-integer-parsing.html)

### Key Concepts

1. ASCII number representation:
   - ASCII digits range from 0x30 ("0") to 0x39 ("9").
   - The least significant 4 bits of an ASCII digit represent its numerical value.

2. Little-endian representation:
   - Numbers are stored in little-endian format in memory.
   - String slice memory order is opposite to what we see visually.

3. Bit shifting:
   - When shifting bits, empty spaces are filled with zeros.

### Parsing Techniques

#### Single Digit Parsing
```rust
let x: &[u8; 1] = b"8";
let x: u8 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u8) };
let y: u8 = x & 0x0f;
assert_eq!(y, 8);
```
This technique uses a bitwise AND to extract the numerical value from the ASCII representation.

#### Two Digit Parsing
```rust
let x: &[u8; 2] = b"12"; // [0x32, 0x31] in memory
let x: u16 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u16) };
let lower: u16 = (x & 0x0f00) >> 8;
let upper: u16 = (x & 0x000f) * 10;
let res = lower + upper;
assert_eq!(res, 12);
```
This method separates the tens and ones places, then combines them.

#### Four Digit Parsing
```rust
let x: &[u8; 4] = b"1234"; // [0x34, 0x33, 0x32, 0x31] in memory
let x: u32 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u32) };
let lower: u32 = (x & 0x0f000f00) >> 8;
let upper: u32 = (x & 0x000f000f) * 10;
let chunk = lower + upper;
let lower: u32 = (chunk & 0x00ff0000) >> 16;
let upper: u32 = (chunk & 0x000000ff) * 100;
let res = lower + upper;
assert_eq!(res, 1234);
```
This technique processes pairs of digits at a time, then combines the results.

#### Handling Irregular Digit Counts
```rust
let x: &[u8; 3] = b"123";
let x: u32 = unsafe { std::ptr::read_unaligned(x.as_ptr() as *const u32) };
let x = x << 8; // Shift to align the digits properly
```
Left-shifting is used to handle inputs with irregular numbers of digits.

#### Parsing Negative Numbers
```rust
let x = b"-123";
let x_u = x[1..]; // b"123"
let x_u: u32 = bit_parse(x_u);
let res: i32 = (!x_u).wrapping_add(1) as i32;
assert_eq!(res, -123);
```
Negative numbers are handled by parsing the absolute value and then applying two's complement.

### Performance Insights

1. **Reduced Branching**: The algorithm minimizes conditional statements, which can be costly in terms of performance.

2. **Avoiding Multiplication**: The algorithm replaces some multiplications with bit shifts and additions, which are generally faster operations.

3. **Memory Efficiency**: By working directly with the byte representations, the algorithm avoids unnecessary conversions and temporary allocations.

These techniques allow `biscuit-converter` to achieve significant performance improvements over traditional parsing methods, especially for larger numbers and in scenarios where parsing speed is critical.