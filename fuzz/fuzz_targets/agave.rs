#![no_main]

use {
    libfuzzer_sys::fuzz_target,
    protosol::fixture::Fixture,
    std::{env, fs},
};

// Agave program runtime.
fuzz_target!(|data: &[u8]| {
    let elf = fs::read(env::var("PROGRAM").expect("Environment variable PROGRAM not set"))
        .expect("Failed to read program ELF file.");

    if let Ok(fixture) = Fixture::decode(data) {
        protosol::process_fixture(fixture, &elf);
    }
});
