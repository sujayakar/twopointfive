//! Where the game is, found by listing `rust/`.
//!
//! No pass of this tool names a crate. The crates are the folders of `rust/` that hold a `Cargo.toml`, and what each is
//! comes from what it holds:
//! - the crate whose `src/knobs.rs` declares `struct Knobs` holds the tables (the simulation has a knobs.rs of its own: a list
//!   of the knobs it reads, and no tables);
//! - THE GAME'S CRATES are that crate and every plain library that depends on one of the game's crates, however far;
//! - a crate that builds something to run (a `src/bin`, a `src/main.rs`, a `[[bin]]`, a `cdylib`) is a USER of the game, and so
//!   is every crate that is none of the game's: a name one of them spells keeps an item, and each is checked after a pass;
//! - a BODY CRATE is one of the game's crates with body files (`dead::is_body`): the files the passes read and edit.
//! One crate today, several once the game is split: the passes read this and nothing else.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

pub struct Crate {
    /// its folder under `rust/`
    pub name: String,
    /// the crates under `rust/` its manifest names by path
    pub deps: BTreeSet<String>,
    /// it builds something to run
    pub ends: bool,
    /// it builds only a module for the page (a `cdylib` and no `rlib`): it is checked for wasm
    pub page: bool,
}

pub struct Layout {
    pub crates: Vec<Crate>,
    /// the game's crates
    pub game: BTreeSet<String>,
    /// the tables' file, from the root
    pub knobs: PathBuf,
    /// the game's crates that hold body files, a crate after the ones it depends on
    pub bodies: Vec<String>,
}

/// The manifest's path dependencies that are folders beside it, whether it builds something to run, and whether that is only a
/// module for the page.
fn manifest(text: &str) -> (BTreeSet<String>, bool, bool) {
    let mut deps = BTreeSet::new();
    let mut ends = false;
    let mut page = false;
    let mut in_deps = false;
    for l in text.lines() {
        let l = l.split('#').next().unwrap_or("").trim();
        if l.starts_with('[') {
            in_deps = l.trim_matches(|c| c == '[' || c == ']').ends_with("dependencies");
            ends |= l == "[[bin]]";
            continue;
        }
        if l.starts_with("crate-type") && l.contains("\"cdylib\"") {
            ends = true;
            page = !l.contains("\"rlib\"") && !l.contains("\"lib\"");
        }
        if !in_deps {
            continue;
        }
        if let Some(i) = l.find("path") {
            let rest = l[i + 4..].trim_start();
            if let Some(v) = rest.strip_prefix('=').map(|r| r.trim_start()).and_then(|r| r.strip_prefix('"')).and_then(|r| r.split('"').next()) {
                if let Some(d) = v.strip_prefix("../") {
                    deps.insert(d.trim_end_matches('/').to_string());
                }
            }
        }
    }
    (deps, ends, page)
}

pub fn find(root: &Path) -> Result<Layout, String> {
    let rust = root.join("rust");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&rust).map_err(|e| format!("cannot list {}: {e}", rust.display()))?.flatten().map(|e| e.path()).filter(|p| p.is_dir() && p.join("Cargo.toml").exists()).collect();
    dirs.sort();
    let mut crates = Vec::new();
    for d in &dirs {
        let name = d.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let text = std::fs::read_to_string(d.join("Cargo.toml")).map_err(|e| format!("cannot read {}/Cargo.toml: {e}", d.display()))?;
        let (deps, mut ends, page) = manifest(&text);
        ends |= d.join("src/bin").is_dir() || d.join("src/main.rs").exists();
        crates.push(Crate { name, deps, ends, page });
    }
    let with_knobs: Vec<&Crate> = crates.iter().filter(|c| std::fs::read_to_string(rust.join(&c.name).join("src/knobs.rs")).map(|t| t.contains("pub struct Knobs ")).unwrap_or(false)).collect();
    let [types] = with_knobs.as_slice() else {
        return Err(format!("{} crates under rust/ have a src/knobs.rs that declares `struct Knobs` ({}): the tables are in exactly one", with_knobs.len(), with_knobs.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")));
    };
    let mut game: BTreeSet<String> = BTreeSet::from([types.name.clone()]);
    loop {
        let more: Vec<String> = crates.iter().filter(|c| !c.ends && !game.contains(&c.name) && c.deps.iter().any(|d| game.contains(d))).map(|c| c.name.clone()).collect();
        if more.is_empty() {
            break;
        }
        game.extend(more);
    }
    let knobs = Path::new("rust").join(&types.name).join("src/knobs.rs");
    let mut l = Layout { crates, game, knobs, bodies: Vec::new() };
    // the body crates, a crate after the ones it depends on; a module is in one of them only (a pass names a module by its name)
    let mut have: BTreeMap<String, String> = BTreeMap::new();
    let mut with_bodies: BTreeSet<String> = BTreeSet::new();
    for p in l.body_files(root) {
        let rel = p.strip_prefix(&rust).unwrap_or(&p);
        let c = rel.components().next().map(|c| c.as_os_str().to_string_lossy().to_string()).unwrap_or_default();
        let m = p.parent().and_then(|d| d.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if let Some(other) = have.insert(m.clone(), c.clone()) {
            if other != c {
                return Err(format!("module {m} has body files in two crates, {other} and {c}: a pass could not tell them apart"));
            }
        }
        with_bodies.insert(c);
    }
    while l.bodies.len() < with_bodies.len() {
        let next = with_bodies.iter().find(|c| !l.bodies.contains(*c) && l.below(c).iter().all(|d| !with_bodies.contains(d) || l.bodies.contains(d))).cloned();
        match next {
            Some(c) => l.bodies.push(c),
            None => return Err("the game's crates depend on each other in a ring".to_string()),
        }
    }
    Ok(l)
}

impl Layout {
    fn get(&self, name: &str) -> Option<&Crate> {
        self.crates.iter().find(|c| c.name == name)
    }
    /// every crate under `rust/` this one depends on, however far
    fn below(&self, name: &str) -> BTreeSet<String> {
        let mut out: BTreeSet<String> = BTreeSet::new();
        let mut todo = vec![name.to_string()];
        while let Some(n) = todo.pop() {
            for d in self.get(&n).map(|c| c.deps.iter().cloned().collect::<Vec<_>>()).unwrap_or_default() {
                if self.get(&d).is_some() && out.insert(d.clone()) {
                    todo.push(d);
                }
            }
        }
        out
    }
    /// the `src` of each of the game's crates
    pub fn src_dirs(&self, root: &Path) -> Vec<PathBuf> {
        self.game.iter().map(|c| root.join("rust").join(c).join("src")).collect()
    }
    /// every Rust file of the game's crates
    pub fn game_files(&self, root: &Path) -> Vec<PathBuf> {
        let mut all = Vec::new();
        for d in self.src_dirs(root) {
            crate::dead::rs_files(&d, &mut all);
        }
        all
    }
    pub fn body_files(&self, root: &Path) -> Vec<PathBuf> {
        // (the tables' crate holds the game's types: a file of its own there, as a seat's borrowed view, is one a pass leaves alone)
        let tables = self.knobs.components().nth(1).map(|c| c.as_os_str().to_os_string());
        self.game_files(root).into_iter().filter(|p| crate::dead::is_body(p) && !p.strip_prefix(root.join("rust")).ok().is_some_and(|r| r.components().next().map(|c| c.as_os_str().to_os_string()) == tables)).collect()
    }
    /// the crates that are not body crates and use one, however far: each has to build after a pass has changed the bodies
    pub fn users(&self) -> Vec<String> {
        self.crates.iter().filter(|c| !self.bodies.contains(&c.name) && self.below(&c.name).iter().any(|d| self.bodies.contains(d))).map(|c| c.name.clone()).collect()
    }
    /// the game's crates and whatever uses one: where a table of the game can be written out as a literal
    pub fn game_and_users(&self) -> Vec<String> {
        self.crates.iter().filter(|c| self.game.contains(&c.name) || self.below(&c.name).iter().any(|d| self.game.contains(d))).map(|c| c.name.clone()).collect()
    }
    /// what the body files' paths share from the root on (`rust/game/src/` with one body crate): left off a path in a report
    pub fn common(&self) -> String {
        match self.bodies.as_slice() {
            [one] => format!("rust/{one}/src/"),
            _ => "rust/".to_string(),
        }
    }
    pub fn say(&self) -> String {
        let others: Vec<&str> = self.crates.iter().filter(|c| !self.game.contains(&c.name)).map(|c| c.name.as_str()).collect();
        format!("the game's crates: {}; bodies in {}; the tables are {}; the other crates under rust/: {}", self.game.iter().cloned().collect::<Vec<_>>().join(", "), self.bodies.join(", "), self.knobs.display(), others.join(", "))
    }
}

/// A file the compiler named, as its path from the root: the compiler names a file of the crate it was run in from that crate's
/// folder (`src/doors/p_01.rs`) and a file of another crate in full. `at` is the folder the check ran in; `base` is the root the
/// check's tree stands in (the root itself, or the private copy's folder).
pub fn from_root(base: &Path, at: &Path, file: &str) -> String {
    let p = Path::new(file);
    let joined = if p.is_absolute() { p.to_path_buf() } else { base.join(at).join(p) };
    let mut out: Vec<Component> = Vec::new();
    for c in joined.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            c => out.push(c),
        }
    }
    let clean: PathBuf = out.iter().collect();
    let base_abs = std::fs::canonicalize(base).unwrap_or_else(|_| base.to_path_buf());
    let clean_abs = if clean.is_absolute() { clean.clone() } else { std::env::current_dir().map(|d| d.join(&clean)).unwrap_or(clean.clone()) };
    clean_abs.strip_prefix(&base_abs).or_else(|_| clean.strip_prefix(base)).map(|r| r.to_string_lossy().to_string()).unwrap_or_else(|_| file.to_string())
}

/// A check's output (`cargo check --message-format=json`) with every file named from the root, so that what reads it need not
/// know which crate the check ran in. A line that is no message is left as it is.
pub fn diag_from_root(text: &str, base: &Path, at: &Path) -> String {
    fn walk(v: &mut serde_json::Value, base: &Path, at: &Path) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m.iter_mut() {
                    match x {
                        serde_json::Value::String(s) if k == "file_name" => *s = from_root(base, at, s),
                        x => walk(x, base, at),
                    }
                }
            }
            serde_json::Value::Array(a) => a.iter_mut().for_each(|x| walk(x, base, at)),
            _ => {}
        }
    }
    let mut out = String::with_capacity(text.len());
    for l in text.lines() {
        match serde_json::from_str::<serde_json::Value>(l) {
            Ok(mut v) if l.contains("\"file_name\"") => {
                walk(&mut v, base, at);
                out.push_str(&v.to_string());
            }
            _ => out.push_str(l),
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(name: &str, files: &[(&str, &str)]) -> PathBuf {
        let d = std::env::temp_dir().join(format!("fold-layout-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        for (p, t) in files {
            let f = d.join(p);
            std::fs::create_dir_all(f.parent().unwrap()).unwrap();
            std::fs::write(f, t).unwrap();
        }
        d
    }
    const LIB: &str = "[package]\nname = \"x\"\n";

    #[test]
    fn one_crate_is_found_with_its_users() {
        let d = tree("one", &[
            ("rust/game/Cargo.toml", "[package]\nname = \"game\"\n[dependencies]\nsim = { path = \"../sim\" }\n"),
            ("rust/game/src/knobs.rs", "pub struct Knobs {}"),
            ("rust/sim/src/knobs.rs", "pub const GAIT_LEAN: usize = 0;"),
            ("rust/game/src/doors/p_01.rs", ""),
            ("rust/game/src/doors/mod.rs", ""),
            ("rust/sim/Cargo.toml", "[package]\nname = \"sim\"\n[lib]\ncrate-type = [\"cdylib\", \"rlib\"]\n"),
            ("rust/game_face/Cargo.toml", "[lib]\ncrate-type = [\"cdylib\"]\n[dependencies]\ngame = { path = \"../game\" }\ngame_run = { path = \"../game_run\" }\n"),
            ("rust/game_run/Cargo.toml", "[dependencies]\ngame = { path = \"../game\" }\n"),
            ("rust/game_run/src/bin/mission.rs", ""),
            ("rust/target/x.rs", ""),
        ]);
        let l = find(&d).unwrap();
        assert_eq!(l.game.iter().cloned().collect::<Vec<_>>(), ["game"]);
        assert_eq!(l.bodies, ["game"]);
        assert_eq!(l.users(), ["game_face", "game_run"]);
        assert_eq!(l.crates.iter().filter(|c| c.page).map(|c| c.name.as_str()).collect::<Vec<_>>(), ["game_face"]);
        assert_eq!(l.game_and_users(), ["game", "game_face", "game_run"]);
        assert_eq!(l.knobs, Path::new("rust/game/src/knobs.rs"));
        assert_eq!(l.common(), "rust/game/src/");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_split_game_is_found_by_what_its_crates_hold() {
        let d = tree("split", &[
            ("rust/game_types/Cargo.toml", LIB),
            ("rust/game_types/src/knobs.rs", "pub struct Knobs {}"),
            ("rust/game_types/src/doors/types.rs", ""),
            ("rust/game_types/src/doors/mod.rs", ""),
            ("rust/game_wire/Cargo.toml", "[dependencies]\ngame_types = { path = \"../game_types\" }\n"),
            ("rust/game_wire/src/io.rs", ""),
            ("rust/game_world/Cargo.toml", "[dependencies]\ngame_types = { path = \"../game_types\" }\n"),
            ("rust/game_world/src/scene/p_01.rs", ""),
            ("rust/game_rules/Cargo.toml", "[dependencies]\ngame_types = { path = \"../game_types\" }\ngame_world = { path = \"../game_world\" }\n"),
            ("rust/game_rules/src/doors/p_01.rs", ""),
            ("rust/game/Cargo.toml", "[dependencies]\ngame_rules = { path = \"../game_rules\" }\ngame_wire = { path = \"../game_wire\" }\n"),
            ("rust/game/src/lib.rs", ""),
            ("rust/sim/Cargo.toml", LIB),
            ("rust/game_start/Cargo.toml", "[dependencies]\ngame = { path = \"../game\" }\n"),
            ("rust/game_start/src/bin/hold.rs", ""),
        ]);
        let l = find(&d).unwrap();
        assert_eq!(l.game.iter().cloned().collect::<Vec<_>>(), ["game", "game_rules", "game_types", "game_wire", "game_world"]);
        assert_eq!(l.bodies, ["game_world", "game_rules"]);
        assert_eq!(l.users(), ["game", "game_start"]);
        assert_eq!(l.knobs, Path::new("rust/game_types/src/knobs.rs"));
        assert_eq!(l.common(), "rust/");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_module_with_bodies_in_two_crates_is_refused() {
        let d = tree("two", &[
            ("rust/a/Cargo.toml", LIB),
            ("rust/a/src/knobs.rs", "pub struct Knobs {}"),
            ("rust/a/src/doors/p_01.rs", ""),
            ("rust/b/Cargo.toml", "[dependencies]\na = { path = \"../a\" }\n"),
            ("rust/b/src/doors/p_02.rs", ""),
        ]);
        assert!(find(&d).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_checks_output_names_its_files_from_the_root() {
        let l = r#"{"reason":"compiler-message","message":{"spans":[{"file_name":"src/doors/p_01.rs","expansion":{"span":{"file_name":"/w/repo/rust/sim/src/doors.rs"}}}],"children":[{"spans":[{"file_name":"src/lib.rs"}]}]}}"#;
        let out = diag_from_root(&format!("{l}\nnot json\n"), Path::new("/w/repo"), Path::new("rust/game"));
        assert!(out.contains(r#""file_name":"rust/game/src/doors/p_01.rs""#) && out.contains(r#""file_name":"rust/sim/src/doors.rs""#) && out.contains(r#""file_name":"rust/game/src/lib.rs""#));
        assert!(out.ends_with("not json\n"));
    }

    #[test]
    fn a_file_the_compiler_names_is_read_from_the_root() {
        let base = Path::new("/w/repo");
        assert_eq!(from_root(base, Path::new("rust/game"), "src/doors/p_01.rs"), "rust/game/src/doors/p_01.rs");
        assert_eq!(from_root(base, Path::new("rust/game_face"), "/w/repo/rust/game/src/doors/p_01.rs"), "rust/game/src/doors/p_01.rs");
        assert_eq!(from_root(base, Path::new("rust/game_face"), "../game/src/doors/p_01.rs"), "rust/game/src/doors/p_01.rs");
    }
}
