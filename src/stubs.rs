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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_rem() {
        assert_eq!(div_rem(10, 3), (3, 1));
        assert_eq!(div_rem(10, -3), (-3, 1));
        assert_eq!(div_rem(-10, 3), (-3, -1));
        assert_eq!(div_rem(-10, -3), (3, -1));
    }
}
