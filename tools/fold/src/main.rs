//! fold: make settled switch rows constants of the Rust game.
//!
//!   cargo run --release --manifest-path tools/fold/Cargo.toml -- [--write] [--report FILE] [--settle FILE] [--table FILE]
//!
//! Reads tools/qa/switches.json (every switch row, its page value, its kind) and rust/game/src/knobs.rs (the
//! tables, at page values), then every body file of rust/game/src (`<module>/p_NN.rs`, `<module>/b_NN.rs`).
//! Where a body reads a SETTLED yes-or-no row, the read becomes the page value, and what that settles is
//! folded: the `!`, the `&&` and `||`, the `if` (the side kept stands in its place, the other is deleted), a
//! `let` that named the value, a function that is now one yes or no or does nothing (at its callers), and then
//! every function nothing calls any more. Rows of the other kinds are not touched. README.md has the rules.
//!
//! Without `--write` nothing is changed: the report says what would be.
//! `--unnamed` also deletes the functions nothing named before the fold (by default only what the fold left unnamed goes).
//! `--settle FILE` names more rows to take as settled at their page value (one `TABLE.row` a line).
//! `--table FILE` folds nothing for good: it tries each row that is NOT settled, alone, on top of the fold,
//! and writes what each would take with it.

mod dead;
mod fields;
mod fold;
mod readers;
mod rows;
mod sharp;

use fold::{FnFact, FnTable, Pass, Report, MARK};
use rows::{Kind, Rows};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::path::PathBuf;
use syn::visit::Visit;

struct Body {
    path: PathBuf,
    /// the path from the repository's root, for the report
    rel: String,
    module: String,
    was: String,
}

fn merge(into: &mut Report, from: Report) {
    for (k, v) in from.reads {
        *into.reads.entry(k).or_default() += v;
    }
    for (k, v) in from.other_reads {
        *into.other_reads.entry(k).or_default() += v;
    }
    for (k, v) in from.read_files {
        let m = into.read_files.entry(k).or_default();
        for (f, n) in v {
            *m.entry(f).or_default() += n;
        }
    }
    for (k, v) in from.writes {
        into.writes.entry(k).or_default().extend(v);
    }
    for (k, v) in from.table_writes {
        into.table_writes.entry(k).or_default().extend(v);
    }
    into.left.extend(from.left);
    for (k, v) in from.leaf_reads {
        *into.leaf_reads.entry(k).or_default() += v;
    }
    for (k, v) in from.table_uses {
        *into.table_uses.entry(k).or_default() += v;
    }
}

struct Engine<'a> {
    rows: &'a Rows,
    bodies: &'a [Body],
    modules: Vec<String>,
    /// each body's part name (`p_02`), as its module's mod.rs names it
    stems: Vec<String>,
    /// module -> function name -> how many body functions of the module bear it
    mfns: HashMap<String, HashMap<String, usize>>,
    /// functions that are plain readings (fold::pure_fns_of), to a fixed point over the tree as it stood
    pure: HashSet<(String, String)>,
    world: dead::World,
}

#[derive(Default)]
struct Outcome {
    /// body index -> its text after the run (only the ones that changed)
    texts: HashMap<usize, String>,
    modrs: HashMap<String, String>,
    fns: FnTable,
    deleted: Vec<(String, String)>,
    surface_only: Vec<(String, String)>,
    rule_counts: BTreeMap<&'static str, usize>,
    passes: usize,
    /// reads the last pass met and could not take
    left: Vec<String>,
    /// lines where a value the fold wrote stands as a value
    values: Vec<String>,
}

impl<'a> Engine<'a> {
    /// One pass over the given bodies: count, and say what is written. Nothing is changed.
    fn scan(&self, texts: &[String], take: &HashSet<String>, fns: &FnTable, only: Option<&[usize]>) -> Report {
        let mut rep = Report::default();
        let all: Vec<usize> = (0..texts.len()).collect();
        for &i in only.unwrap_or(&all) {
            let b = &self.bodies[i];
            let file = match syn::parse_file(&texts[i]) {
                Ok(f) => f,
                Err(e) => {
                    rep.left.push(format!("{} does not parse: {e}", b.rel));
                    continue;
                }
            };
            let mut p = Pass::new(&texts[i], &b.rel, &b.module, self.rows, take, fns, &self.mfns, &self.pure, true);
            p.visit_file(&file);
            merge(&mut rep, p.report);
        }
        rep
    }

    /// Fold the rows of `take` in `texts` to a fixed point, then delete the functions that left uncalled.
    /// `start` = the bodies that read a taken row, where the caller knows them (else every body is walked).
    fn fold(&self, texts: &mut Vec<String>, take: &HashSet<String>, start: Option<Vec<usize>>, base: &dead::Cache, spare: &HashSet<(String, String)>) -> Outcome {
        let mut out = Outcome::default();
        let mut changed: BTreeSet<usize> = BTreeSet::new();
        let mut work: Vec<usize> = start.unwrap_or_else(|| (0..texts.len()).collect());
        loop {
            for &i in &work {
                let b = &self.bodies[i];
                loop {
                    let file = match syn::parse_file(&texts[i]) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("fold: {} no longer parses after a pass: {e} (the text is in /tmp/fold-broken.rs)", b.rel);
                            let _ = std::fs::write("/tmp/fold-broken.rs", &texts[i]);
                            std::process::exit(2);
                        }
                    };
                    let mut p = Pass::new(&texts[i], &b.rel, &b.module, self.rows, take, &out.fns, &self.mfns, &self.pure, false);
                    p.visit_file(&file);
                    let edits = fold::innermost(std::mem::take(&mut p.edits));
                    if edits.is_empty() {
                        break;
                    }
                    for e in &edits {
                        *out.rule_counts.entry(e.rule).or_default() += 1;
                    }
                    let next = fold::apply(&texts[i], &edits);
                    drop(p);
                    texts[i] = next;
                    changed.insert(i);
                    out.passes += 1;
                }
            }
            // what the functions have become, and who calls the ones that changed
            let mut fresh: Vec<String> = Vec::new();
            for &i in &changed {
                if !texts[i].contains(MARK) {
                    continue;
                }
                if let Ok(f) = syn::parse_file(&texts[i]) {
                    for (name, fact) in fold::fn_facts(&texts[i], &f) {
                        let key = (self.bodies[i].module.clone(), name.clone());
                        if out.fns.get(&key) != Some(&fact) {
                            out.fns.insert(key, fact);
                            fresh.push(name);
                        }
                    }
                }
            }
            if fresh.is_empty() {
                break;
            }
            work = (0..texts.len()).filter(|&i| fresh.iter().any(|n| texts[i].contains(n.as_str()))).collect();
        }
        let ch: Vec<usize> = changed.iter().copied().collect();
        let after = self.scan(texts, take, &out.fns, Some(&ch));
        out.left = after.left;
        for &i in &ch {
            for (n, l) in texts[i].lines().enumerate() {
                if l.contains(&format!("true{MARK}")) || l.contains(&format!("false{MARK}")) {
                    let shown: String = l.trim().replace(MARK, "").chars().take(200).collect();
                    out.values.push(format!("{}:{} {}", self.bodies[i].rel, n + 1, shown));
                }
            }
            texts[i] = strip_marks(&texts[i]);
        }
        let s = dead::fixpoint(&self.world, base, &self.modules, &self.stems, texts, &ch, spare, true);
        for (i, t) in texts.iter().enumerate() {
            if *t != self.bodies[i].was {
                out.texts.insert(i, t.clone());
            }
        }
        out.deleted = s.deleted;
        out.surface_only = s.surface_only;
        out.modrs = s.modrs;
        out
    }
}

/// The marks out, and what they leave behind them tidied: a block that held only a mark is `{}`.
fn strip_marks(s: &str) -> String {
    if !s.contains(MARK) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    for l in s.split_inclusive('\n') {
        if l.contains(MARK) {
            let t = l.replace(MARK, "");
            if t.trim().is_empty() {
                continue;
            }
            out.push_str(&t.replace("{ }", "{}"));
        } else {
            out.push_str(l);
        }
    }
    out
}

fn lines_of(t: &str) -> usize {
    t.lines().count()
}

/// `fold sharp-prep --scratch DIR [--no-tests]` and `fold sharp-apply --diag FILE [--write] [--report FILE]`:
/// the compiler's dead-code reading (sharp.rs; tools/fold/sharp.sh drives the two to a fixed point).
fn sharp_main(cmd: &str, args: &[String]) -> ! {
    let mut root = PathBuf::from(".");
    let mut scratch = std::env::temp_dir().join("fold-sharp");
    let mut diag: Option<String> = None;
    let mut report: Option<String> = None;
    let mut write = false;
    let mut tests = true;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().unwrap_or_default();
        match a.as_str() {
            "--root" => root = PathBuf::from(val()),
            "--scratch" => scratch = PathBuf::from(val()),
            "--diag" => diag = Some(val()),
            "--report" => report = Some(val()),
            "--write" => write = true,
            "--no-tests" => tests = false,
            x => {
                eprintln!("fold {cmd}: no option {x}");
                std::process::exit(64);
            }
        }
    }
    if cmd == "sharp-prep" {
        match sharp::prep(&root, &scratch, tests) {
            Ok(s) => {
                println!("fold: {s}");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("fold: {e}");
                std::process::exit(3);
            }
        }
    }
    let Some(d) = diag else {
        eprintln!("fold sharp-apply: --diag FILE (the output of `cargo check --message-format=json` on the private copy)");
        std::process::exit(64);
    };
    let text = std::fs::read_to_string(&d).unwrap_or_else(|e| {
        eprintln!("fold: cannot read {d}: {e}");
        std::process::exit(3);
    });
    let named = sharp::read_diag(&text);
    match sharp::apply(&root, &named, write) {
        Ok(a) => {
            let mut o = String::new();
            let mut by_kind: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
            let mut by_module: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
            for (name, kind, n) in &a.deleted {
                let e = by_kind.entry(kind.as_str()).or_default();
                e.0 += 1;
                e.1 += n;
                let e = by_module.entry(name.split("::").next().unwrap_or("")).or_default();
                e.0 += 1;
                e.1 += n;
            }
            let _ = writeln!(o, "{} items, {} lines{}", a.deleted.len(), a.lines, if write { "" } else { " (nothing written: --write)" });
            for (k, (n, l)) in &by_kind {
                let _ = writeln!(o, "- {k}: {n}, {l} lines");
            }
            let _ = writeln!(o, "\n## By module\n");
            let mut v: Vec<_> = by_module.into_iter().collect();
            v.sort_by_key(|(_, (_, l))| std::cmp::Reverse(*l));
            for (m, (n, l)) in v {
                let _ = writeln!(o, "- {m}: {n} items, {l} lines");
            }
            let _ = writeln!(o, "\n## Deleted\n");
            let mut d: Vec<String> = a.deleted.iter().map(|(n, k, l)| format!("- {n} ({k}, {l} lines)")).collect();
            d.sort();
            let _ = writeln!(o, "{}", d.join("\n"));
            let _ = writeln!(o, "\n## Named dead by the compiler and not deleted here: fields, variants, items of files this pass does not edit ({})\n", a.listed.len());
            let _ = writeln!(o, "{}", a.listed.iter().map(|l| format!("- {l}")).collect::<Vec<_>>().join("\n"));
            match report {
                Some(f) => {
                    let _ = std::fs::write(&f, &o);
                    println!("{}", o.lines().take(8).collect::<Vec<_>>().join("\n"));
                }
                None => print!("{o}"),
            }
            std::process::exit(if a.deleted.is_empty() { 0 } else { 10 });
        }
        Err(e) => {
            eprintln!("fold: {e}");
            std::process::exit(3);
        }
    }
}

/// `fold readers-demote | readers-restore [--diag FILE] | readers-report`, each with `--state FILE`:
/// whether a function writes, said by its signature (readers.rs; tools/fold/readers.sh drives them).
fn readers_main(cmd: &str, args: &[String]) -> ! {
    let mut root = PathBuf::from(".");
    let mut state = std::env::temp_dir().join("fold-readers.txt");
    let mut diag: Option<String> = None;
    let mut taint = false;
    let mut bare: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().unwrap_or_default();
        match a.as_str() {
            "--root" => root = PathBuf::from(val()),
            "--state" => state = PathBuf::from(val()),
            "--diag" => diag = Some(val()),
            "--taint" => taint = true,
            "--bare" => bare = Some(val()),
            x => {
                eprintln!("fold {cmd}: no option {x}");
                std::process::exit(64);
            }
        }
    }
    let fail = |e: String| -> ! {
        eprintln!("fold: {e}");
        std::process::exit(3)
    };
    match cmd {
        "readers-demote" => match readers::demote(&root, &state) {
            Ok(s) => println!("fold: {s}"),
            Err(e) => fail(e),
        },
        "readers-restore" => {
            let text = diag.map(|d| std::fs::read_to_string(&d).unwrap_or_else(|e| fail(format!("cannot read {d}: {e}"))));
            match readers::restore(&root, &state, text.as_deref(), taint) {
                Ok(r) => {
                    println!("fold: `mut` put back on {} parameters the compiler named, {} that write past the compiler's sight, {} more by the calls; {} stay `&`", r.by_error, r.by_taint, r.by_calls, r.left);
                    for u in r.unplaced.iter().take(12) {
                        println!("  an error this program could not place: {u}");
                    }
                    std::process::exit(if r.by_error + r.by_taint + r.by_calls > 0 { 10 } else if r.unplaced.is_empty() { 0 } else { 4 });
                }
                Err(e) => fail(e),
            }
        }
        "readers-report" => {
            let b = bare.map(|f| std::fs::read_to_string(&f).unwrap_or_else(|e| fail(format!("cannot read {f}: {e}"))));
            match readers::report(&root, &state, b.as_deref()) {
                Ok(s) => print!("{s}"),
                Err(e) => fail(e),
            }
        }
        "readers-now" => match readers::readers_now(&root) {
            Ok(v) => println!("{}", v.join("\n")),
            Err(e) => fail(e),
        },
        x => {
            eprintln!("fold: no command {x}");
            std::process::exit(64);
        }
    }
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(c) = args.first() {
        if c == "sharp-prep" || c == "sharp-apply" {
            sharp_main(c, &args[1..]);
        }
        if c.starts_with("readers-") {
            readers_main(c, &args[1..]);
        }
    }
    let mut write = false;
    let mut report_to: Option<String> = None;
    let mut settle_file: Option<String> = None;
    let mut table_to: Option<String> = None;
    let mut knobs_to: Option<String> = None;
    let mut unnamed = false;
    let mut root = PathBuf::from(".");
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().unwrap_or_default();
        match a.as_str() {
            "--write" => write = true,
            "--unnamed" => unnamed = true,
            "--report" => report_to = Some(val()),
            "--settle" => settle_file = Some(val()),
            "--table" => table_to = Some(val()),
            "--knobs" => knobs_to = Some(val()),
            "--root" => root = PathBuf::from(val()),
            x => {
                eprintln!("fold: no option {x}");
                std::process::exit(64);
            }
        }
    }
    let src_dir = root.join("rust/game/src");
    let read = |p: PathBuf| {
        std::fs::read_to_string(&p).unwrap_or_else(|e| {
            eprintln!("fold: cannot read {}: {e}", p.display());
            std::process::exit(3);
        })
    };
    let rows = match rows::load(&read(root.join("tools/qa/switches.json")), &read(src_dir.join("knobs.rs"))) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("fold: {e}");
            std::process::exit(3);
        }
    };
    let by_name: HashMap<&str, &String> = rows.by_path.iter().map(|(p, r)| (r.name.as_str(), p)).collect();
    // tools/fold/keep.txt: rows a hand keeps out of the fold (`TABLE.row  why`), and writes a hand has read and
    // found to be no reason to keep a row (`written-ok TABLE.row  why`)
    let mut kept_by_hand: BTreeMap<String, String> = BTreeMap::new();
    let mut written_ok: BTreeMap<String, String> = BTreeMap::new();
    if let Ok(t) = std::fs::read_to_string(root.join("tools/fold/keep.txt")) {
        for l in t.lines() {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let (ok, rest) = match l.strip_prefix("written-ok ") {
                Some(r) => (true, r.trim()),
                None => (false, l),
            };
            let (name, why) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
            // a row by its name, or a whole table by its (`PRESSURE`: every row of it)
            let pre = format!("{name}.");
            let paths: Vec<String> = match by_name.get(name) {
                Some(p) => vec![(*p).clone()],
                None => by_name.iter().filter(|(n, _)| n.starts_with(&pre)).map(|(_, p)| (*p).clone()).collect(),
            };
            if paths.is_empty() {
                eprintln!("fold: keep.txt names {name}, which is no row and no table of the Rust tables");
                std::process::exit(3);
            }
            for p in paths {
                if ok { written_ok.insert(p, why.trim().to_string()) } else { kept_by_hand.insert(p, why.trim().to_string()) };
            }
        }
    }
    // rows told settled on top of the list's own
    let mut told: BTreeSet<String> = BTreeSet::new();
    if let Some(f) = &settle_file {
        for l in read(PathBuf::from(f)).lines() {
            let name = l.split_whitespace().next().unwrap_or("");
            if name.is_empty() || name.starts_with('#') {
                continue;
            }
            match by_name.get(name) {
                Some(p) => {
                    told.insert((*p).clone());
                }
                None => {
                    eprintln!("fold: --settle names {name}, which is no row of the Rust tables");
                    std::process::exit(3);
                }
            }
        }
    }

    let mut all = Vec::new();
    dead::rs_files(&src_dir, &mut all);
    let mut bodies: Vec<Body> = Vec::new();
    for p in &all {
        let n = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !((n.starts_with("p_") || n.starts_with("b_")) && n.ends_with(".rs")) {
            continue;
        }
        let module = p.parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string();
        let rel = p.strip_prefix(&root).unwrap_or(p).to_string_lossy().to_string();
        bodies.push(Body { path: p.clone(), rel, module, was: read(p.clone()) });
    }
    let mut mfns: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for b in &bodies {
        if let Ok(f) = syn::parse_file(&b.was) {
            for it in &f.items {
                if let syn::Item::Fn(x) = it {
                    *mfns.entry(b.module.clone()).or_default().entry(x.sig.ident.to_string()).or_default() += 1;
                }
            }
        }
    }
    let body_paths: HashSet<PathBuf> = bodies.iter().map(|b| b.path.clone()).collect();
    // which functions are plain readings: found over and over until no more are (one that calls another waits for it)
    let mut pure: HashSet<(String, String)> = HashSet::new();
    {
        let none_taken: HashSet<String> = HashSet::new();
        let no_facts: FnTable = HashMap::new();
        let parsed: Vec<Option<syn::File>> = bodies.iter().map(|b| syn::parse_file(&b.was).ok()).collect();
        loop {
            let mut grew = false;
            for (b, f) in bodies.iter().zip(parsed.iter()) {
                let Some(f) = f else { continue };
                let mut p = Pass::new(&b.was, &b.rel, &b.module, &rows, &none_taken, &no_facts, &mfns, &pure, true);
                let names = fold::pure_fns_of(f, &mut p);
                drop(p);
                for n in names {
                    grew |= pure.insert((b.module.clone(), n));
                }
            }
            if !grew {
                break;
            }
        }
    }
    let eng = Engine { rows: &rows, bodies: &bodies, modules: bodies.iter().map(|b| b.module.clone()).collect(), stems: bodies.iter().map(|b| b.path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()).collect(), mfns, pure, world: dead::world(&root, &body_paths) };
    let pristine: Vec<String> = bodies.iter().map(|b| b.was.clone()).collect();
    let no_fns: FnTable = HashMap::new();

    // 1. what is read and what is written, as the tree stands
    let is_bool = |p: &String| rows.by_path[p].bool_value.is_some();
    let wanted: HashSet<String> = rows.by_path.iter().filter(|(p, r)| (r.kind == Kind::Settled || told.contains(*p)) && is_bool(p)).map(|(p, _)| p.clone()).collect();
    let before = eng.scan(&pristine, &wanted, &no_fns, None);
    // A row a body WRITES is no constant: it is kept out, and so is every wanted row of a table a body borrows
    // `&mut` or assigns whole, unless keep.txt says a hand read that write (`written-ok`).
    let written = |rep: &Report, pool: &HashSet<String>| -> BTreeMap<String, Vec<String>> {
        let mut f: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (p, at) in &rep.writes {
            if pool.contains(p) && !written_ok.contains_key(p) {
                f.entry(p.clone()).or_default().extend(at.iter().cloned());
            }
        }
        for (t, at) in &rep.table_writes {
            let pre = format!("{t}.");
            for p in pool {
                if (t.is_empty() || p.starts_with(&pre)) && !written_ok.contains_key(p) {
                    f.entry(p.clone()).or_default().extend(at.iter().map(|a| format!("(its table) {a}")));
                }
            }
        }
        f
    };
    let all_settled: HashSet<String> = rows.by_path.iter().filter(|(p, r)| r.kind == Kind::Settled || told.contains(*p)).map(|(p, _)| p.clone()).collect();
    let findings = written(&before, &all_settled);
    let take: HashSet<String> = wanted.iter().filter(|p| !findings.contains_key(*p) && !kept_by_hand.contains_key(*p)).cloned().collect();

    // 2. the fold
    let base = dead::cache(&eng.world, &pristine);
    let dead_before: HashSet<(String, String)> = {
        let mut t = pristine.clone();
        dead::fixpoint(&eng.world, &base, &eng.modules, &eng.stems, &mut t, &[], &HashSet::new(), true).deleted.into_iter().collect()
    };
    let surface_before: HashSet<(String, String)> = dead::fixpoint(&eng.world, &base, &eng.modules, &eng.stems, &mut pristine.clone(), &[], &HashSet::new(), false).surface_only.into_iter().collect();
    let mut texts = pristine.clone();
    // with --unnamed the functions nothing named BEFORE the fold go too
    let spare: HashSet<(String, String)> = if unnamed { HashSet::new() } else { dead_before.clone() };
    let out = eng.fold(&mut texts, &take, None, &base, &spare);

    // numbers
    let lines_before: usize = pristine.iter().map(|t| lines_of(t)).sum();
    let lines_after: usize = texts.iter().map(|t| lines_of(t)).sum();
    let settled_all: Vec<&String> = rows.by_path.iter().filter(|(p, r)| r.kind == Kind::Settled || told.contains(*p)).map(|(k, _)| k).collect();
    let reads_before: usize = before.reads.values().sum();
    let taken_reads: usize = before.reads.iter().filter(|(p, _)| take.contains(*p)).map(|(_, n)| n).sum();
    let taken_rows = before.reads.keys().filter(|p| take.contains(*p)).count();
    let number_rows: Vec<&String> = settled_all.iter().filter(|p| !is_bool(p)).copied().collect();

    let mut o = String::new();
    let _ = writeln!(o, "# The settled rows, folded: the report\n");
    let _ = writeln!(o, "rows in the list: {} ({} of them settled and in the Rust tables{}; {} of any kind have no field there)", rows.by_path.len() + rows.not_in_rust.len(), settled_all.len(), if told.is_empty() { String::new() } else { format!(", {} of those told settled by --settle", told.len()) }, rows.not_in_rust.len());
    let _ = writeln!(o, "settled yes-or-no rows a body read before: {} rows, {reads_before} reads", before.reads.len());
    let _ = writeln!(o, "  folded: {taken_rows} rows, {taken_reads} reads");
    let wr = findings.keys().filter(|p| before.reads.contains_key(*p)).count();
    let _ = writeln!(o, "  kept, because a body WRITES the row: {} rows ({wr} of them read too, {} reads; {} written and never read)", findings.len(), findings.keys().filter_map(|p| before.reads.get(p)).sum::<usize>(), findings.len() - wr);
    let _ = writeln!(o, "  kept by hand (keep.txt): {}", kept_by_hand.len());
    let _ = writeln!(o, "settled rows that are numbers or names, not yes or no (left as they are): {}, {} of them read by a body", number_rows.len(), number_rows.iter().filter(|p| before.other_reads.contains_key(**p)).count());
    for (k, name) in [(Kind::Open, "open decision"), (Kind::Player, "the player's or run-time")] {
        let rs: Vec<&String> = rows.by_path.iter().filter(|(p, r)| r.kind == k && !told.contains(*p)).map(|(p, _)| p).collect();
        let read: Vec<usize> = rs.iter().filter_map(|p| before.other_reads.get(*p).copied()).collect();
        let _ = writeln!(o, "rows that stay switches, {name}: {} in the Rust tables, {} of them read by a body, {} reads", rs.len(), read.len(), read.iter().sum::<usize>());
    }
    let _ = writeln!(o, "body files: {} ({} changed); lines {lines_before} -> {lines_after} ({} fewer)", bodies.len(), out.texts.len(), lines_before as i64 - lines_after as i64);
    let _ = writeln!(o, "functions deleted: {}; mod.rs files changed: {}", out.deleted.len(), out.modrs.len());
    let _ = writeln!(o, "functions found to be plain readings (a test may be dropped after a call of one): {}", eng.pure.len());
    let _ = writeln!(o, "\n## Lines by module, where they changed (before -> after)\n");
    {
        let mut by: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
        for (i, b) in bodies.iter().enumerate() {
            let e = by.entry(b.module.as_str()).or_default();
            e.0 += lines_of(&pristine[i]);
            e.1 += lines_of(&texts[i]);
        }
        let mut v: Vec<(&str, usize, usize)> = by.into_iter().filter(|(_, (a, b))| a != b).map(|(m, (a, b))| (m, a, b)).collect();
        v.sort_by_key(|(_, a, b)| std::cmp::Reverse(a - b));
        for (m, a, b) in v {
            let _ = writeln!(o, "- {m}: {a} -> {b} ({} fewer, {}%)", a - b, (a - b) * 100 / a.max(1));
        }
    }
    let _ = writeln!(o, "\n## Edits by rule\n");
    for (r, n) in &out.rule_counts {
        let _ = writeln!(o, "- {n} · {r}");
    }
    let _ = writeln!(o, "\n## Functions the fold settled whole ({})\n", out.fns.len());
    let mut fl: Vec<String> = out.fns.iter().map(|((m, n), f)| format!("- {m}::{n}: {}", match f { FnFact::NoOp => "does nothing".to_string(), FnFact::Const(b) => format!("is {b}") })).collect();
    fl.sort();
    for l in fl {
        let _ = writeln!(o, "{l}");
    }
    let del: BTreeSet<String> = out.deleted.iter().map(|(m, n)| format!("{m}::{n}")).collect();
    let was_dead: BTreeSet<String> = dead_before.iter().map(|(m, n)| format!("{m}::{n}")).collect();
    let _ = writeln!(o, "\n## Functions the fold left with nothing naming them, deleted ({})\n", del.difference(&was_dead).count());
    for d in del.difference(&was_dead) {
        let _ = writeln!(o, "- {d}");
    }
    if unnamed {
        let _ = writeln!(o, "\n## Functions nothing named before the fold, deleted (--unnamed) ({})\n", del.intersection(&was_dead).count());
        let _ = writeln!(o, "{}", del.intersection(&was_dead).cloned().collect::<Vec<_>>().join(", "));
    }
    let only: BTreeSet<String> = out.surface_only.iter().filter(|k| !surface_before.contains(*k)).map(|(m, n)| format!("{m}::{n}")).collect();
    let _ = writeln!(o, "\n## Functions only the tools' call surface names since the fold, kept ({})\n", only.len());
    for d in &only {
        let _ = writeln!(o, "- {d}");
    }
    let _ = writeln!(o, "\n## FINDINGS: settled rows a body writes ({})\n\nKept as they are. Each is either not settled after all, or its write is dead and goes by hand (then `written-ok` in keep.txt).\n", findings.len());
    for (p, at) in &findings {
        let _ = writeln!(o, "- {} ({p}), read {} times", rows.by_path[p].name, before.reads.get(p).or(before.other_reads.get(p)).copied().unwrap_or(0));
        let uniq: BTreeSet<&String> = at.iter().collect();
        for a in uniq.iter().take(8) {
            let _ = writeln!(o, "    - {a}");
        }
    }
    if !kept_by_hand.is_empty() {
        let _ = writeln!(o, "\n## Kept by hand (keep.txt)\n");
        for (p, why) in &kept_by_hand {
            let _ = writeln!(o, "- {}: {why}", rows.by_path[p].name);
        }
    }
    let mut left = out.left.clone();
    left.sort();
    left.dedup();
    let _ = writeln!(o, "\n## Left for a hand: what the program met and did not take ({})\n", left.len());
    for l in &left {
        let _ = writeln!(o, "- {l}");
    }
    let _ = writeln!(o, "\n## A settled value that stands as a value now ({})\n\nThe row's value was written in and nothing more folds: it is passed on, stored, returned or compared.\n", out.values.len());
    for l in &out.values {
        let _ = writeln!(o, "- {l}");
    }
    let _ = writeln!(o, "\n## Settled rows that are numbers or names ({})\n\nA number is a tuning number like its neighbours in the table: the program leaves it.\n", number_rows.len());
    for p in &number_rows {
        let _ = writeln!(o, "- {} = {}: read {} times", rows.by_path[*p].name, rows.by_path[*p].value, before.other_reads.get(*p).copied().unwrap_or(0));
    }
    let _ = writeln!(o, "\n## Rows of the list with no field in the Rust tables ({})\n", rows.not_in_rust.len());
    for r in &rows.not_in_rust {
        let _ = writeln!(o, "- {r}");
    }
    if !rows.value_differs.is_empty() {
        let _ = writeln!(o, "\n## STOP: the list and the tables disagree on a page value ({})\n", rows.value_differs.len());
        for r in &rows.value_differs {
            let _ = writeln!(o, "- {r}");
        }
    }
    let untouched: Vec<&String> = settled_all.iter().filter(|p| !before.reads.contains_key(**p) && !before.other_reads.contains_key(**p) && !findings.contains_key(**p)).copied().collect();
    let _ = writeln!(o, "\n## Settled rows no body reads or writes: the tables' and the wire's alone ({})\n", untouched.len());
    let _ = writeln!(o, "{}", untouched.iter().map(|p| rows.by_path[*p].name.clone()).collect::<Vec<_>>().join(", "));
    let _ = writeln!(o, "\n## Settled rows the fold took, with the reads it met\n");
    for (p, n) in before.reads.iter().filter(|(p, _)| take.contains(*p)) {
        let _ = writeln!(o, "- {} = {}: {n}", rows.by_path[p].name, rows.by_path[p].value);
    }
    if !unnamed {
        let dead_lines: usize = {
            let mut t = pristine.clone();
            dead::fixpoint(&eng.world, &base, &eng.modules, &eng.stems, &mut t, &[], &HashSet::new(), true);
            lines_before - t.iter().map(|x| lines_of(x)).sum::<usize>()
        };
        let _ = writeln!(o, "\n## Functions nothing named before the fold either ({}, {dead_lines} lines): not touched (--unnamed deletes them)\n", dead_before.len());
        let _ = writeln!(o, "{}", was_dead.iter().cloned().collect::<Vec<_>>().join(", "));
    }

    if write {
        for (i, t) in &out.texts {
            if let Err(e) = std::fs::write(&bodies[*i].path, t) {
                eprintln!("fold: cannot write {}: {e}", bodies[*i].rel);
                std::process::exit(3);
            }
        }
        for (m, t) in &out.modrs {
            let _ = std::fs::write(&eng.world.modrs[m].0, t);
        }
    }
    match &report_to {
        Some(f) => {
            let _ = std::fs::write(f, &o);
            let head: Vec<&str> = o.lines().skip(2).take(10).collect();
            println!("{}", head.join("\n"));
            println!("fold: the report is {f}{}", if write { "" } else { " (nothing written: --write)" });
        }
        None => print!("{o}"),
    }

    // the tables' dead weight: every leaf no body reads, as the tree stands after this run
    if let Some(f) = &knobs_to {
        let all: HashSet<String> = HashSet::new();
        let sc = eng.scan(&texts, &all, &out.fns, None);
        // a table whose struct a body names outside a parameter's type (returned, stored, built) can be read where
        // this program does not follow it: its leaves count as read
        let mut named: HashMap<String, i64> = HashMap::new();
        for t in &texts {
            if let Ok(ts) = t.parse::<proc_macro2::TokenStream>() {
                fn walk(ts: proc_macro2::TokenStream, n: &mut HashMap<String, i64>) {
                    for t in ts {
                        match t {
                            proc_macro2::TokenTree::Ident(i) => *n.entry(i.to_string()).or_default() += 1,
                            proc_macro2::TokenTree::Group(g) => walk(g.stream(), n),
                            _ => {}
                        }
                    }
                }
                walk(ts, &mut named);
            }
            if let Ok(file) = syn::parse_file(t) {
                for it in &file.items {
                    if let syn::Item::Fn(x) = it {
                        for a in &x.sig.inputs {
                            if let syn::FnArg::Typed(pt) = a {
                                let mut ty: &syn::Type = &pt.ty;
                                while let syn::Type::Reference(r) = ty {
                                    ty = &r.elem;
                                }
                                if let syn::Type::Path(p) = ty {
                                    if let Some(sg) = p.path.segments.last() {
                                        *named.entry(sg.ident.to_string()).or_default() -= 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // every leaf, with its table's path
        let mut leaves: Vec<(String, String, String)> = Vec::new(); // (path, type, top table)
        fn walk_tree(tree: &rows::Tree, ty: &str, at: &mut Vec<String>, out: &mut Vec<(String, String, String)>) {
            match tree.get(ty) {
                Some(fs) => {
                    for (f, t) in fs {
                        at.push(f.clone());
                        walk_tree(tree, t, at, out);
                        at.pop();
                    }
                }
                None => out.push((at.join("."), ty.to_string(), at.first().cloned().unwrap_or_default())),
            }
        }
        walk_tree(&rows.tree, "Knobs", &mut Vec::new(), &mut leaves);
        let escapes = |path: &str| -> bool {
            // the table itself, or any table it sits in, used as a value or named as a type
            let parts: Vec<&str> = path.split('.').collect();
            (1..parts.len()).any(|n| {
                let pre = parts[..n].join(".");
                let ty = rows.type_path.iter().find(|(_, p)| p.join(".") == pre).map(|(t, _)| t.clone());
                sc.table_uses.get(&pre).copied().unwrap_or(0) > 0 || ty.map(|t| named.get(&t).copied().unwrap_or(0) > 0).unwrap_or(false)
            })
        };
        // the Rust files that are not bodies (a module's types.rs and sig.rs; never the tables' own file or the wire's):
        // a leaf one of them names after `.k.` is read there
        let mut elsewhere: HashSet<String> = HashSet::new();
        let leaf_set: HashSet<&str> = leaves.iter().map(|l| l.0.as_str()).collect();
        let mut others = Vec::new();
        dead::rs_files(&root.join("rust/game/src"), &mut others);
        for p in others {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if body_paths.contains(&p) || matches!(name, "knobs.rs" | "io.rs" | "puppet.rs" | "call_gen.rs") {
                continue;
            }
            let Ok(t) = std::fs::read_to_string(&p) else { continue };
            let mut at = 0;
            while let Some(i) = t[at..].find(".k.") {
                let s0 = at + i + 3;
                let end = t[s0..].find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#')).map(|j| s0 + j).unwrap_or(t.len());
                let mut path = t[s0..end].trim_end_matches('.').to_string();
                loop {
                    if leaf_set.contains(path.as_str()) {
                        elsewhere.insert(path.clone());
                        break;
                    }
                    match path.rfind('.') {
                        Some(j) => path.truncate(j),
                        None => break,
                    }
                }
                at = end.max(s0);
            }
        }
        let mut o = String::new();
        let (mut read, mut maybe, mut dead) = (0usize, 0usize, Vec::new());
        let mut handed = 0usize;
        let mut by_table: BTreeMap<String, (usize, usize)> = BTreeMap::new();
        for (path, ty, top) in &leaves {
            let e = by_table.entry(top.clone()).or_default();
            e.0 += 1;
            if sc.leaf_reads.get(path).copied().unwrap_or(0) > 0 {
                read += 1;
            } else if elsewhere.contains(path) {
                handed += 1;
            } else if escapes(path) {
                maybe += 1;
            } else {
                e.1 += 1;
                dead.push(format!("{path} ({ty}){}", if sc.writes.contains_key(path) { " · written by a body" } else { "" }));
            }
        }
        let whole: Vec<&String> = by_table.iter().filter(|(_, (n, d))| n == d).map(|(t, _)| t).collect();
        let _ = writeln!(o, "# The knob tables' dead weight, as the bodies stand\n");
        let _ = writeln!(o, "leaves of the tables (a yes-or-no row, a number, a name, a list): {}", leaves.len());
        let _ = writeln!(o, "read by a body: {read}");
        let _ = writeln!(o, "read by no body, but named by a made file beside the bodies (the list of numbers the picture's poses get, a test): {handed}");
        let _ = writeln!(o, "not read where this program can follow, but their table is handed on or named as a type by a body (counted as read): {maybe}");
        let _ = writeln!(o, "READ BY NO BODY: {} ({}%) · whole tables no body reads: {} of {}", dead.len(), dead.len() * 100 / leaves.len().max(1), whole.len(), by_table.len());
        let _ = writeln!(o, "\nThe generated wire reads and writes every leaf, and the page's own code may read a table the Rust game only carries: a leaf here is dead for the RUST game.\n");
        let _ = writeln!(o, "## Whole tables no body reads ({})\n\n{}\n", whole.len(), whole.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
        let _ = writeln!(o, "## Leaves no body reads ({})\n", dead.len());
        for d in &dead {
            let _ = writeln!(o, "- {d}");
        }
        let _ = std::fs::write(f, &o);
        println!("{}", o.lines().skip(2).take(4).collect::<Vec<_>>().join("\n"));
        println!("fold: the tables' dead weight is {f}");
    }

    // 3. the table: each row that stays a switch, tried alone on top of the fold
    if let Some(f) = &table_to {
        let folded = texts.clone();
        let base2 = dead::cache(&eng.world, &folded);
        let fr_before: Vec<HashMap<String, i64>> = folded.iter().map(|t| fields::field_reads(t)).collect();
        let mut fr_sum: HashMap<String, i64> = HashMap::new();
        for m in &fr_before {
            for (k, v) in m {
                *fr_sum.entry(k.clone()).or_default() += v;
            }
        }
        let rel_ix: HashMap<&str, usize> = bodies.iter().enumerate().map(|(i, b)| (b.rel.as_str(), i)).collect();
        let mut t = String::new();
        let _ = writeln!(t, "row\tkind\tpage\treads\tfiles\tlines\tfns_deleted\tfns_settled\tfields_unread\tvalues_left\tleft_for_hand\twrites\tread_in\tdeleted\tsettled\tfields\twritten_at");
        // one walk for every row that stays a switch: where each is read, where each is written
        let rest: HashSet<String> = rows.by_path.iter().filter(|(p, r)| r.kind != Kind::Settled && !told.contains(*p) && r.bool_value.is_some()).map(|(p, _)| p.clone()).collect();
        let sc = eng.scan(&folded, &rest, &no_fns, None);
        let none: BTreeMap<String, usize> = BTreeMap::new();
        for (p, r) in rows.by_path.iter().filter(|(p, _)| rest.contains(*p)) {
            let one: HashSet<String> = [p.clone()].into_iter().collect();
            let reads = sc.reads.get(p).copied().unwrap_or(0);
            let w = written(&sc, &one);
            let read_in = sc.read_files.get(p).unwrap_or(&none);
            let start: Vec<usize> = read_in.keys().filter_map(|f| rel_ix.get(f.as_str()).copied()).collect();
            let mut tx = folded.clone();
            let o1 = eng.fold(&mut tx, &one, Some(start), &base2, &dead_before);
            let la: i64 = (0..tx.len()).filter(|i| tx[*i] != folded[*i]).map(|i| lines_of(&folded[i]) as i64 - lines_of(&tx[i]) as i64).sum();
            // the field names no body reads once the row is folded
            let mut delta: HashMap<String, i64> = HashMap::new();
            for i in 0..tx.len() {
                if tx[i] != folded[i] {
                    for (k, v) in &fr_before[i] {
                        *delta.entry(k.clone()).or_default() -= v;
                    }
                    for (k, v) in fields::field_reads(&tx[i]) {
                        *delta.entry(k).or_default() += v;
                    }
                }
            }
            let mut unread: Vec<String> = delta.iter().filter(|(k, d)| **d < 0 && fr_sum.get(*k).copied().unwrap_or(0) + **d == 0).map(|(k, _)| k.clone()).collect();
            unread.sort();
            let del: BTreeSet<String> = o1.deleted.iter().map(|(m, n)| format!("{m}::{n}")).collect();
            let set: BTreeSet<String> = o1.fns.iter().map(|((m, n), _)| format!("{m}::{n}")).collect();
            let wa: BTreeSet<String> = w.values().flatten().map(|s| s.split(": ").next().unwrap_or("").to_string()).collect();
            let _ = writeln!(
                t,
                "{}\t{}\t{}\t{reads}\t{}\t{la}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                r.name,
                if r.kind == Kind::Open { "open" } else { "player" },
                r.value,
                read_in.len(),
                del.len(),
                set.len(),
                unread.len(),
                o1.values.len(),
                o1.left.len(),
                wa.len(),
                read_in.iter().map(|(f, n)| format!("{}:{n}", f.trim_start_matches("rust/game/src/"))).collect::<Vec<_>>().join(" "),
                del.into_iter().collect::<Vec<_>>().join(" "),
                set.into_iter().collect::<Vec<_>>().join(" "),
                unread.join(" "),
                wa.into_iter().collect::<Vec<_>>().join(" | "),
            );
        }
        let _ = std::fs::write(f, &t);
        println!("fold: the table of rows that stay switches is {f}");
    }
    if !rows.value_differs.is_empty() {
        std::process::exit(1);
    }
}
