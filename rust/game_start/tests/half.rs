//! The page's half is of these declarations: what `half` would write is what is checked in.

/// A FAILURE MEANS: a declaration of what crosses to a page was changed and the page's own source was not written again (`cargo run --profile gate --bin half`).
#[test]
fn the_page_s_half_is_of_these_declarations() {
    let o = std::process::Command::new(env!("CARGO_BIN_EXE_half")).arg("--check").output().expect("half");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stdout));
}
