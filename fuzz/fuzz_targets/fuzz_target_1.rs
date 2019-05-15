#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use serde_sexpr::Value;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = s.parse::<Value>();
    }
});
