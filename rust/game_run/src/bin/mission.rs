//! The Rust game stepped natively through a whole mission beside the TypeScript game (tools/blitz/README.md).
//!
//!   mission <dir> [ticks] [--keep "1,21,300-310"] [--every N] [--after N]
//!
//! Reads from <dir>, as tools/blitz/mission.ts wrote them: c1.bin (the state as the first tick opens), s0.bin (rust/sim's block then), host.txt, post.txt,
//! gives.txt, hostwrites.txt, knobs.bin, and h<k>.bin (the host's half) before each tick.
//!
//! THE GAME IS IMPORTED ONCE. From then on the Rust game is its own: its die, its knobs, its poses, its lights. Before each tick it takes only the host's half
//! (see `host_half`), because the host's code between two ticks is not run on this side. AFTER EVERY TICK the block of rust/sim is hashed part by part and
//! compared with post.txt, and so is the die's count. The alarm is every part but `poses` (see POSES below). At the first tick a part parts, the state and
//! the block are written (n<k>.bin, ns<k>.bin), `--after` more ticks run, and the exit code is 1. A panic prints `PANIC in tick <k>`, its message and
//! place and the game's frames of the backtrace; the exit code is then 2. A kept tick (--keep / --every) writes n<k>.bin and ns<k>.bin for tools/blitz/diff.ts.
//!
//! BLITZ_SKIP=<names> leaves one of the host's calls round a step out (lights, upload), to see by a run what it is for.
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

static TICK: AtomicUsize = AtomicUsize::new(0);

use game_run::{part_hash, NAMES, POSES};

fn ranges(s: &str) -> Vec<(usize, usize)> {
    s.split(',').filter(|x| !x.is_empty()).map(|x| { let mut p = x.split('-').map(|y| y.trim().parse::<usize>().expect("a tick number")); let lo = p.next().unwrap(); (lo, p.next().unwrap_or(lo)) }).collect()
}

/// the host's half of tick k from its file (game_run::host_half says what is taken); None where the TypeScript's run wrote none
fn host_half(g: &mut game::game::Game, dir: &str, k: usize) -> Option<game::cmd::PlayerCmd> {
    let hb = std::fs::read(format!("{}/h{}.bin", dir, k)).ok()?;
    Some(game_run::host_half(g, &hb, k == 1))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 2 { eprintln!("mission <dir> [ticks] [--keep \"1,21,300-310\"] [--every N] [--after N]"); std::process::exit(64); }
    let dir = a[1].clone();
    let opt = |n: &str| a.iter().position(|x| x == n).and_then(|i| a.get(i + 1)).cloned();
    let ticks: usize = a.get(2).filter(|s| !s.starts_with("--")).map(|s| s.parse().expect("ticks")).unwrap_or(usize::MAX);
    let keep = ranges(&opt("--keep").unwrap_or_default());
    let every: usize = opt("--every").map(|s| s.parse().unwrap()).unwrap_or(0);
    let after: usize = opt("--after").map(|s| s.parse().unwrap()).unwrap_or(0);
    let skips: Vec<String> = std::env::var("BLITZ_SKIP").unwrap_or_default().split(',').map(String::from).collect();
    let text = |n: &str| std::fs::read_to_string(format!("{}/{}", dir, n)).unwrap_or_default();
    let rd = |n: &str| std::fs::read(format!("{}/{}", dir, n)).unwrap_or_else(|_| panic!("{} missing in {}", n, dir));
    let mut post: Vec<Vec<String>> = text("post.txt").lines().map(|l| l.split(' ').map(String::from).collect()).collect();
    let head = if post.is_empty() { Vec::new() } else { post.remove(0) };
    if head.first().map(|s| s.as_str()) != Some("parts") || head[1..].iter().map(|s| s.as_str()).ne(NAMES.iter().copied()) {
        println!("post.txt names the block's parts {:?} and this runner {:?}: one side is of another tree", head, NAMES);
        std::process::exit(3);
    }
    let a0 = rd("c1.bin");
    let s0 = rd("s0.bin");
    let host: Vec<Vec<f64>> = text("host.txt").lines().map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect()).collect();
    let gives: Vec<String> = text("gives.txt").lines().map(String::from).collect();
    let hostwrites: Vec<String> = text("hostwrites.txt").lines().map(String::from).collect();
    let len = core::mem::size_of::<sim::state::State>();
    println!("runner: rust/sim's block is {} bytes here, {} in s0.bin; layout word {}", len, s0.len(), sim::state::LAYOUT);
    std::panic::set_hook(Box::new(|info| {
        println!("PANIC in tick {}: {}", TICK.load(Ordering::Relaxed), info);
        let bt = std::backtrace::Backtrace::force_capture().to_string();
        let mut n = 0;
        for l in bt.lines() {
            let l = l.trim();
            if let Some(i) = l.find(": ") {
                let f = &l[i + 2..];
                if (f.starts_with("game::") || f.starts_with("sim::") || f.starts_with("<game::")) && !f.contains("::{closure") && n < 22 { println!("   {}", f); n += 1; }
            }
        }
        let _ = std::io::stdout().flush();
    }));
    // (the tick's frames are deep in a build with no optimisation: a thread with room)
    let h = std::thread::Builder::new().stack_size(2 << 30).spawn(move || {
        let skip = |n: &str| skips.iter().any(|s| s == n);
        let mut g = game_run::start(&a0, &s0, 0);
        let (mut parted, mut spent, mut worst) = (0usize, std::time::Duration::ZERO, std::time::Duration::ZERO);
        let (mut n_parted, mut was_same, mut pool_parted, mut die_parted, mut done) = (0usize, true, false, 0usize, 0usize);
        for k in 1..=ticks {
            let cmd = match host_half(&mut g, &dir, k) { Some(c) => c, None => { println!("no h{}.bin: the TypeScript's run ended at tick {}", k, k - 1); break; } };
            for l in gives.iter().filter(|l| game_run::line_of(l, k)) { println!("{}", game_run::give(&mut g, k, l)); }
            // HARNESS STAND-IN: the test driver's own question that writes the game (`g.fight.roomAt(p)` between ticks floods a room and keeps it)
            for l in hostwrites.iter().filter(|l| game_run::line_of(l, k)) { if let Some(t) = game_run::write_before(&mut g, k, l) { println!("{}", t); } }
            let dt = host[k - 1][0];
            // HOST STEP: where the headless host listed the box world again between two ticks
            if !skip("upload") && host[k - 1].get(4).copied().unwrap_or(0.0) > 0.0 { game_run::upload(&mut g); }
            TICK.store(k, Ordering::Relaxed);
            let t0 = std::time::Instant::now();
            game_run::step(&mut g, dt, &cmd);
            let el = t0.elapsed(); spent += el; if el > worst { worst = el; }
            done = k;
            for l in hostwrites.iter().filter(|l| game_run::line_of(l, k)) { if let Some(t) = game_run::write_after(&mut g, k, l) { println!("{}", t); } }
            if k == 1 {
                if let Ok(want) = std::fs::read(format!("{}/knobs.bin", dir)) {
                    let n = sim::knobs::len(0);
                    let got = unsafe { std::slice::from_raw_parts(sim::knobs::addr(0) as *const f64, n) };
                    let bad: Vec<usize> = (0..n.min(want.len() / 8)).filter(|i| got[*i].to_bits() != f64::from_le_bytes(want[i * 8..i * 8 + 8].try_into().unwrap()).to_bits()).collect();
                    println!("KNOBS: {} of {} written by knobs_to_rust differ from the TypeScript module's table ({} doubles){}", bad.len(), n, want.len() / 8, if bad.is_empty() { String::new() } else { format!(": lines {:?}", &bad[..bad.len().min(12)]) });
                }
            }
            let d = &sim::state::state(0).dice;
            let w = unsafe { std::slice::from_raw_parts(sim::state::addr(0) as *const u32, len / 4) };
            let p = post.get(k - 1);
            let mut off: Vec<&str> = Vec::new();
            if let Some(p) = p { for (i, P) in sim::state::PARTS.iter().enumerate() { if p.get(4 + i).map(|s| s.as_str()) != Some(part_hash(&w[P.off / 4..(P.off + P.len) / 4]).to_string().as_str()) { off.push(NAMES[i]); } } } else { off.push("no line of post.txt"); }
            let pool_off = off.contains(&NAMES[POSES]);
            off.retain(|n| *n != NAMES[POSES]);
            let same = off.is_empty();
            if same && pool_off && !pool_parted { pool_parted = true; println!("TICK {}: the part `poses` parts and every other part is the same: from here the alarm is the block without `poses`", k); }
            let die_same = p.map(|p| p[1] == d.n.to_string()).unwrap_or(false);
            if !die_same && die_parted == 0 { die_parted = k; }
            let first_part = !same && parted == 0; if first_part { parted = k; }
            if !same { n_parted += 1; if n_parted <= 40 && !first_part { println!("TICK {}: the block PARTS in {:?} (die n={} / {})", k, off, d.n, p.map(|p| p[1].as_str()).unwrap_or("?")); } } else if n_parted > 0 && !was_same { println!("TICK {}: the block is the same again", k); }
            was_same = same;
            if first_part || k % 100 == 0 || k == ticks || !die_same {
                println!("TICK {}; time {:.4}; die n={} (TypeScript {}); block {}; boxes {}; mean tick {:.2} ms, worst {:.1} ms", k, g.time, d.n, p.map(|p| p[1].as_str()).unwrap_or("?"), if same { "the same".to_string() } else { format!("PARTS in {:?}", off) }, g.tab.boxes.len(), spent.as_secs_f64() * 1e3 / k as f64, worst.as_secs_f64() * 1e3);
            }
            if first_part || keep.iter().any(|(a, b)| k >= *a && k <= *b) || (every > 0 && k % every == 0) {
                std::fs::write(format!("{}/n{}.bin", dir, k), game::io::export(&g)).unwrap();
                let b = unsafe { std::slice::from_raw_parts(sim::state::addr(0) as *const u8, len) };
                std::fs::write(format!("{}/ns{}.bin", dir, k), b).unwrap();
            }
            // HOST STEP AFTER THE TICK (the headless host's `engine.lights.update(dt, game.time)`): every flash's envelope and retirement, which the next tick's light reads
            if !skip("lights") { game_run::lights(&mut g, dt); }
            if parted > 0 && k >= parted + after {
                println!("THE BLOCK FIRST PARTED AT TICK {}; stopped after tick {}", parted, k);
                let _ = std::io::stdout().flush();
                std::process::exit(1);
            }
        }
        println!("DONE: {} ticks; the die's count {}; the block without `poses` the same after every tick; `poses` {}; mean Rust tick {:.2} ms, worst {:.1} ms",
            done, if die_parted == 0 { "the same after every tick".to_string() } else { format!("FIRST PARTED AT TICK {}", die_parted) },
            if pool_parted { "parted (reported above)" } else { "the same after every tick" }, spent.as_secs_f64() * 1e3 / done.max(1) as f64, worst.as_secs_f64() * 1e3);
        if die_parted != 0 { let _ = std::io::stdout().flush(); std::process::exit(1); }
    }).unwrap();
    std::process::exit(if h.join().is_ok() { 0 } else { 2 });
}
