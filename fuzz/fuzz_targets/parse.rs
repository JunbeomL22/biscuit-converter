#![no_main]

use libfuzzer_sys::fuzz_target;
use biscuit_converter::BiscuitConverter;

fuzz_target!(|data: &[u8]| {
    let parser = BiscuitConverter::default();

    let _ = parser.to_u32(data);
    let _ = parser.to_u64(data);
    let _ = parser.to_i32(data);
    let _ = parser.to_i64(data);
});
