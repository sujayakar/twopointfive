//! The Rust game stepped natively through a whole mission beside the TypeScript game (tools/blitz/README.md).
//!
//!   mission <dir> [ticks] [--keep "1,21,300-310"] [--every N] [--after N]
//!
//! Reads from <dir>, as tools/blitz/mission.ts wrote them: c1.bin (the state as the first tick opens), s0.bin (rust/sim's block then), host.txt, post.txt,
//! gives.txt, hostwrites.txt, menrows.txt, knobs.bin, and h<k>.bin (the host's half) before each tick.
//!
//! THE GAME IS IMPORTED ONCE. From then on the Rust game is its own: its die, its knobs, its poses, its lights. Before each tick it takes only the host's half
//! (see `host_half`), because the host's code between two ticks is not run on this side. AFTER EVERY TICK the block of rust/sim is hashed part by part and
//! compared with post.txt, and so is the die's count. The alarm is every part but `poses` (see POSES below). At the first tick a part parts, the state and
//! the block are written (n<k>.bin, ns<k>.bin), `--after` more ticks run, and the exit code is 1. A panic prints `PANIC in tick <k>`, its message and
//! place and the game's frames of the backtrace; the exit code is then 2. A kept tick (--keep / --every) writes n<k>.bin and ns<k>.bin for tools/blitz/diff.ts.
//!
//! BLITZ_SKIP=<names> leaves a host stand-in out (debris, sparks, men, lights, upload, rects), to learn by a run whether it is still needed.
use game::ids::Id;
use game::io::Io;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

static TICK: AtomicUsize = AtomicUsize::new(0);

/// The block's parts by name, in src/sim/state.ts SIM_PARTS' order. post.txt's first line carries the TypeScript's list and the two must agree.
const NAMES: [&str; 13] = ["head", "dice", "doors", "falls", "props", "slabs", "items", "men", "lights", "weather", "smoke", "poses", "panes"];
/// POSES: a dead man's ragdoll drive holds a pose the TypeScript keeps OUTSIDE the pool for good, and a Rust pose lives in the pool, so from the first death
/// the pool's bytes differ by that pose and say nothing more. That part is reported when it first parts and is not the alarm.
const POSES: usize = 11;
const MEN: usize = 7;

/// one part's hash: its words up to the last that is not zero (a parked block of the TypeScript's may be shorter than the Rust slot; the zeros say nothing)
fn part_hash(w: &[u32]) -> u32 {
    let mut n = w.len();
    while n > 0 && w[n - 1] == 0 { n -= 1; }
    let mut h = 0x811c9dc5u32;
    for x in &w[..n] { h = (h ^ *x).wrapping_mul(0x01000193); }
    h
}

fn ranges(s: &str) -> Vec<(usize, usize)> {
    s.split(',').filter(|x| !x.is_empty()).map(|x| { let mut p = x.split('-').map(|y| y.trim().parse::<usize>().expect("a tick number")); let lo = p.next().unwrap(); (lo, p.next().unwrap_or(lo)) }).collect()
}

/// HARNESS STAND-IN: THE HOST'S HALF BETWEEN TWO TICKS (h<k>.bin). `Game.update` round the step and the headless host's own asks are not run on this side, so
/// what they wrote is taken as tick k opens: the record the host sampled (`cmd_buf`), the host's own camera and Input (every row that is no seat's), and the
/// dialogue's room memo once the HUD's asks have filled it. Nothing else of the TypeScript's state is taken.
fn host_half(g: &mut game::game::Game, dir: &str, k: usize, skip: &dyn Fn(&str) -> bool) -> Option<game::cmd::PlayerCmd> {
    let hb = std::fs::read(format!("{}/h{}.bin", dir, k)).ok()?;
    let mut r = game::io::R::new(&hb);
    let cmd_in = <Option<game::cmd::PlayerCmd> as Io>::r(&mut r);
    let nc = <u32 as Io>::r(&mut r) as usize;
    let cams: Vec<game::camera::FollowCamera> = (0..nc).map(|_| <game::camera::FollowCamera as Io>::r(&mut r)).collect();
    let ni = <u32 as Io>::r(&mut r) as usize;
    let inputs: Vec<game::input::Input> = (0..ni).map(|_| <game::input::Input as Io>::r(&mut r)).collect();
    let rects = <Option<game::dialogue::DialogueModRoomRectsValue> as Io>::r(&mut r);
    assert!(r.p == hb.len(), "h{}.bin: {} bytes left over", k, hb.len() - r.p);
    if k == 1 { return Some(g.cmd_buf.clone().expect("the start holds no record of the tick's input")); }
    let seat_cams: Vec<game::ids::CameraId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].cam).collect();
    for c in g.tab.camera.ids().collect::<Vec<_>>() { if !seat_cams.contains(&c) && c.ix() < cams.len() { g.tab.camera[c] = cams[c.ix()].clone(); } }
    let seat_ins: Vec<game::ids::InputId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].input).collect();
    for i in g.tab.input.ids().collect::<Vec<_>>() { if !seat_ins.contains(&i) && i.ix() < inputs.len() { g.tab.input[i] = inputs[i.ix()].clone(); } }
    if !skip("rects") && g.m.dialogue.room_rects.is_none() { g.m.dialogue.room_rects = rects; }
    g.cmd_buf = cmd_in.clone();
    Some(cmd_in.expect("the TypeScript state holds no record of the tick's input"))
}

/// HARNESS STAND-IN: THE TEST DRIVER'S ORDERS. The scripted player gives its orders through the menu's own call between ticks; mission.ts writes each as a line
/// of gives.txt, `<tick> <id> <who> <kind> <x> <y> <z> [door <place> <side> book<id> | man <place>]`, and the same call is made here before that tick.
fn give(g: &mut game::game::Game, k: usize, l: &str) {
    use game::orders::{OrderTarget as T, Who};
    let w: Vec<&str> = l.split(' ').collect();
    if l.contains("UNSUPPORTED") || w.len() < 7 { println!("  NOTE tick {}: an order of the driver's this runner cannot give: {}", k, l); return; }
    let at = [w[4].parse::<f64>().unwrap(), w[5].parse::<f64>().unwrap(), w[6].parse::<f64>().unwrap()];
    let who = match w[2] { "-" => None, "all" => Some(Who::All), "red" => Some(Who::Red), "blue" => Some(Who::Blue), x => match x.parse::<f64>() { Ok(n) => Some(Who::Num(n)), Err(_) => { println!("  NOTE tick {}: a `who` this runner cannot give: {}", k, l); return; } } };
    let t = match w[3] {
        "floor" => { let serial = g.serial(); T::Floor(game::orders::OrderTargetFloor { at, wall: None, thing: None, serial }) }
        "door" => T::Door(game::orders::OrderTargetDoor { at, door: game::ids::DoorId::from_ix(w[8].parse().unwrap()), side: w[9].parse().unwrap() }),
        "none" => T::None(game::orders::OrderTargetNone { at, man: if w.len() > 8 { Some(g.guards[w[8].parse::<usize>().unwrap()]) } else { None } }),
        _ => T::Foe(game::orders::OrderTargetFoe { at, foe: g.guards[w[8].parse::<usize>().unwrap()] }),
    };
    let why = game::orders::squad_orders_give(g, w[1], Some(&t), who, None);
    println!("  the driver's order before tick {}: {} -> {:?}", k, l, why);
}

/// HARNESS STAND-IN: THE MEN'S COLUMNS WRITTEN BACK. In the TypeScript a man's owner / rank / reason / since ARE the block's `men` columns (src/sim/men);
/// the Rust skeleton has them as plain fields of Guard and the bridge does not say which row a man has (menrows.txt does), so the Rust tick never writes that
/// part. A row whose `gen` is even is free: its man has left and the row stands at zeros (hostwrites.txt `men_gen`, from the TypeScript's column).
fn men_back(g: &game::game::Game, menrows: &[(usize, usize)], men_gen: &[u32]) {
    unsafe {
        let men = (sim::state::addr(0) as *mut u8).add(sim::state::PARTS[MEN].off);
        for (r, ge) in men_gen.iter().enumerate() { (men.add(sim::men::COLS[1] + 4 * r) as *mut u32).write_unaligned(*ge); }
        for (id, row) in menrows.iter() {
            let gd = game::ids::GuardId::from_ix(*id);
            if gd.ix() >= g.gd.len() { continue; }
            let m = &g.gd[gd];
            if !men_gen.is_empty() && men_gen[*row] & 1 == 0 {
                (men.add(sim::men::COLS[0] + 8 * row) as *mut f64).write_unaligned(0.0);
                for c in 2..5 { (men.add(sim::men::COLS[c] + 4 * row) as *mut u32).write_unaligned(0); }
                continue;
            }
            (men.add(sim::men::COLS[0] + 8 * row) as *mut f64).write_unaligned(m.since);
            (men.add(sim::men::COLS[2] + 4 * row) as *mut u32).write_unaligned(m.owner);
            (men.add(sim::men::COLS[3] + 4 * row) as *mut u32).write_unaligned(m.rank);
            (men.add(sim::men::COLS[4] + 4 * row) as *mut u32).write_unaligned(m.reason);
        }
    }
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
    let menrows: Vec<(usize, usize)> = text("menrows.txt").lines().filter_map(|l| { let mut w = l.split(' '); Some((w.next()?.parse().ok()?, w.next()?.parse().ok()?)) }).collect();
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
        let mut g = Box::new(game::io::import(&a0));
        unsafe {
            let at = sim::state::addr(0) as *mut u8;
            core::ptr::write_bytes(at, 0, len);
            core::ptr::copy_nonoverlapping(s0.as_ptr(), at, s0.len().min(len));
        }
        g.sim = 0;
        // HOST STEP: the TypeScript side runs with every DRESS row down (mission.ts dressAll(false)); the bridge does not carry the DRESS table, and the one row a
        // Rust body reads is `debris` (debris.ts: under DEBRIS.dress a host that draws nothing holds no pool).
        if !skip("debris") { g.m.dress.dress.debris = false; }
        // HOST STEP: Game.sparks is not carried by the bridge, so an imported game has an empty pool (the constructor's line again).
        if !skip("sparks") { game::sparks::b_01::sparks_after_import(&mut g); }
        let (mut parted, mut spent, mut worst) = (0usize, std::time::Duration::ZERO, std::time::Duration::ZERO);
        let (mut n_parted, mut was_same, mut pool_parted, mut die_parted, mut done) = (0usize, true, false, 0usize, 0usize);
        let mut men_gen: Vec<u32> = Vec::new();
        for k in 1..=ticks {
            let cmd = match host_half(&mut g, &dir, k, &skip) { Some(c) => c, None => { println!("no h{}.bin: the TypeScript's run ended at tick {}", k, k - 1); break; } };
            for l in gives.iter().filter(|l| l.split(' ').next().and_then(|x| x.parse::<usize>().ok()) == Some(k)) { give(&mut g, k, l); }
            // HARNESS STAND-IN: the test driver's own question that writes the game (`g.fight.roomAt(p)` between ticks floods a room and keeps it)
            for l in hostwrites.iter().filter(|l| l.split(' ').next().and_then(|x| x.parse::<usize>().ok()) == Some(k)) {
                let w: Vec<&str> = l.split(' ').collect();
                if w.get(1) == Some(&"room_at") {
                    let p = [w[2].parse::<f64>().unwrap(), w[3].parse::<f64>().unwrap(), w[4].parse::<f64>().unwrap()];
                    let r = game::roomfight::room_fight_room_at(&mut g, p);
                    println!("  the driver's question before tick {}: roomAt({:?}) -> {:?} ({} rooms)", k, p, r, g.fight.rooms.len());
                }
            }
            let dt = host[k - 1][0];
            // HOST STEP: where the headless host listed the box world again between two ticks
            if !skip("upload") && host[k - 1].get(4).copied().unwrap_or(0.0) > 0.0 { game::host_api::b_01::host_world_upload(&mut g); }
            TICK.store(k, Ordering::Relaxed);
            let t0 = std::time::Instant::now();
            game::game::game_step(&mut g, dt, &cmd);
            let el = t0.elapsed(); spent += el; if el > worst { worst = el; }
            done = k;
            for l in hostwrites.iter().filter(|l| l.split(' ').next().and_then(|x| x.parse::<usize>().ok()) == Some(k)) {
                let mut w = l.splitn(3, ' '); let (_, what, rest) = (w.next(), w.next().unwrap_or(""), w.next().unwrap_or(""));
                if what == "men_gen" { men_gen = rest.split(',').map(|x| x.parse::<u32>().unwrap()).collect(); println!("  the men's rows after tick {}: a man left or entered", k); }
                // HOST STEP: the autosave controller is the host's, and its one write into the game is the chronicle's row, the tick's last thing
                if what == "chron_save" { let label = rest.trim_matches('"'); let _ = game::chronicle::chron_checkpoint(&mut g, game::chronicle::ChronCheckpointWhat::Save, Some(label)); println!("  the host's checkpoint after tick {}: {}", k, rest); }
            }
            if !skip("men") { men_back(&g, &menrows, &men_gen); }
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
            if !skip("lights") { let now = g.time; game::render_lights::light_set_update(&mut g, dt, now); }
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
