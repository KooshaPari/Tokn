#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz the JSON parsing path in tokenledger.
    // This catches panics on malformed or adversarial payloads.
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = serde_json::from_str::<serde_json::Value>(s);
    }
});
