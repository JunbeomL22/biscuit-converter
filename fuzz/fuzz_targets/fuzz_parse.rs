#![no_main]

use libfuzzer_sys::fuzz_target;
use biscuit_converter::Biscuit;

fuzz_target!(|data: &[u8]| {
    if data.iter().all(|&b| b >= 48 && b <= 57) && data.len() < 38 {
        let parser = Biscuit::default();
        let _ = parser.to_i128(data);
        let _ = parser.to_u128(data);
    }   

    if data.iter().all(|&b| b >= 48 && b <= 57) && data.len() < 18 {
        let parser = Biscuit::default();
        let _ = parser.to_i64(data);
        let _ = parser.to_u64(data);
    }
});
