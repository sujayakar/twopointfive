//! The Rust game stepped natively beside the TypeScript game (tools/blitz/README.md).
//!
//!   game_run <dir> <ticks> [--lockstep]
//!
//! Reads from <dir>, as tools/blitz/ts-ticks.ts wrote them: c1.bin (the state as the first tick opens), s0.bin (rust/sim's block then), engine.txt, host.txt,
//! gives.txt, and c<k>.bin for the host's half before each later tick. Writes n<k>.bin (the state) and ns<k>.bin (the block) after each tick and prints `TICK <k> …`.
//! With --lockstep it then waits for a line on stdin: `go` steps the next tick, anything else (or the end of input) stops. A panic prints `PANIC <tick>`, its
//! message and place, and the game's frames of the backtrace; the exit code is then 2.
//!
//! THE GAME IS IMPORTED ONCE. From then on the Rust game is its own: the die, the knobs, the poses, the lights, the probe's table. What it takes from the
//! TypeScript side before each tick is the host's half only (see `host_half`), because the host's code between two ticks is not run on this side.
use game::ids::Id;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

static TICK: AtomicUsize = AtomicUsize::new(0);

/// For tools/blitz/watch-rust.sh: the address of the double being watched, and the function a debugger stops in once it is known.
#[no_mangle]
pub static mut GAME_RUN_WATCH_ADDR: usize = 0;
#[no_mangle]
#[inline(never)]
pub extern "C" fn game_run_watch_ready() { std::hint::black_box(()); }

/// `GAME_RUN_WATCH=<what>:<row>:<index>`: one double of the imported game, by a short name (`time` takes no row). Add an arm for the field you need.
/// The address is taken once, after the import: a table that grows may move its rows, and the watch does not follow them.
fn watch_addr(g: &game::game::Game, spec: &str) -> *const f64 {
    let w: Vec<&str> = spec.split(':').collect();
    let n = |i: usize| -> usize { w.get(i).and_then(|x| x.parse().ok()).unwrap_or(0) };
    match w[0] {
        "time" => &g.time,
        "ch.pos" => &g.ch[game::ids::CharId(n(1) as u32)].pos[n(2)],
        "ch.vel" => &g.ch[game::ids::CharId(n(1) as u32)].vel[n(2)],
        "box.c" => &g.tab.boxes[game::ids::BoxId(n(1) as u32)].c[n(2)],
        "box.h" => &g.tab.boxes[game::ids::BoxId(n(1) as u32)].h[n(2)],
        x => panic!("GAME_RUN_WATCH: no arm for `{}` (rust/game_run/src/main.rs watch_addr)", x),
    }
}

/// HARNESS STAND-IN: THE HOST'S HALF BETWEEN TWO TICKS. `Game.update` round the step and the headless host's own asks are not run on the Rust side, so what
/// they wrote is read out of the TypeScript's state as tick k opens: the record the host sampled (`cmd_buf`), the host's own camera and Input (every one that
/// is no seat's), and the dialogue's room memo once the HUD's asks have filled it. NOTHING ELSE of that state is taken.
fn host_half(g: &mut game::game::Game, dir: &str, k: usize) -> game::cmd::PlayerCmd {
    if k == 1 { return g.cmd_buf.clone().expect("the start holds no record of the tick's input"); }
    let h = game::io::import(&std::fs::read(format!("{}/c{}.bin", dir, k)).unwrap_or_else(|_| panic!("c{}.bin missing", k)));
    let seat_cams: Vec<game::ids::CameraId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].cam).collect();
    for c in g.tab.camera.ids().collect::<Vec<_>>() { if !seat_cams.contains(&c) && c.ix() < h.tab.camera.len() { g.tab.camera[c] = h.tab.camera[c].clone(); } }
    let seat_ins: Vec<game::ids::InputId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].input).collect();
    for i in g.tab.input.ids().collect::<Vec<_>>() { if !seat_ins.contains(&i) && i.ix() < h.tab.input.len() { g.tab.input[i] = h.tab.input[i].clone(); } }
    if g.m.dialogue.room_rects.is_none() { g.m.dialogue.room_rects = h.m.dialogue.room_rects.clone(); }
    g.cmd_buf = h.cmd_buf.clone();
    h.cmd_buf.expect("the TypeScript state holds no record of the tick's input")
}

/// HARNESS STAND-IN: THE TEST DRIVER'S ORDERS. The scripted player gives its orders through the menu's own call between ticks; ts-ticks.ts writes each as a line
/// of gives.txt, `<tick> <id> <who> <kind> <x> <y> <z> [door <place> <side> | man <place>]`, and the same call is made here before that tick.
fn give(g: &mut game::game::Game, k: usize, l: &str) {
    use game::orders::{OrderTarget as T, Who};
    let w: Vec<&str> = l.split(' ').collect();
    if l.contains("UNSUPPORTED") || w.len() < 7 { println!("NOTE tick {}: an order of the driver's this runner cannot give: {}", k, l); return; }
    let at = [w[4].parse::<f64>().unwrap(), w[5].parse::<f64>().unwrap(), w[6].parse::<f64>().unwrap()];
    let who = match w[2] {
        "-" => None, "all" => Some(Who::All), "red" => Some(Who::Red), "blue" => Some(Who::Blue),
        x => match x.parse::<f64>() { Ok(n) => Some(Who::Num(n)), Err(_) => { println!("NOTE tick {}: a `who` this runner cannot give: {}", k, l); return; } },
    };
    let place = |i: usize| w[i].parse::<usize>().unwrap();
    let t = match w[3] {
        "floor" => { let serial = g.serial(); T::Floor(game::orders::OrderTargetFloor { at, wall: None, thing: None, serial }) }
        "door" => T::Door(game::orders::OrderTargetDoor { at, door: game::ids::DoorId::from_ix(place(8)), side: w[9].parse().unwrap() }),
        "none" => T::None(game::orders::OrderTargetNone { at, man: if w.len() > 8 { Some(g.guards[place(8)]) } else { None } }),
        _ => T::Foe(game::orders::OrderTargetFoe { at, foe: g.guards[place(8)] }),
    };
    let why = game::orders::squad_orders_give(g, w[1], Some(&t), who, None);
    println!("NOTE tick {}: the driver's order `{}` -> {:?}", k, l, why);
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 { eprintln!("usage: game_run <dir> <ticks> [--lockstep]"); std::process::exit(64); }
    let dir = a[1].clone();
    let ticks: usize = a[2].parse().expect("ticks");
    let lockstep = a.iter().any(|x| x == "--lockstep");
    let rd = |n: &str| std::fs::read(format!("{}/{}", dir, n)).unwrap_or_else(|_| panic!("{}/{} missing", dir, n));
    let start = rd("c1.bin");
    let block0 = rd("s0.bin");
    let engine = String::from_utf8(rd("engine.txt")).unwrap();
    // host.txt, a line a tick: dt · the probe's asks pending · the game's time · the die's count · how often the host listed the box world again before this tick
    let host: Vec<Vec<f64>> = String::from_utf8(rd("host.txt")).unwrap().lines().map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect()).collect();
    let gives: Vec<String> = std::fs::read_to_string(format!("{}/gives.txt", dir)).unwrap_or_default().lines().map(String::from).collect();
    let knobs = std::fs::read(format!("{}/knobs.bin", dir)).ok();
    let len = core::mem::size_of::<sim::state::State>();
    if block0.len() != len { println!("NOTE the block handed over is {} bytes and this build's State is {}: rust/sim and src/sim are not of one build", block0.len(), len); }

    std::panic::set_hook(Box::new(|info| {
        println!("PANIC {}", TICK.load(Ordering::Relaxed));
        println!("{}", info);
        let bt = std::backtrace::Backtrace::force_capture().to_string();
        let mut n = 0;
        for l in bt.lines() {
            let l = l.trim();
            if let Some(i) = l.find(": ") { let f = &l[i + 2..]; if (f.starts_with("game::") || f.starts_with("sim::") || f.starts_with("<game::")) && n < 24 { println!("   {}", f); n += 1; } }
        }
        let _ = std::io::stdout().flush();
    }));

    // (an import walks a whole game by recursion and the tick's frames are a debug build's: a deep stack)
    let h = std::thread::Builder::new().stack_size(2 << 30).spawn(move || {
        let mut g = Box::new(game::io::import(&start));
        unsafe {
            let at = sim::state::addr(0) as *mut u8;
            core::ptr::write_bytes(at, 0, len);
            core::ptr::copy_nonoverlapping(block0.as_ptr(), at, block0.len().min(len));
        }
        g.sim = 0;
        // HARNESS STAND-INS for what the bridge does not carry (tools/blitz/README.md "Host steps"): rust/sim's module-level tables, the orders' closures, `Game.engine`.
        game::anim_skeleton::b_01::bringup_module_state(&mut g, None);
        let unknown = game::orders::b_01::orders_closures_again(&mut g);
        if !unknown.is_empty() { println!("NOTE orders with no closures dealt (no variant for them): {:?}", unknown); }
        let unknown = game::host_api::b_01::host_engine_fill(&mut g, &engine);
        if !unknown.is_empty() { println!("NOTE engine.txt: {} lines not taken: {:?}", unknown.len(), &unknown[..unknown.len().min(12)]); }
        if let Ok(spec) = std::env::var("GAME_RUN_WATCH") {
            let p = watch_addr(&g, &spec);
            unsafe { GAME_RUN_WATCH_ADDR = p as usize; }
            println!("NOTE watching {} at {:p} = {}", spec, p, unsafe { *p });
            game_run_watch_ready();
        }
        let stdin = std::io::stdin();
        for k in 1..=ticks {
            TICK.store(k, Ordering::Relaxed);
            let cmd = host_half(&mut g, &dir, k);
            for l in gives.iter().filter(|l| l.split(' ').next().and_then(|x| x.parse::<usize>().ok()) == Some(k)) { give(&mut g, k, l); }
            let row = &host[k - 1];
            // HARNESS STAND-IN: the headless host's `engine.world.upload()` between ticks lists the boxes again (the cursor's ray reads that list)
            if row.get(4).copied().unwrap_or(0.0) > 0.0 { game::host_api::b_01::host_world_upload(&mut g); }
            game::game::game_step(&mut g, row[0], &cmd);
            if k == 1 { if let Some(want) = &knobs {
                // (a check of rustknobs::knobs_to_rust: after the first tick the module's table of knobs is the TypeScript module's, to the bit)
                let n = sim::knobs::len(0); let got = unsafe { std::slice::from_raw_parts(sim::knobs::addr(0) as *const f64, n) };
                let bad: Vec<usize> = (0..n.min(want.len() / 8)).filter(|i| got[*i].to_bits() != f64::from_le_bytes(want[i * 8..i * 8 + 8].try_into().unwrap()).to_bits()).collect();
                if !bad.is_empty() || n * 8 != want.len() { println!("NOTE knobs: {} of {} lines written by knobs_to_rust differ from the TypeScript module's table ({} there): lines {:?}", bad.len(), n, want.len() / 8, &bad[..bad.len().min(12)]); }
            } }
            std::fs::write(format!("{}/n{}.bin", dir, k), game::io::export(&g)).unwrap();
            let b = unsafe { std::slice::from_raw_parts(sim::state::addr(0) as *const u8, len) };
            std::fs::write(format!("{}/ns{}.bin", dir, k), b).unwrap();
            let d = &sim::state::state(0).dice;
            println!("TICK {} time {} die {} {}", k, g.time, d.n, d.s);
            let _ = std::io::stdout().flush();
            if lockstep && k < ticks {
                let mut line = String::new();
                if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 || line.trim() != "go" { return; }
            }
        }
    }).unwrap();
    std::process::exit(if h.join().is_ok() { 0 } else { 2 });
}
