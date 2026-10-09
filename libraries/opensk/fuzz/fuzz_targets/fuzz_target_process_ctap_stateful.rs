#![no_main]

use fuzz_helper::process_ctap_stateful;
use libfuzzer_sys::fuzz_target;

// Fuzz sequences of CTAP commands against a persistent authenticator state.
fuzz_target!(|data: &[u8]| {
    process_ctap_stateful(data).ok();
});
