# biscuit-converter

biscuit-converter is a high-performance integer conversion library using bitwise operations. It provides safe and efficient ASCII to integer conversion.

## Key Features

- **High Performance**: Maintains consistent performance as the number of digits increases, showing exceptional speed especially for large numbers.
- **Safety**: No unsafe code is used.
- **Lightweight**: No external libraries are used besides the std library.
- **Wide Integer Support**: Supports conversion for u16, u32, u64, u128, i16, i32, i64, i128.
- **ASCII Specialized**: Only capable of converting ASCII type numbers.

## Performance

biscuit-converter shows outstanding performance, particularly for large number conversions:

- For numbers with 16 or more digits, it's more than 3 times faster than atoi.
- For numbers with 32 or more digits, it's nearly 4 times faster than atoi.

Test machine: ***Ryzen 7 7700 3.8Ghz, rust 1.79***

### `i128`
| Input                             | biscuit | std    | atoi   |
|-----------------------------------|---------|--------|--------|
| -1                                | 3.2 ns  | 7.1 ns | 3.4 ns |
| -12                               | 3.1 ns  | 8.2 ns | 3.7 ns |
| -123                              | 3.2 ns  | 7.5 ns | 4.9 ns |
| -1234                             | 3.2 ns  | 7.9 ns | 4.9 ns |
| -12345                            | 3.2 ns  | 9.4 ns | 5.7 ns |
| -123456                           | 3.4 ns  | 8.9 ns | 6.3 ns |
| -1234567                          | 3.6 ns  | 9.5 ns | 7.2 ns |
| -12345678                         | 3.2 ns  | 10.6 ns| 8.1 ns |
| -123456789                        | 3.4 ns  | 10.2 ns| 9.3 ns |
| -1234567890                       | 3.6 ns  | 9.8 ns | 9.6 ns |
| -12345678901                      | 3.9 ns  | 11.4 ns| 10.9 ns|
| -123456789012                     | 3.8 ns  | 11.6 ns| 12.0 ns|
| -1234567890123                    | 4.1 ns  | 13.4 ns| 12.1 ns|
| -12345678901234                   | 4.1 ns  | 13.5 ns| 13.5 ns|
| -123456789012345                  | 4.6 ns  | 15.6 ns| 14.1 ns|
| -1234567890123456                 | 4.1 ns  | 16.7 ns| 15.0 ns|
| -12345678901234567                | 4.4 ns  | 17.2 ns| 16.0 ns|
| -123456789012345678               | 4.5 ns  | 17.6 ns| 17.3 ns|
| -1234567890123456789              | 4.8 ns  | 19.2 ns| 18.4 ns|
| -12345678901234567890             | 4.7 ns  | 20.0 ns| 18.9 ns|
| -123456789012345678901            | 5.0 ns  | 20.2 ns| 20.0 ns|
| -1234567890123456789012           | 5.1 ns  | 21.0 ns| 20.6 ns|
| -12345678901234567890123          | 5.3 ns  | 22.6 ns| 21.3 ns|
| -123456789012345678901234         | 4.9 ns  | 22.9 ns| 22.4 ns|
| -1234567890123456789012345        | 5.1 ns  | 23.6 ns| 23.3 ns|
| -12345678901234567890123456       | 5.4 ns  | 25.4 ns| 24.1 ns|
| -123456789012345678901234567      | 5.7 ns  | 25.9 ns| 25.1 ns|
| -1234567890123456789012345678     | 5.4 ns  | 26.5 ns| 25.9 ns|
| -12345678901234567890123456789    | 5.8 ns  | 28.3 ns| 27.0 ns|
| -123456789012345678901234567890   | 6.1 ns  | 28.7 ns| 27.9 ns|
| -1234567890123456789012345678901  | 6.5 ns  | 28.8 ns| 28.9 ns|
| -12345678901234567890123456789012 | 5.9 ns  | 47.5 ns| 30.6 ns|
| -123456789012345678901234567890123| 6.7 ns | 53.9 ns| 30.9 ns|
| -1234567890123456789012345678901234| 7.1 ns| 55.9 ns| 31.6 ns|
| -12345678901234567890123456789012345| 7.0 ns| 53.2 ns| 31.5 ns|
| -123456789012345678901234567890123456| 6.9 ns| 54.1 ns| 31.7 ns|
| -1234567890123456789012345678901234567| 7.8 ns| 58.4 ns| 32.5 ns|
| -12345678901234567890123456789012345678| 8.2 ns| 59.7 ns| 34.3 ns|

### `i64`
| Input                    | biscuit | std    | atoi   |
|--------------------------|---------|--------|--------|
| -1                       | 1.7 ns  | 3.4 ns | 1.7 ns |
| -12                      | 1.9 ns  | 3.5 ns | 2.1 ns |
| -123                     | 2.0 ns  | 4.0 ns | 2.4 ns |
| -1234                    | 2.0 ns  | 4.1 ns | 2.8 ns |
| -12345                   | 2.2 ns  | 4.3 ns | 3.2 ns |
| -123456                  | 2.2 ns  | 4.7 ns | 3.8 ns |
| -1234567                 | 2.4 ns  | 4.9 ns | 4.1 ns |
| -12345678                | 2.2 ns  | 5.1 ns | 4.5 ns |
| -123456789               | 2.4 ns  | 5.5 ns | 5.0 ns |
| -1234567890              | 2.6 ns  | 5.6 ns | 5.4 ns |
| -123456789012            | 2.8 ns  | 6.5 ns | 6.4 ns |
| -1234567890123           | 3.0 ns  | 6.9 ns | 6.9 ns |
| -12345678901234          | 3.2 ns  | 7.4 ns | 7.4 ns |
| -123456789012345         | 3.4 ns  | 7.9 ns | 7.8 ns |
| -1234567890123456        | 3.2 ns  | 9.7 ns | 8.2 ns |
| -12345678901234567       | 3.2 ns  | 10.4 ns| 9.0 ns |
| -123456789012345678      | 3.5 ns  | 11.0 ns| 9.1 ns |
| -1234567890123456789     | 3.9 ns  | 11.5 ns| 10.0 ns|

## Features
- The return type is `Option`
- When MAX+1 is given:
  - For unsigned integer types, it returns None
  - For signed integer types, if it's within the bounds of the unsigned type (e.g., u64 bound for i64 case), it returns its two's complement
- None cases: empty string, "-" in signed integer, numeric over the bound of unsigned

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
biscuit-converter = "0.2"
```
Then, use it in your Rust code:
```rust
use biscuit_converter::BiscuitConverter;
fn main() {
    let biscuit_converter = BiscuitConverter::default();
    // Parsing examples
    let int_result: u64 = biscuit_converter.to_u64("123");
    assert_eq!(u64_result, 123);
    let i64_result: i64 = biscuit_converter.to_i64("-123");
    assert_eq!(i64_result, 123);
}
```

This crate provides a fast, safe, and efficient way to convert ASCII representations of numbers to various integer types, outperforming standard library and atoi implementations, especially for larger numbers.
## Algorithm Explanation

The `biscuit-converter` library achieves its high performance through bit manipulation techniques. The algorithm is heavily influenced by ideas from:

- Rust: [Faster Integer Parsing](https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html)
- C++: [Faster Integer Parsing](https://kholdstare.github.io/technical/2020/05/26/faster-integer-parsing.html)

### Key Concepts

1. ASCII number representation:
   - ASCII digits range from 0x30 ("0") to 0x39 ("9").
   - The least significant 4 bits of an ASCII digit represent its numerical value.

2. Little-endian representation is assumed

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

# License
This project is licensed under either of

- Apache License, Version 2.0, <http://www.apache.org/licenses/LICENSE-2.0>
- MIT license <http://opensource.org/licenses/MIT>

at your option.

# Contributions

We welcome all kinds of contributions! Whether it's bug reports, feature requests, or code contributions, your input is valuable and appreciated. Feel free to open issues or submit pull requests on our GitHub repository.