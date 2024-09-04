use num_traits::{PrimInt, Signed};

#[inline(always)]
fn div_rem<T>(dividend: T, divisor: T) -> (T, T)
where
    T: PrimInt + Signed,
{
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);
    (quotient, remainder)
}

#[inline]
pub fn u64_length(n: u64) -> usize {
    if n < 10_000_000_000 {
        if n < 10 { return 1; }
        else if n < 100 { return 2; }
        else if n < 1_000 { return 3; }
        else if n < 10_000 { return 4; }
        else if n < 100_000 { return 5; }
        else if n < 1_000_000 { return 6; }
        else if n < 10_000_000 { return 7; }
        else if n < 100_000_000 { return 8; }
        else if n < 1_000_000_000 { return 9; }
        else { return 10; }
    } else {
        if n < 100_000_000_000 { return 11; }
        else if n < 1_000_000_000_000 { return 12; }
        else if n < 10_000_000_000_000 { return 13; }
        else if n < 100_000_000_000_000 { return 14; }
        else if n < 1_000_000_000_000_000 { return 15; }
        else if n < 10_000_000_000_000_000 { return 16; }
        else if n < 100_000_000_000_000_000 { return 17; }
        else if n < 1_000_000_000_000_000_000 { return 18; }
        else { return 19; }
    }
}

#[inline]
pub fn u32_length(n: u32) -> usize {
    if n < 10_000 {
        if n < 10 { return 1; }
        else if n < 100 { return 2; }
        else if n < 1_000 { return 3; }
        else { return 4; }
    } else {
        if n < 100_000 { return 5; }
        else if n < 1_000_000 { return 6; }
        else if n < 10_000_000 { return 7; }
        else if n < 100_000_000 { return 8; }
        else { return 9; }
    }
}

#[inline]
pub fn div10u32(n: u32) -> (u32, u32) {
    let q = (n >> 1) + (n >> 2);
    let q = q + (q >> 4);
    let q = q + (q >> 8);
    let q = q + (q >> 16);
    let q = q >> 3;
    let r = n - (((q << 2) + q) << 1);
    (q, r)
}

#[inline]
pub fn div10u64(n: u64) -> (u64, u64) {
    let q = (n >> 1) + (n >> 2);
    let q = q + (q >> 4);
    let q = q + (q >> 8);
    let q = q + (q >> 16);
    let q = q + (q >> 32);
    let q = q >> 3;
    let r = n - q * 10;
    if r > 9 {
        (q + 1, r - 10)
    } else {
        (q, r)
    }
}


pub fn split_f64(value: f64) -> (u64, u64, bool) {
    if value.is_nan() || value.is_infinite() {
        return (0, 0, false);
    }

    let is_negative = value.is_sign_negative();
    let abs_value = value.abs();
    let integer_part = abs_value.trunc() as u64;
    
    // 소수 부분 처리
    let fractional_part = abs_value.fract();
    let scaled_fract = (fractional_part * 1000.0).round() as u64;

    (integer_part, scaled_fract, is_negative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split_f64(123.456), (123, 456, false));
        assert_eq!(split_f64(-123.456), (123, 456, true));
        assert_eq!(split_f64(0.456), (0, 456, false));
        assert_eq!(split_f64(-0.456), (0, 456, true));
        assert_eq!(split_f64(123456789123.456789), (123456789123, 456789, false));
    }

    #[test]
    fn test_div_rem() {
        assert_eq!(div_rem(10, 3), (3, 1));
        assert_eq!(div_rem(10, 3), (-3, 1));
        assert_eq!(div_rem(-10, 3), (-3, -1));
        assert_eq!(div_rem(-10, -3), (3, -1));
    }

    #[test]
    fn test_div_rem_bit_u32() {
        assert_eq!(div10u32(1), (0, 1));
        assert_eq!(div10u32(12), (1, 2));
        assert_eq!(div10u32(123), (12, 3));
        assert_eq!(div10u32(1234), (123, 4));
        assert_eq!(div10u32(12345), (1234, 5));
        assert_eq!(div10u32(123456), (12345, 6));
        assert_eq!(div10u32(1234567), (123456, 7));
        assert_eq!(div10u32(12345678), (1234567, 8));
        assert_eq!(div10u32(123456789), (12345678, 9));
    }

    #[test]
    fn test_div_rem_bit_u64() {
        assert_eq!(div10u64(1), (0, 1));
        assert_eq!(div10u64(12), (1, 2));
        assert_eq!(div10u64(123), (12, 3));
        assert_eq!(div10u64(1234), (123, 4));
        assert_eq!(div10u64(12345), (1234, 5));
        assert_eq!(div10u64(123456), (12345, 6));
        assert_eq!(div10u64(1234567), (123456, 7));
        assert_eq!(div10u64(12345678), (1234567, 8));
        assert_eq!(div10u64(123456789), (12345678, 9));
        assert_eq!(div10u64(1234567890), (123456789, 0));
        assert_eq!(div10u64(12345678901), (1234567890, 1));
        assert_eq!(div10u64(123456789012), (12345678901, 2));
        assert_eq!(div10u64(1234567890123), (123456789012, 3));
        assert_eq!(div10u64(12345678901234), (1234567890123, 4));
        assert_eq!(div10u64(123456789012345), (12345678901234, 5));
    }

    #[test]
    fn test_u64_length() {
        assert_eq!(u64_length(0), 1);
        assert_eq!(u64_length(1), 1);
        assert_eq!(u64_length(9), 1);
        assert_eq!(u64_length(10), 2);
        assert_eq!(u64_length(99), 2);
        assert_eq!(u64_length(100), 3);
        assert_eq!(u64_length(999), 3);
        assert_eq!(u64_length(1000), 4);
        assert_eq!(u64_length(9999), 4);
        assert_eq!(u64_length(10000), 5);
        assert_eq!(u64_length(99999), 5);
        assert_eq!(u64_length(100000), 6);
        assert_eq!(u64_length(999999), 6);
        assert_eq!(u64_length(1000000), 7);
        assert_eq!(u64_length(9999999), 7);
        assert_eq!(u64_length(10000000), 8);
        assert_eq!(u64_length(99999999), 8);
        assert_eq!(u64_length(100000000), 9);
        assert_eq!(u64_length(999999999), 9);
        assert_eq!(u64_length(1000000000), 10);
        assert_eq!(u64_length(9999999999), 10);
        assert_eq!(u64_length(10000000000), 11);
        assert_eq!(u64_length(99999999999), 11);
        assert_eq!(u64_length(100000000000), 12);
        assert_eq!(u64_length(999999999999), 12);
        assert_eq!(u64_length(1000000000000), 13);
        assert_eq!(u64_length(9999999999999), 13);
        assert_eq!(u64_length(10000000000000), 14);
        assert_eq!(u64_length(99999999999999), 14);
        assert_eq!(u64_length(100000000000000), 15);
        assert_eq!(u64_length(999999999999999), 15);
        assert_eq!(u64_length(1000000000000000), 16);
        assert_eq!(u64_length(9999999999999999), 16);
        assert_eq!(u64_length(10000000000000000), 17);
        assert_eq!(u64_length(99999999999999999), 17);
        assert_eq!(u64_length(100000000000000000), 18);
        assert_eq!(u64_length(999999999999999999), 18);
        assert_eq!(u64_length(1000000000000000000), 19);
    }

    #[test]
    fn test_u32_length() {
        assert_eq!(u32_length(0), 1);
        assert_eq!(u32_length(1), 1);
        assert_eq!(u32_length(9), 1);
        assert_eq!(u32_length(10), 2);
        assert_eq!(u32_length(99), 2);
        assert_eq!(u32_length(100), 3);
        assert_eq!(u32_length(999), 3);
        assert_eq!(u32_length(1000), 4);
        assert_eq!(u32_length(9999), 4);
        assert_eq!(u32_length(10000), 5);
        assert_eq!(u32_length(99999), 5);
        assert_eq!(u32_length(100000), 6);
        assert_eq!(u32_length(999999), 6);
        assert_eq!(u32_length(1000000), 7);
        assert_eq!(u32_length(9999999), 7);
        assert_eq!(u32_length(10000000), 8);
        assert_eq!(u32_length(99999999), 8);
        assert_eq!(u32_length(100000000), 9);
        assert_eq!(u32_length(999999999), 9);
    }
}

