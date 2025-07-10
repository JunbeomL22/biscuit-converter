#![no_main]

use libfuzzer_sys::fuzz_target;
use biscuit_converter::Biscuit;

fuzz_target!(|data: &[u8]| {
    if data.iter().all(|&b| b >= 48 && b <= 57) && data.len() < 18 {
        let _ = i64::parse_decimal(data);
        let _ = u64::parse_decimal(data);
    }   

});
