//! THE GATE: one native command for what a build is held to before it joins the others (README.md, "The gate"). Nothing of TypeScript is in it, and no page.
//!
//!   gate                 the quick kind: every row, four at a time; exit 0 = all pass · 1 = a row failed · 64 = a wrong word
//!   gate ROW…            only rows whose name starts with one of the words (`gate start`, `gate "start depot-four"`, `gate save`)
//!   gate pin [ROW…]      record the pins again (all, or the rows named) and say which moved; writes pins.txt
//!   gate breaks          each row of the breaks' table plays unbroken and must PASS; then each of its breaks must FAIL, in its check's own words. exit 0 = all did
//!   gate long            the quick kind, then the rows that are too slow for it
//!   gate list            the rows
//!   gate tests [BINARY [TEST]]   the whole native test set (tests.rs); no part of the quick kind
//!
//! A row is a line of a child process of this tool (`gate one ROW… [--break=NAME] [--pin]`), because a game is one a process and because a game that panics takes only its own rows with it.
//! The rows of one run share a child, since the run is played once for all of them. The longest kinds are started first, and the rows are printed in their order.
mod draw;
mod pins;
mod played;
mod rows;
mod tests;

use rows::{Flags, Job, Row};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};

const USAGE: &str = "gate [ROW…]         every row of the quick kind, or those whose name starts with one of the words\ngate long           the quick kind, then the slow rows\ngate pin [ROW…]     record the pins again and say which moved\ngate breaks         every check's own fault planted: each must FAIL in its words\ngate list           the rows\ngate tests [BINARY [TEST]]   the whole native test set";

/// what a child said of a row
#[derive(Clone)]
struct Said { pass: bool, secs: f64, words: String }

/// what a child came back with
#[derive(Default)]
struct Came { said: BTreeMap<String, Said>, pins: Vec<String> }

/// the job's rows in a child, as one call of this tool. A row the child never spoke of stopped with it: the first lines of what it wrote on its way down say why.
fn child(names: &[String], brk: Option<&str>, pin: bool) -> Came {
    let me = std::env::current_exe().expect("this tool's own path");
    let mut c = Command::new(me);
    c.arg("one").args(names);
    if let Some(name) = brk { c.arg(format!("--break={}", name)); }
    if pin { c.arg("--pin"); }
    let o = c.stdin(Stdio::null()).output().expect("a child of this tool");
    let mut came = Came::default();
    for line in String::from_utf8_lossy(&o.stdout).lines() {
        let w: Vec<&str> = line.splitn(5, '\t').collect();
        match w[0] {
            "row" if w.len() == 5 => { came.said.insert(w[1].to_string(), Said { pass: w[2] == "PASS", secs: w[3].parse().unwrap_or(0.0), words: w[4].to_string() }); }
            _ => if let Some(pin) = line.strip_prefix("pin\t") { came.pins.push(pin.to_string()); },
        }
    }
    let err = String::from_utf8_lossy(&o.stderr);
    let why: Vec<&str> = err.lines().filter(|l| !l.trim().is_empty() && !l.starts_with("note:")).take(3).collect();
    for n in names { came.said.entry(n.clone()).or_insert_with(|| Said { pass: false, secs: 0.0, words: format!("the child stopped ({}): {}", o.status, why.join(" | ")) }); }
    if !o.status.success() {
        if let Some(s) = names.last().and_then(|n| came.said.get_mut(n)).filter(|s| s.pass) { (s.pass, s.words) = (false, format!("the child ended badly after its rows ({}): {}", o.status, why.join(" | "))); }
    }
    came
}

/// every job of the list in a child, as many at once as there are cores and at most four, the jobs in the order given; `done` hears each as it comes back
fn run_jobs(jobs: Vec<(Vec<String>, Option<&'static str>)>, pin: bool, done: mpsc::Sender<(usize, Came)>) {
    let queue = Arc::new(Mutex::new((0..jobs.len()).collect::<Vec<usize>>().into_iter()));
    let jobs = Arc::new(jobs);
    for _ in 0..std::thread::available_parallelism().map_or(4, |n| n.get()).min(4) {
        let (queue, jobs, done) = (queue.clone(), jobs.clone(), done.clone());
        std::thread::spawn(move || loop {
            let Some(k) = queue.lock().unwrap().next() else { return };
            let _ = done.send((k, child(&jobs[k].0, jobs[k].1, pin)));
        });
    }
}

/// the rows' jobs, the longest kinds first
fn jobs_of(picked: &[Row]) -> Vec<(Vec<String>, Option<&'static str>)> {
    let mut out: Vec<(u32, Job, Vec<String>)> = Vec::new();
    for r in picked {
        match out.iter_mut().find(|j| j.1 == r.job) { Some(j) => j.2.push(r.name.clone()), None => out.push((r.job.rank(), r.job.clone(), vec![r.name.clone()])) }
    }
    out.sort_by_key(|j| j.0);
    out.into_iter().map(|j| (j.2, None)).collect()
}

/// the rows whose name starts with one of the words; a word that names none is a wrong word
fn pick(all: Vec<Row>, words: &[String]) -> Vec<Row> {
    if let Some(w) = words.iter().find(|w| !all.iter().any(|r| r.name.starts_with(w.as_str()))) { eprintln!("no row's name starts with '{}' (gate list says them)", w); std::process::exit(64); }
    all.into_iter().filter(|r| words.is_empty() || words.iter().any(|w| r.name.starts_with(w.as_str()))).collect()
}

fn line(name: &str, s: &Said) -> String {
    if s.pass {
        if s.words.is_empty() { format!("PASS  {} · {:.1} s", name, s.secs) } else { format!("PASS  {} · {} · {:.1} s", name, s.words, s.secs) }
    } else { format!("FAIL  {}: {}", name, s.words) }
}

/// THE QUICK KIND: every row picked, printed in their order as the children come back. A whole gate (`whole`) also holds the rows' list to the pinned one.
fn gate(picked: Vec<Row>, whole: bool) -> i32 {
    let t = std::time::Instant::now();
    let order: Vec<String> = picked.iter().map(|r| r.name.clone()).collect();
    let jobs = jobs_of(&picked);
    let count = jobs.len();
    let (tx, rx) = mpsc::channel();
    run_jobs(jobs, false, tx);
    let (mut said, mut next, mut failed): (BTreeMap<String, Said>, usize, usize) = (BTreeMap::new(), 0, 0);
    let mut kinds: BTreeMap<String, f64> = BTreeMap::new();
    for _ in 0..count {
        let (_, came) = rx.recv().expect("the children's threads");
        said.extend(came.said);
        while next < order.len() {
            let Some(s) = said.get(&order[next]) else { break };
            println!("{}", line(&order[next], s));
            if !s.pass { failed += 1; }
            *kinds.entry(order[next].split(' ').next().unwrap().to_string()).or_default() += s.secs;
            next += 1;
        }
    }
    if whole {
        let list = pins::Pins::read_file().rows_differ(&rows::every().into_iter().map(|r| r.name).collect::<Vec<_>>());
        for l in &list { println!("{}", l); }
        failed += usize::from(!list.is_empty());
    }
    let share: Vec<String> = kinds.iter().map(|(k, v)| format!("{} {:.0} s", k, v)).collect();
    let passed = order.iter().filter(|n| said.get(*n).is_some_and(|s| s.pass)).count();
    println!("gate: {} of {} rows pass · {:.1} s (the rows' own seconds: {})", passed, order.len(), t.elapsed().as_secs_f64(), share.join(", "));
    if failed > 0 { 1 } else { 0 }
}

/// THE LONG KIND: the quick kind, and the rows too slow for it.
fn long() -> i32 { gate(rows::all(), true).max(gate(rows::long(), false)) }

/// RECORD THE PINS AGAIN: the rows picked are played as far as a pin goes, and what moved is said. Nothing moved: the file is left as it is, byte for byte.
fn pin(picked: Vec<Row>, whole: bool) -> i32 {
    let picked: Vec<Row> = picked.into_iter().filter(|r| r.pinned()).collect();
    if picked.is_empty() { eprintln!("none of those rows has a pin"); return 64; }
    let jobs = jobs_of(&picked);
    let count = jobs.len();
    let (tx, rx) = mpsc::channel();
    run_jobs(jobs, true, tx);
    let mut was = pins::Pins::read_file();
    let mut lines = Vec::new();
    let mut bad = 0;
    for _ in 0..count {
        let (_, came) = rx.recv().expect("the children's threads");
        for (n, s) in &came.said { if !s.pass { println!("{}", line(n, s)); bad += 1; } }
        lines.extend(came.pins);
    }
    if bad > 0 { println!("gate pin: {} rows could not be played: pins.txt is left as it is", bad); return 1; }
    let said = was.take(&lines, whole);
    for s in &said { println!("{}", s); }
    let text = was.text(&rows::every());
    let before = std::fs::read_to_string(pins::path()).unwrap_or_default();
    if text == before { println!("gate pin: nothing moved: pins.txt is as it was"); return 0; }
    std::fs::write(pins::path(), &text).expect("pins.txt");
    println!("gate pin: pins.txt written: {} moved", said.len());
    0
}

/// EVERY KIND'S BREAK: the check is run with its own fault planted, and must say so. A child that stopped has shown nothing of the check.
fn breaks() -> i32 {
    let table = rows::breaks();
    // each row unbroken first (a row that fails anyway shows nothing of its break), and then each break
    let mut plain: Vec<String> = Vec::new();
    for b in &table { if !plain.contains(&b.0) { plain.push(b.0.clone()); } }
    let jobs: Vec<(Vec<String>, Option<&'static str>)> = plain.iter().map(|n| (vec![n.clone()], None)).chain(table.iter().map(|b| (vec![b.0.clone()], Some(b.1)))).collect();
    let count = jobs.len();
    let (tx, rx) = mpsc::channel();
    run_jobs(jobs, false, tx);
    let mut back: BTreeMap<usize, Came> = BTreeMap::new();
    for _ in 0..count { let (k, came) = rx.recv().expect("the children's threads"); back.insert(k, came); }
    let mut wrong = 0;
    for (k, n) in plain.iter().enumerate() {
        let s = &back[&k].said[n];
        println!("{}", line(n, s));
        wrong += usize::from(!s.pass);
    }
    for (k, (n, name, words)) in table.iter().enumerate() {
        let s = &back[&(plain.len() + k)].said[n];
        if s.pass { wrong += 1; println!("STOOD   {} [{}]: its break did not show", n, name); }
        else if !s.words.contains(words) { wrong += 1; println!("ASTRAY  {} [{}]: it failed without '{}': {}", n, name, words, s.words); }
        else { println!("BROKEN  {} [{}]: {}", n, name, s.words); }
    }
    println!("gate breaks: {} rows unbroken, {} breaks, {} not as they should be", plain.len(), table.len(), wrong);
    if wrong > 0 { 1 } else { 0 }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let code = match a.first().map(|s| s.as_str()) {
        Some("one") => {
            let brk = a.iter().find_map(|w| w.strip_prefix("--break=")).map(|n| &*n.to_string().leak());
            let flags = Flags { brk, pin: a.iter().any(|w| w == "--pin") };
            let names: Vec<String> = a[1..].iter().filter(|w| !w.starts_with("--")).cloned().collect();
            // (the start's frames are deep: a thread with room)
            let h = std::thread::Builder::new().stack_size(1 << 30).spawn(move || rows::one(&names, flags)).unwrap();
            if h.join().is_ok() { 0 } else { 2 }
        }
        Some("tests") => tests::run(&a[1..]),
        Some("list") => { for r in rows::every() { println!("{}", r.name); } 0 }
        Some("long") if a.len() == 1 => long(),
        Some("pin") => pin(pick(rows::every(), &a[1..]), a.len() == 1),
        Some("breaks") if a.len() == 1 => breaks(),
        Some(w) if w.starts_with("--") || w == "help" => { eprintln!("{}", USAGE); 64 }
        None => gate(rows::all(), true),
        _ => gate(pick(rows::every(), &a), false),
    };
    std::process::exit(code);
}
