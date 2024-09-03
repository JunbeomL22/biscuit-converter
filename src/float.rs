use crate::FlashParser;
use memchr::memchr;

impl FlashParser {
    pub fn float_to_u64<T: AsRef<[u8]>>(self, input: T) -> f64 {
        let u = input.as_ref();
        let length = u.len();
        let fraction_length = match self.fraction_length {
            Some(fraction_length) => fraction_length,
            None => memchr(b'.', u).unwrap_or(0)
        };
     
        match fraction_length {
            0 => self.to_u64(u),
            1 => self.to_u64(&u[..(length - 1)]),
            _ => {
                let integer = self.to_u64(&u[..(fraction_length)]);
                let fraction = self.to_u64(&u[(fraction_length + 1)..]);
                let fraction_length = fraction.to_string().len();
                let fraction = fraction as f64 / 10u64.pow(fraction_length as u32) as f64;
                integer as f64 + fraction
            }
        },
    }
}
/*
#[inline(always)]
pub fn parse_under8_with_floating_point(u: &[u8], length: usize, point_length: usize) -> u64 {
    debug_assert!(
        (1..=8).contains(&length),
        "parse_under8: length must be less than or equal to 8"
    );
    match point_length {
        // ex) u = "12345"
        0 => parse_under8(u, length),
        // ex) u = "12345."
        1 => parse_under8(&u[..(length - 1)], length - 1),
        _ => {
            // ex) u = "123.45", length = 6, point_length = 3,
            // "123.45" => "??123.45"
            let mut chunk: u64 = unsafe { read_unaligned(u.as_ptr() as *const u64) };
            // "??123.45" => "123.4500"
            chunk <<= 64 - (length * 8);
            // "123.4500" => "12345000"

            let point_mask = 0xffff_ffff_ffff_ffff << ((9 - point_length) * 8);
            let decimal_mask = !point_mask;
            chunk = (chunk & point_mask) + ((chunk & (decimal_mask >> 8)) << 8);
            u8_chunk_to_u64_decimal(chunk)
        }
    }
}
    */