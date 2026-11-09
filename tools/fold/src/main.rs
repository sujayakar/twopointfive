//! fold: what deletes dead code from the Rust game, and what says a function only reads (README.md).
//!
//!   fold unnamed [--write] [--report FILE]      the free functions of the body files that nothing names, to a fixed point
//!   fold sharp-prep | sharp-apply | sharp-mend | tests-cut   the steps of sharp.sh and never.sh
//!   fold readers-*                              the steps of readers.sh
//!   fold fields-drop | types-drop --file FILE [--write]    take these fields or types out of the game's crates
//!   fold layout [list]                          which crates are the game's, found by listing rust/

mod dead;
mod knobdrop;
mod layout;
mod readers;
mod sharp;
mod testcut;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::PathBuf;

/// `fold unnamed`: a function of a body file that nothing names goes, then what only it named, until nothing is left to go
fn unnamed_main(args: &[String]) -> ! {
    let (mut root, mut write, mut report) = (PathBuf::from("."), false, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--root" => root = PathBuf::from(it.next().cloned().unwrap_or_default()),
            "--write" => write = true,
            "--report" => report = it.next().cloned(),
            x => {
                eprintln!("fold unnamed: no option {x}");
                std::process::exit(64);
            }
        }
    }
    let lay = layout::find(&root).unwrap_or_else(|e| {
        eprintln!("fold: {e}");
        std::process::exit(3);
    });
    let paths = lay.body_files(&root);
    let name_of = |p: &PathBuf, up: usize| -> String {
        let mut d = p.as_path();
        for _ in 0..up {
            d = d.parent().unwrap();
        }
        d.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()
    };
    let (modules, stems): (Vec<String>, Vec<String>) = (paths.iter().map(|p| name_of(p, 1)).collect(), paths.iter().map(|p| name_of(p, 0)).collect());
    let fail = |p: &std::path::Path, e: std::io::Error| -> ! {
        eprintln!("fold unnamed: {}: {e}", p.display());
        std::process::exit(3);
    };
    let mut texts: Vec<String> = paths.iter().map(|p| std::fs::read_to_string(p).unwrap_or_else(|e| fail(p, e))).collect();
    let was = texts.clone();
    let world = dead::world(&root, &paths.iter().cloned().collect::<HashSet<_>>());
    let base = dead::cache(&world, &texts);
    let swept = dead::fixpoint(&world, &base, &modules, &stems, &mut texts, &[], &HashSet::new(), true);
    let mut o = String::new();
    let _ = writeln!(o, "{} functions nothing named or only such a function named\n", swept.deleted.len());
    let deleted: BTreeSet<&(String, String)> = swept.deleted.iter().collect();
    let later: Vec<String> = deleted.iter().filter(|k| !swept.first.contains(**k)).map(|(m, f)| format!("{m}::{f}")).collect();
    let _ = writeln!(o, "### Nothing names it\n");
    for (m, f) in deleted.iter().filter(|k| swept.first.contains(**k)) {
        let _ = writeln!(o, "- {m}::{f}");
    }
    let _ = writeln!(o, "\n### Only a function deleted here named it\n\n{}", later.join(", "));
    if write {
        for ((p, t), w) in paths.iter().zip(&texts).zip(&was) {
            if t != w {
                std::fs::write(p, t).unwrap_or_else(|e| fail(p, e));
            }
        }
        for (module, t) in &swept.modrs {
            let p = &world.modrs[module].0;
            std::fs::write(p, t).unwrap_or_else(|e| fail(p, e));
        }
    }
    match report {
        Some(f) => {
            std::fs::write(&f, &o).unwrap_or_else(|e| fail(std::path::Path::new(&f), e));
        }
        None => print!("{o}"),
    }
    println!("fold unnamed: {} functions{}", swept.deleted.len(), if write { "" } else { " (nothing written: --write)" });
    std::process::exit(0);
}

fn layout_main(cmd: &str, args: &[String]) -> ! {
    let mut root = PathBuf::from(".");
    let (mut what, mut diag, mut at, mut base): (Option<String>, Option<String>, Option<String>, Option<String>) = (None, None, None, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().unwrap_or_default();
        match a.as_str() {
            "--root" => root = PathBuf::from(val()),
            "--diag" => diag = Some(val()),
            "--in" => at = Some(val()),
            "--base" => base = Some(val()),
            x if !x.starts_with('-') && what.is_none() && cmd == "layout" => what = Some(x.to_string()),
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
    if cmd == "diag-from-root" {
        let (Some(d), Some(at)) = (diag, at) else {
            eprintln!("fold diag-from-root: --diag FILE --in FOLDER (the folder of the crate the check ran in, from the root)");
            std::process::exit(64);
        };
        let text = std::fs::read_to_string(&d).unwrap_or_else(|e| fail(format!("cannot read {d}: {e}")));
        let base = base.map(PathBuf::from).unwrap_or_else(|| root.clone());
        std::fs::write(&d, layout::diag_from_root(&text, &base, std::path::Path::new(&at))).unwrap_or_else(|e| fail(format!("cannot write {d}: {e}")));
        std::process::exit(0);
    }
    let lay = layout::find(&root).unwrap_or_else(|e| fail(e));
    let src = |names: Vec<String>| names.into_iter().map(|c| format!("rust/{c}/src")).collect::<Vec<_>>();
    let lines: Vec<String> = match what.as_deref() {
        None => vec![format!("fold: {}", lay.say())],
        Some("game") => lay.game.iter().cloned().collect(),
        Some("bodies") => lay.bodies.clone(),
        Some("users") => lay.users(),
        Some("src") => src(lay.game.iter().cloned().collect()),
        Some("body-src") => src(lay.bodies.clone()),
        Some("knobs") => vec![lay.knobs.to_string_lossy().to_string()],
        Some("page") => lay.crates.iter().filter(|c| c.page).map(|c| c.name.clone()).collect(),
        Some(x) => {
            eprintln!("fold layout: no list {x} (game, bodies, users, src, body-src, knobs, page)");
            std::process::exit(64);
        }
    };
    println!("{}", lines.join("\n"));
    std::process::exit(0);
}

/// `fold sharp-prep --scratch DIR` and `fold sharp-apply --diag FILE [--write] [--report FILE]`:
/// the compiler's dead-code reading (sharp.rs; tools/fold/sharp.sh drives the two to a fixed point).
fn sharp_main(cmd: &str, args: &[String]) -> ! {
    let mut root = PathBuf::from(".");
    let mut scratch = std::env::temp_dir().join("fold-sharp");
    let mut diag: Option<String> = None;
    let mut report: Option<String> = None;
    let mut write = false;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().unwrap_or_default();
        match a.as_str() {
            "--root" => root = PathBuf::from(val()),
            "--scratch" => scratch = PathBuf::from(val()),
            "--diag" => diag = Some(val()),
            "--report" => report = Some(val()),
            "--write" => write = true,
            x => {
                eprintln!("fold {cmd}: no option {x}");
                std::process::exit(64);
            }
        }
    }
    if cmd == "sharp-prep" {
        match sharp::prep(&root, &scratch) {
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
    if cmd == "sharp-mend" {
        // `fold sharp-mend --diag FILE [--write]`: a `use` of a name the pass deleted loses the name. Exit 10 mended, 4 an error it cannot mend.
        match testcut::mend(&root, &text, write) {
            Ok((n, other)) => {
                println!("fold: {n} names taken out of a `use` that no longer resolved");
                for o in other.iter().take(8) {
                    println!("  an error this is no mend for: {}", o.chars().take(200).collect::<String>());
                }
                std::process::exit(if !other.is_empty() { 4 } else if n > 0 { 10 } else { 0 });
            }
            Err(e) => {
                eprintln!("fold: {e}");
                std::process::exit(3);
            }
        }
    }
    let named = sharp::read_diag(&text);
    if cmd == "tests-cut" {
        // `fold tests-cut --diag FILE [--write] [--report FILE]`: FILE is the reading WITHOUT the tests; the tests that are
        // there for what it names go (testcut.rs). Exit 10 when a test went.
        match testcut::cut(&root, &named, write) {
            Ok(c) => {
                let mut o = String::new();
                let _ = writeln!(o, "{} tests, {} lines ({} test modules whole){}", c.tests.len(), c.lines, c.mods, if write { "" } else { " (nothing written: --write)" });
                let _ = writeln!(o, "\n## Tests deleted: each calls something only tests use, and no live function of a body file\n");
                let _ = writeln!(o, "{}", c.tests.iter().map(|l| format!("- {l}")).collect::<Vec<_>>().join("\n"));
                let _ = writeln!(o, "\n## Tests that call something only tests use and stay ({})\n", c.stay.len());
                let _ = writeln!(o, "{}", c.stay.iter().map(|l| format!("- {l}")).collect::<Vec<_>>().join("\n"));
                if let Some(f) = &report {
                    let _ = std::fs::write(f, &o);
                }
                println!("{}", o.lines().next().unwrap_or(""));
                std::process::exit(if c.tests.is_empty() { 0 } else { 10 });
            }
            Err(e) => {
                eprintln!("fold: {e}");
                std::process::exit(3);
            }
        }
    }
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

/// `fold readers-demote | readers-restore [--diag FILE] [--taint] | readers-now | readers-report [--bare FILE]`, each with `--state FILE`:
/// whether a function writes, said by its signature (readers.rs; tools/fold/readers.sh drives them).
fn readers_main(cmd: &str, args: &[String]) -> ! {
    let mut root = PathBuf::from(".");
    let mut state = std::env::temp_dir().join("fold-readers.txt");
    let mut diag: Option<String> = None;
    let mut taint = false;
    let mut soften = false;
    let mut bare: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().unwrap_or_default();
        match a.as_str() {
            "--root" => root = PathBuf::from(val()),
            "--state" => state = PathBuf::from(val()),
            "--diag" => diag = Some(val()),
            "--taint" => taint = true,
            "--soften" => soften = true,
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
        "readers-demote" => match readers::demote(&root, &state, soften) {
            Ok(s) => println!("fold: {s}"),
            Err(e) => fail(e),
        },
        "readers-restore" => {
            let text = diag.map(|d| std::fs::read_to_string(&d).unwrap_or_else(|e| fail(format!("cannot read {d}: {e}"))));
            match readers::restore(&root, &state, text.as_deref(), taint) {
                Ok(r) => {
                    println!("fold: `mut` put back on {} parameters the compiler named{}, {} that write past the compiler's sight, {} more by the calls; {} stay `&`", r.by_error, if r.coarse > 0 { format!(" ({} of them a whole file at a time: an error with no function to place it on)", r.coarse) } else { String::new() }, r.by_taint, r.by_calls, r.left);
                    if r.soft_back + r.soft_left > 0 {
                        println!("fold: {} reads spelled as writes needed their `mut` after all; {} stay shared", r.soft_back, r.soft_left);
                    }
                    for u in r.unplaced.iter().take(12) {
                        println!("  an error this program could not place: {u}");
                    }
                    std::process::exit(if r.by_error + r.by_taint + r.by_calls + r.soft_back > 0 { 10 } else if r.unplaced.is_empty() { 0 } else { 4 });
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
        if c == "types-drop" {
            // `fold types-drop --file LIST [--write] [--root DIR]`: each `mod::Type` of LIST out of the Rust source, its declaration
            // and its impls (knobdrop.rs). Exit 4, and nothing written, while anything still names one.
            let mut root = PathBuf::from(".");
            let (mut file, mut write) = (None, false);
            let mut it = args[1..].iter();
            while let Some(a) = it.next() {
                match a.as_str() {
                    "--root" => root = PathBuf::from(it.next().cloned().unwrap_or_default()),
                    "--file" => file = it.next().cloned(),
                    "--write" => write = true,
                    x => {
                        eprintln!("fold {c}: no option {x}");
                        std::process::exit(64);
                    }
                }
            }
            let Some(f) = file else {
                eprintln!("fold {c}: --file LIST");
                std::process::exit(64);
            };
            let names: Vec<String> = std::fs::read_to_string(&f).unwrap_or_else(|e| { eprintln!("fold: cannot read {f}: {e}"); std::process::exit(3) }).lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
            match knobdrop::drop_types(&root, &names, write) {
                Ok((n, still)) => {
                    println!("fold: {n} items of {} types{}", names.len(), if !still.is_empty() { " (NOTHING WRITTEN: something still names one)" } else if write { "" } else { " (nothing written: --write)" });
                    for s in still.iter().take(12) {
                        println!("  {s}");
                    }
                    std::process::exit(if still.is_empty() { 0 } else { 4 });
                }
                Err(e) => {
                    eprintln!("fold: {e}");
                    std::process::exit(3);
                }
            }
        }
        if c == "fields-drop" || c == "knobs-drop" {
            // `fold fields-drop --file LIST [--write] [--root DIR]`: each `mod::Type.field` of LIST out of the Rust source (knobdrop.rs;
            // `knobs-drop` is its first name, for a table's rows)
            let mut root = PathBuf::from(".");
            let (mut file, mut write) = (None, false);
            let mut it = args[1..].iter();
            while let Some(a) = it.next() {
                match a.as_str() {
                    "--root" => root = PathBuf::from(it.next().cloned().unwrap_or_default()),
                    "--file" => file = it.next().cloned(),
                    "--write" => write = true,
                    x => {
                        eprintln!("fold {c}: no option {x}");
                        std::process::exit(64);
                    }
                }
            }
            let Some(f) = file else {
                eprintln!("fold {c}: --file LIST");
                std::process::exit(64);
            };
            let names: Vec<String> = std::fs::read_to_string(&f).unwrap_or_else(|e| { eprintln!("fold: cannot read {f}: {e}"); std::process::exit(3) }).lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
            match knobdrop::drop(&root, &names, write) {
                Ok((n, places, missing)) => {
                    println!("fold: {n} of {} fields out of their struct, {places} places cut{}", names.len(), if write { "" } else { " (nothing written: --write)" });
                    for m in missing.iter().take(12) {
                        println!("  no struct of the Rust source has {m}");
                    }
                    std::process::exit(if missing.is_empty() { 0 } else { 4 });
                }
                Err(e) => {
                    eprintln!("fold: {e}");
                    std::process::exit(3);
                }
            }
        }
        if c == "unnamed" {
            unnamed_main(&args[1..]);
        }
        if c == "sharp-prep" || c == "sharp-apply" || c == "tests-cut" || c == "sharp-mend" {
            sharp_main(c, &args[1..]);
        }
        if c == "layout" || c == "diag-from-root" {
            layout_main(c, &args[1..]);
        }
        if c.starts_with("readers-") {
            readers_main(c, &args[1..]);
        }
    }
    eprintln!("fold: unnamed, sharp-prep, sharp-apply, sharp-mend, tests-cut, readers-*, fields-drop, types-drop or layout");
    std::process::exit(64);
}
