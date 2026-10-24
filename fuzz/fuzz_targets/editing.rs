#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| revue_fuzz::editing(data));
