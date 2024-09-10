#![no_main]

use libfuzzer_sys::fuzz_target;
use biscuit_converter::BiscuitConverter;

fuzz_target!(|data: &[u8]| {
    let parser = BiscuitConverter::default();

    if data.iter().all(|&b| b >= 48 && b <= 57) {
        let parser = BiscuitConverter::default();
        let _ = parser.to_i128(data);
        let _ = parser.to_i64(data);
        let _ = parser.to_u128(data);
        let _ = parser.to_u64(data);
    }   
});