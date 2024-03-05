use protosol::fixture::Fixture;

// Perhaps this can be macro-ized.
fn get_test_elf<'a>() -> &'a [u8] {
    include_bytes!("../target/deploy/test_program.so")
}

pub fn process_fixture(fixture: Fixture) {
    let elf = get_test_elf();
    protosol::process_fixture(fixture, elf);
}
