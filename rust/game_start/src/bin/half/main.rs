//! THE PAGE'S HALF, WRITTEN: what Rust declares of what crosses to a page, as the page's own source. It reads the declarations' own tables and no source text.
//!
//!   half            writes the files · half --check   says which would change, and is 1 where one would
//!
//! The files are checked in, so a machine without cargo builds and plays. A page holds the stamp they were written with, and a module of another is refused as it loads.
//! To change one, change its declaration: a section in `game_draw`, a command in `host/command.rs`, an ask in `host/desk.rs`, a view in `game_types`,
//! a row a shader reads in `game_draw::gpu`, a word's bits where they stand (`bits!`), a table's row where it stands.
mod bits;
mod names;
mod tables;
mod typescript;
mod wgsl;
use std::path::PathBuf;

fn main() {
    let check = match std::env::args().nth(1).as_deref() { None => false, Some("--check") => true, Some(w) => { eprintln!("half: no such word {w} (--check)"); std::process::exit(64) } };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let stamp = typescript::stamp();
    let mut off = Vec::new();
    for (file, text) in typescript::files(stamp).into_iter().chain(wgsl::files()).chain(bits::files()).chain(names::files()).chain(tables::files()) {
        let at = root.join(file);
        if std::fs::read_to_string(&at).is_ok_and(|t| t == text) { continue; }
        off.push(file);
        if !check { std::fs::write(&at, text).unwrap_or_else(|e| panic!("{file}: {e}")); }
    }
    match (check, off.is_empty()) {
        (_, true) => println!("half: the page's half is of these declarations (stamp {stamp})"),
        (true, false) => { println!("half: {} would change (stamp {stamp}): cargo run --profile gate --bin half", off.join(", ")); std::process::exit(1) }
        (false, false) => println!("half: {} written (stamp {stamp})", off.join(", ")),
    }
}
