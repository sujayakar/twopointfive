//! The Rust game stepped natively through a whole mission beside the TypeScript game (tools/blitz/README.md).
//!
//!   mission <dir> [ticks] [--keep "1,21,300-310"] [--every N] [--after N]
//!
//! Reads from <dir>, as tools/blitz/mission.ts (or seats.ts, a game of two seats) wrote them: c1.bin (the state as the first tick opens), s0.bin (rust/sim's
//! block then), host.txt, post.txt, gives.txt, hostwrites.txt, knobs.bin, and h<k>.bin (the host's half: a record a seat, the host's cameras and Inputs) before each tick.
//!
//! THE GAME IS IMPORTED ONCE. From then on the Rust game is its own: its die, its knobs, its poses, its lights. Before each tick it takes only the host's half
//! (see `host_half`), because the host's code between two ticks is not run on this side. AFTER EVERY TICK the block of rust/sim is hashed part by part and
//! compared with post.txt, and so is the die's count. The alarm is every part, the pose pool among them. At the first tick a part parts, the state and
//! the block are written (n<k>.bin, ns<k>.bin), `--after` more ticks run, and the exit code is 1. A panic prints `PANIC in tick <k>`, its message and
//! place and the game's frames of the backtrace; the exit code is then 2. A kept tick (--keep / --every) writes n<k>.bin and ns<k>.bin for tools/blitz/diff.ts.
//!
//! BLITZ_SKIP=<names> leaves one of the host's calls round a step out (lights, upload, view, follow), to see by a run what it is for.
use game::ids::Id;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

static TICK: AtomicUsize = AtomicUsize::new(0);

use game_run::{part_hash, NAMES};

fn ranges(s: &str) -> Vec<(usize, usize)> {
    s.split(',').filter(|x| !x.is_empty()).map(|x| { let mut p = x.split('-').map(|y| y.trim().parse::<usize>().expect("a tick number")); let lo = p.next().unwrap(); (lo, p.next().unwrap_or(lo)) }).collect()
}

/// the host's half of tick k from its file (game_run::host_half says what is taken): a record a seat; None where the TypeScript's run wrote none
fn host_half(g: &mut game::game::Game, dir: &str, k: usize) -> Option<Vec<game::cmd::PlayerCmd>> {
    let hb = std::fs::read(format!("{}/h{}.bin", dir, k)).ok()?;
    let cmds = game_run::host_records(g, &hb);
    // (HARNESS: a game of several seats here is seats-run.ts's, which steps `stepSeats` itself and never fills the Game's own `cmdBuf`; the generated
    //  reader sets it from the local seat's record, as a host that goes through `Game.update` has it)
    if cmds.len() > 1 { g.cmd_buf = None; }
    Some(cmds)
}

/// one line at the run's end: every beat of the level's script the Rust game started, with how often it was started and the resumes it had in all
fn beats_line(g: &game::game::Game) {
    let mut seen: Vec<(String, usize, f64, String)> = Vec::new();
    for b in g.tab.director_live_beat.ids().collect::<Vec<_>>() {
        let lb = &g.tab.director_live_beat[b];
        let st = format!("{:?}", lb.state).to_lowercase();
        match seen.iter_mut().find(|x| x.0 == lb.name) { Some(x) => { x.1 += 1; x.2 += lb.step; x.3 = st; } None => seen.push((lb.name.clone(), 1, lb.step, st)) }
    }
    // (a few to a line: run.sh shows a line's first 330 characters)
    for (i, c) in seen.chunks(6).enumerate() { println!("BEATS the Rust game ran ({} of {}): {}", i * 6 + c.len(), seen.len(), c.iter().map(|x| format!("{} x{} ({} resumes, {})", x.0, x.1, x.2, x.3)).collect::<Vec<_>>().join(" · ")); }
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
    let meter: Vec<f64> = text("meter.txt").lines().filter_map(|l| l.trim().parse().ok()).collect();
    let hostwrites: Vec<String> = text("hostwrites.txt").lines().map(String::from).collect();
    // ONE ORDERED RECORD of what the driver did before each tick (between.txt, where mission.ts wrote one): its lines by tick, in the file's order
    let mut between: std::collections::HashMap<usize, Vec<String>> = std::collections::HashMap::new();
    for l in text("between.txt").lines() { if let Some(k) = l.split(' ').next().and_then(|x| x.parse::<usize>().ok()) { between.entry(k).or_default().push(l.to_string()); } }
    let ordered = !between.is_empty();
    let no_between: Vec<String> = std::env::var("BLITZ_NO_BETWEEN").unwrap_or_default().split(',').filter(|s| !s.is_empty()).map(String::from).collect();
    println!("runner: the driver's orders, writes and calls before a tick are replayed {}", if ordered { "from ONE ORDERED RECORD (between.txt), as the driver made them" } else { "from gives.txt, then hostwrites.txt, then meter.txt (no between.txt here)" });
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
        let (mut g, said) = game_run::start(&a0, &s0, 0);
        if g.seats.len() > 1 { println!("a game of {} seats: each tick stepped from a record a seat", g.seats.len()); }
        for l in said { println!("{}", l); }
        let (mut parted, mut spent, mut worst) = (0usize, std::time::Duration::ZERO, std::time::Duration::ZERO);
        let mut restarts: Vec<usize> = Vec::new();
        let rows0 = g.tab.boxes.len();
        let (mut n_parted, mut was_same, mut die_parted, mut done) = (0usize, true, 0usize, 0usize);
        let mut verdict_said = false;
        for k in 1..=ticks {
            // THE HOST PUT A CHECKPOINT BACK in the TypeScript's run before this tick opened (`<k> chron_load`): the whole game was replaced by a saved one,
            // which no stepper can follow from its own state. The Rust game takes the TypeScript's state as this tick opens (L<k>.bin, Ls<k>.bin) as a NEW
            // START and the compare goes on. Said loudly: the ticks in step are then two stretches, each from its own start.
            if hostwrites.iter().any(|l| game_run::line_of(l, k) && l.split(' ').nth(1) == Some("chron_load")) {
                let rd = |n: String| std::fs::read(format!("{}/{}", dir, n)).unwrap_or_else(|_| panic!("{} missing in {}", n, dir));
                let (a, s) = (rd(format!("L{}.bin", k)), rd(format!("Ls{}.bin", k)));
                let (g2, said) = game_run::start(&a, &s, 0);
                g = g2;
                println!("TICK {}: THE HOST PUT A CHECKPOINT BACK before this tick: the Rust game takes the TypeScript's state as the tick opens as a new start ({} ticks compared since the last start)", k, k - 1 - restarts.last().copied().unwrap_or(1) + 1);
                restarts.push(k);
                for l in said { println!("{}", l); }
            }
            if std::env::var("BLITZ_TEND").is_ok() && (k == 1 || restarts.last() == Some(&k)) {
                let mut sets = Vec::new(); game::posehome::each_posed(&mut g, &mut |_, s| sets.push(s));
                let home = sets.iter().filter(|s| g.tab.pose_set[**s].home.is_some()).count();
                let stray: f64 = sets.iter().map(|s| g.tab.pose_set[*s].stray).sum();
                let loose = sets.iter().map(|s| g.tab.pose_set[*s].list.iter().filter(|p| p.at >= game::anim_skeleton::Pose::LOOSE).count()).sum::<usize>();
                let held_not_posed = g.pose_sets.iter().filter(|s| !sets.contains(s)).count();
                println!("TEND as tick {} opens: {} sets posed, {} of them at home, {} held (of them {} on no list), strays {}, poses of posed sets outside the pool {}, loose rows {}", k, sets.len(), home, g.pose_sets.len(), held_not_posed, stray, loose, g.loose_poses.len());
            }
            let cmd = match host_half(&mut g, &dir, k) { Some(c) => c, None => { println!("no h{}.bin: the TypeScript's run ended at tick {}", k, k - 1); break; } };
            // HOST STEP: where the headless host listed the box world again between two ticks. BEFORE the driver's orders and writes: the host lists as its
            // step ends, and what the driver then does to the box world (a wall charge blown by the module's own call) is not listed until the next step
            // ends. Listed after it, the aim ray of the tick met a piece of wall the TypeScript's list did not yet hold (mousehole-four with wall charges, tick 6326).
            // HOST STEP: between two ticks the view's matrix is the host camera's live one (game_run::view_live), as the host's half has just brought it
            if !skip("viewlive") { game_run::view_live(&mut g); }
            if !skip("upload") && host[k - 1].get(4).copied().unwrap_or(0.0) > 0.0 { game_run::upload(&mut g); }
            // HARNESS STAND-IN: the test driver's orders, its writes and the rules it called before this tick, in its own order (game_run::between)
            if ordered { for l in between.get(&k).map(|v| v.as_slice()).unwrap_or(&[]) { let t = game_run::between(&mut g, k, l, &no_between); if !t.is_empty() { println!("{}", t); } } }
            else {
            for l in gives.iter().filter(|l| game_run::line_of(l, k)) { println!("{}", game_run::give(&mut g, k, l)); }
            // HARNESS STAND-IN: the test driver's own question that writes the game (`g.fight.roomAt(p)` between ticks floods a room and keeps it)
            for l in hostwrites.iter().filter(|l| game_run::line_of(l, k)) { if let Some(t) = game_run::write_before(&mut g, k, l) { println!("{}", t); } }
            }
            // BLITZ_CALL="<k> <rule> [start]": a rule of the game (restart_encounter | reset_guards) called before tick k, as mission.ts calls it on the
            // (where the TypeScript side wrote ONE ORDERED RECORD the call is a line of it, made above in the driver's order; this is for a game of two seats)
            // TypeScript side under the same word. With `start` it is not called here: mission.ts wrote the state after the call as a new start.
            if let (false, Ok(c)) = (ordered, std::env::var("BLITZ_CALL")) { let w: Vec<&str> = c.split(' ').collect(); if w[0].parse::<usize>().ok() == Some(k) && w.get(2) != Some(&"start") {
                match w[1] { "restart_encounter" => game::game::game_restart_encounter(&mut g), "reset_guards" => game::game::game_reset_guards(&mut g), x => panic!("BLITZ_CALL: no rule {}", x) }
                println!("BLITZ_CALL before tick {}: {} called; {} guards, time {}", k, w[1], g.guards.len(), g.time); } }
            if !ordered { if let Some(v) = meter.get(k - 1) { game_run::meter(&mut g, *v); } }
            // HOST STEP: the flag the player's own restart leaves for the host's next frame is put down (game_run::restart_cut)
            if !skip("restartcut") { game_run::restart_cut(&mut g); }
            let dt = host[k - 1][0];
            TICK.store(k, Ordering::Relaxed);
            let t0 = std::time::Instant::now();
            game_run::step_seats(&mut g, dt, &cmd);
            let el = t0.elapsed(); spent += el; if el > worst { worst = el; }
            done = k;
            // (the mission's verdict as the game itself answers it, game::verdict::verdict_ended: the first tick it does, once)
            if !verdict_said { if let Some((won, word, why)) = game::verdict::verdict_ended(&mut g) { verdict_said = true; println!("VERDICT at tick {}: {} {}{}", k, if won { "won" } else { "lost" }, word, if why.is_empty() { String::new() } else { format!(" ({})", why) }); } }
            // (a level script's beats as they stand: every beat this game started, how many resumes it has had, where it is)
            if std::env::var("BLITZ_BEATS").is_ok() { for b in g.tab.director_live_beat.ids().collect::<Vec<_>>() { let lb = &g.tab.director_live_beat[b]; println!("  beat after tick {}: {} {:?} step {} waiting {:?} it {:?}", k, lb.name, lb.state, lb.step, lb.waiting, lb.it); } }
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
            // (the pose pool is IN the alarm, like every other part: the bridge carries which sets are at home and what each animator holds)
            // THE TYPESCRIPT'S HOST LOADED A FILE SAVE INSIDE THIS TICK (`<k> load_in_step`: the down card's bare Enter with no team on the floor,
            // player.ts:495 → savestate.ts loadInStep, "a file is poured in at once"). The save store is the host's: the Rust tick's boundary holds
            // nothing, finds no checkpoint and RESTARTS the mission in its place. The two games are different games after this tick by design of
            // the boundary, so the tick is not compared; the next tick opens on the TypeScript's state as a new start (its `chron_load` line).
            let loaded_in = hostwrites.iter().any(|l| game_run::line_of(l, k) && l.split(' ').nth(1) == Some("load_in_step"));
            if loaded_in { off.clear(); println!("TICK {}: THE HOST LOADED A FILE SAVE INSIDE THIS TICK on the TypeScript side; the Rust tick has no store and did what it does with none ({} guards, time {:.4}, die n={}): NOT COMPARED, a new start follows", k, g.guards.len(), g.time, d.n); }
            let same = off.is_empty();
            let die_same = loaded_in || p.map(|p| p[1] == d.n.to_string()).unwrap_or(false);
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
            if !skip("view") { game_run::view(&mut g); }
            if !skip("follow") { game_run::follow(&mut g); }
            if parted > 0 && k >= parted + after {
                beats_line(&g);
                println!("CONTROL (man-ticks a rule had a man) {}", game::control::control_tally(&g));
                println!("THE BLOCK FIRST PARTED AT TICK {}; stopped after tick {}", parted, k);
                let _ = std::io::stdout().flush();
                std::process::exit(1);
            }
        }
        // (BLITZ_BOXES: what the rows of the box table made since the start are, by half size and flags, the most first: which maker still leaves a row for good)
        if std::env::var("BLITZ_BOXES").is_ok() {
            let mut by: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
            let free: std::collections::HashSet<usize> = g.box_frame_free.iter().map(|b| b.ix()).collect();
            for (i, id) in g.tab.boxes.ids().enumerate() { if i < rows0 || free.contains(&i) { continue; } let b = &g.tab.boxes[id]; *by.entry(format!("h {:.3} {:.3} {:.3} flags {} owner {} name {:?}", b.h[0], b.h[1], b.h[2], b.flags, b.owner, b.name)).or_default() += 1; }
            let mut v: Vec<_> = by.into_iter().collect(); v.sort_by(|a, b| b.1.cmp(&a.1));
            println!("BOXES: {} rows at the start, {} now, {} of them free frame rows; made since and held:", rows0, g.tab.boxes.len(), free.len());
            for (k, n) in v.iter().take(14) { println!("   {} x {}", n, k); }
        }
        beats_line(&g);
        println!("CONTROL (man-ticks a rule had a man) {}", game::control::control_tally(&g));
        // (the loop's counters of places mended by reading: game/b_tap.rs)
        { let t = game::game::b_tap::read(); println!("TAPS {}", game::game::b_tap::NAMES.iter().zip(t.iter()).map(|(n, c)| format!("{} {}", n, c)).collect::<Vec<_>>().join(" · ")); }
        if !restarts.is_empty() { println!("NEW STARTS (a checkpoint put back by the TypeScript's host): as ticks {:?} opened", restarts); }
        println!("DONE: {} ticks; the die's count {}; every part of the block, `poses` too, the same after every tick; mean Rust tick {:.2} ms, worst {:.1} ms",
            done, if die_parted == 0 { "the same after every tick".to_string() } else { format!("FIRST PARTED AT TICK {}", die_parted) },
            spent.as_secs_f64() * 1e3 / done.max(1) as f64, worst.as_secs_f64() * 1e3);
        if die_parted != 0 { let _ = std::io::stdout().flush(); std::process::exit(1); }
    }).unwrap();
    std::process::exit(if h.join().is_ok() { 0 } else { 2 });
}
