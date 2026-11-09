//! `gate tests [BINARY [TEST]]`: THE WHOLE NATIVE TEST SET BY ONE COMMAND. No kind of the quick gate: it builds the tests of every crate that has any, and runs each binary as a process of its own, from its
//! crate's folder (tests read ../../levels by that), as many at once as there are cores. Exit 0 = all pass · 1 = a red test, a binary that ended badly or a build that failed · 64 = a wrong word.
//! `--break=BINARY` plants a red in what that binary said, to show that a red is read and the exit is 1.
use std::collections::VecDeque;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Instant;

/// EVERY CRATE OF rust/ WITH TESTS, in the order they are built. A crate with a tests folder or a #[test] that is not here fails the run by name.
const CRATES: [&str; 6] = ["game_start", "game_run", "game_types", "game_rules", "game_draw", "sim"];

fn rust() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..") }

struct Binary { krate: String, name: String, path: String }
/// what one binary said
struct Told { krate: String, name: String, secs: f64, passed: u32, failed: u32, ignored: u32, reds: Vec<(String, String)>, ignores: Vec<(String, String)>, bad: Option<String> }

/// does this folder's source hold a test
fn has_test(dir: &std::path::Path) -> bool {
    let Ok(list) = std::fs::read_dir(dir) else { return false };
    list.flatten().any(|e| { let p = e.path(); if p.is_dir() { has_test(&p) } else { p.extension().is_some_and(|x| x == "rs") && std::fs::read_to_string(&p).is_ok_and(|t| t.contains("#[test]")) } })
}
/// the crates with tests that the list leaves out
fn left_out() -> Vec<String> {
    let Ok(list) = std::fs::read_dir(rust()) else { return Vec::new() };
    let mut out: Vec<String> = list.flatten().filter(|e| e.path().join("Cargo.toml").exists() && (e.path().join("tests").is_dir() || has_test(&e.path().join("src")))).map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| !CRATES.contains(&n.as_str())).collect();
    out.sort();
    out
}

/// a crate's tests built, and its binaries read off cargo's `Executable … (path)` lines. Err: the first lines of the build's failure.
fn build(krate: &str) -> Result<Vec<Binary>, String> {
    // (`CARGO` is what cargo itself calls the cargo to use)
    let o = Command::new(std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo"))).args(["test", "--no-run"]).current_dir(rust().join(krate)).stdin(Stdio::null()).output().map_err(|e| format!("cargo: {}", e))?;
    let err = String::from_utf8_lossy(&o.stderr);
    if !o.status.success() { return Err(err.lines().filter(|l| l.starts_with("error")).take(3).collect::<Vec<_>>().join(" | ")); }
    Ok(err.lines().filter_map(|l| {
        let l = l.trim_start().strip_prefix("Executable ")?;
        let path = l[l.rfind('(')? + 1..l.rfind(')')?].to_string();
        let file = path.rsplit('/').next()?;
        Some(Binary { krate: krate.to_string(), name: file[..file.rfind('-')?].to_string(), path })
    }).collect())
}

/// what a binary wrote, read: libtest's own lines
fn read(b: &Binary, secs: f64, out: &str, ok: bool, err: &str) -> Told {
    let mut t = Told { krate: b.krate.clone(), name: b.name.clone(), secs, passed: 0, failed: 0, ignored: 0, reds: Vec::new(), ignores: Vec::new(), bad: None };
    let mut lines = out.lines().peekable();
    let mut summed = false;
    while let Some(l) = lines.next() {
        if let Some(rest) = l.strip_prefix("test ") {
            if let Some((name, said)) = rest.split_once(" ... ") { if let Some(why) = said.strip_prefix("ignored") { t.ignores.push((name.to_string(), why.trim_start_matches(", ").to_string())); } }
        }
        if let Some(name) = l.strip_prefix("---- ").and_then(|x| x.strip_suffix(" stdout ----")) {
            let mut why = String::from("no panic line");
            while let Some(m) = lines.peek() {
                if m.starts_with("---- ") || m.starts_with("failures:") { break; }
                let m = lines.next().unwrap();
                if m.contains("panicked at") { why = format!("{} {}", m.split("panicked at ").nth(1).unwrap_or(""), lines.next().unwrap_or("")); break; }
            }
            t.reds.push((name.to_string(), why));
        }
        if let Some(rest) = l.strip_prefix("test result: ") {
            summed = true;
            let count = |what: &str| rest.split("; ").find_map(|p| p.strip_suffix(what)?.rsplit(' ').next()?.parse::<u32>().ok()).unwrap_or(0);
            (t.passed, t.failed, t.ignored) = (count(" passed"), count(" failed"), count(" ignored"));
        }
    }
    if !ok && t.failed == 0 || !summed {
        let last: Vec<&str> = err.lines().rev().filter(|l| !l.trim().is_empty()).take(2).collect();
        t.bad = Some(if last.is_empty() { String::from("it said nothing") } else { last.into_iter().rev().collect::<Vec<_>>().join(" | ") });
    }
    t
}

fn one(b: &Binary, filter: Option<&str>, planted: bool) -> Told {
    let t = Instant::now();
    let mut c = Command::new(&b.path);
    if let Some(f) = filter { c.arg(f); }
    let o = c.current_dir(rust().join(&b.krate)).env("RUST_BACKTRACE", "0").stdin(Stdio::null()).output();
    let secs = t.elapsed().as_secs_f64();
    let Ok(o) = o else { let mut told = read(b, secs, "", false, ""); told.bad = Some(String::from("it would not start")); return told };
    let (mut out, err) = (String::from_utf8_lossy(&o.stdout).into_owned(), String::from_utf8_lossy(&o.stderr).into_owned());
    if planted { out = out.replace("test result: ok.", "test result: FAILED.").replace(" 0 failed", " 1 failed") + "\n---- planted stdout ----\n\nthread 'planted' panicked at the --break word:\na red planted in what the binary said\n\nfailures:\n"; }
    read(b, secs, &out, o.status.success() && !planted, &err)
}

/// THE VERB
pub fn run(words: &[String]) -> i32 {
    let planted = words.iter().find_map(|w| w.strip_prefix("--break=")).map(String::from);
    let plain: Vec<&String> = words.iter().filter(|w| !w.starts_with("--")).collect();
    if plain.len() > 2 || words.iter().any(|w| w.starts_with("--") && !w.starts_with("--break=")) { eprintln!("gate tests [BINARY [TEST]] [--break=BINARY]"); return 64; }
    let (only, test) = (plain.first().map(|s| s.as_str()), plain.get(1).map(|s| s.as_str()));
    let wall = Instant::now();
    let unlisted = left_out();
    let (mut binaries, mut broken): (Vec<Binary>, Vec<String>) = (Vec::new(), Vec::new());
    for krate in CRATES {
        match build(krate) {
            Ok(list) => binaries.extend(list),
            Err(why) => { println!("FAIL  build of {}: {}", krate, why); broken.push(krate.to_string()); }
        }
        // (a binary named is found in the first crate that has it: the others need not be built)
        if only.is_some_and(|n| binaries.iter().any(|b| b.name == n)) { break; }
    }
    let built = wall.elapsed().as_secs_f64();
    if let Some(n) = only { binaries.retain(|b| b.name == n); if binaries.is_empty() { eprintln!("no test binary is called '{}'", n); return if broken.is_empty() { 64 } else { 1 }; } }
    let count = binaries.len();
    let queue = Arc::new(Mutex::new(binaries.into_iter().collect::<VecDeque<Binary>>()));
    let (tx, rx) = mpsc::channel();
    for _ in 0..std::thread::available_parallelism().map_or(4, |n| n.get()) {
        let (queue, tx, test, planted) = (queue.clone(), tx.clone(), test.map(String::from), planted.clone());
        std::thread::spawn(move || loop {
            let Some(b) = queue.lock().unwrap().pop_front() else { return };
            let _ = tx.send(one(&b, test.as_deref(), planted.as_deref() == Some(b.name.as_str())));
        });
    }
    let mut told: Vec<Told> = Vec::new();
    for _ in 0..count {
        let t = rx.recv().expect("the binaries' threads");
        // (a binary with no test, such as a tool's, says nothing worth a line)
        if t.passed + t.failed + t.ignored > 0 || t.bad.is_some() {
            println!("{}  {} {} · {} passed{}{} · {:.1} s", if t.failed > 0 || t.bad.is_some() { "FAIL" } else { "ok  " }, t.krate, t.name, t.passed, if t.failed > 0 { format!(", {} failed", t.failed) } else { String::new() }, if t.ignored > 0 { format!(", {} ignored", t.ignored) } else { String::new() }, t.secs);
        }
        told.push(t);
    }
    if test.is_some() && told.iter().all(|t| t.passed + t.failed + t.ignored == 0) { eprintln!("no test of that binary has that in its name"); return 64; }
    for t in &told {
        for (name, why) in &t.reds { println!("RED   {} {}: {}", t.name, name, why); }
        if let Some(why) = &t.bad { println!("RED   {}: it ended badly: {}", t.name, why); }
    }
    for t in &told { for (name, why) in &t.ignores { println!("IGNORED  {} {}: {}", t.name, name, if why.is_empty() { "no reason given" } else { why }); } }
    told.sort_by(|a, b| b.secs.total_cmp(&a.secs));
    for t in told.iter().take(10) { println!("slow  {:.1} s  {} {}", t.secs, t.krate, t.name); }
    for u in &unlisted { println!("FAIL  {} has tests and is not on the list of crates (src/bin/gate/tests.rs)", u); }
    let (passed, failed, ignored) = told.iter().fold((0, 0, 0), |n, t| (n.0 + t.passed, n.1 + t.failed, n.2 + t.ignored));
    let ended_badly = told.iter().filter(|t| t.bad.is_some()).count();
    println!("gate tests: {} binaries · {} passed, {} failed, {} ignored{} · built in {:.0} s · {:.0} s summed, {:.0} s wall", count, passed, failed, ignored, if ended_badly > 0 { format!(", {} ended badly", ended_badly) } else { String::new() }, built, told.iter().map(|t| t.secs).sum::<f64>(), wall.elapsed().as_secs_f64());
    if failed > 0 || ended_badly > 0 || !broken.is_empty() || !unlisted.is_empty() { 1 } else { 0 }
}
