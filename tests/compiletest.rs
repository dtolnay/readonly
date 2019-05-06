#[test]
fn ui() {
    if !version_check::is_nightly().unwrap() {
        return;
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
