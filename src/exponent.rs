#[inline]
pub fn exponent_u128(length: usize) -> u128 {
    if length <= 8 {
        if length == 0 {
            1_u128
        } else if length == 1 {
            10_u128
        } else if length == 2 {
            100_u128
        } else if length == 3 {
            1_000_u128
        } else if length == 4 {
            10_000_u128
        } else if length == 5 {
            100_000_u128
        } else if length == 6 {
            1_000_000_u128
        } else if length == 7 {
            10_000_000_u128
        } else {
            100_000_000_u128
        }
    } else {
        if length == 9 {
            1_000_000_000_u128
        } else if length == 10 {
            10_000_000_000_u128
        } else if length == 11 {
            100_000_000_000_u128
        } else if length == 12 {
            1_000_000_000_000_u128
        } else if length == 13 {
            10_000_000_000_000_u128
        } else if length == 14 {
            100_000_000_000_000_u128
        } else if length == 15 {
            1_000_000_000_000_000_u128
        } else if length == 16 {
            10_000_000_000_000_000_u128
        } else if length == 17 {
            100_000_000_000_000_000_u128
        } else if length == 18 {
            1_000_000_000_000_000_000_u128
        } else {
            10u128.pow(length as u32)
        }
    }
}
/*
#[inline]
pub fn exponent_u64(length: usize) -> u64 {
    if length <= 4 {
        if length == 0 {
            1_u64
        } else if length == 1 {
            10_u64
        } else if length == 2 {
            100_u64
        } else if length == 3 {
            1_000_u64
        } else {
            10_000_u64
        }
    } else {
        if length == 5 {
            100_000_u64
        } else if length == 6 {
            1_000_000_u64
        } else if length == 7 {
            10_000_000_u64
        } else {
            10u64.pow(length as u32)
        }
    }
}
*/