//! The Rust start of a level file, written out for the compare with the TypeScript start (tools/blitz/start.sh; README.md "The compare").
//!
//!   start --lacks <level file>…        a file a line: what the start does not carry of it
//!   start <level file> --out DIR [--seed N] [--seats 2] [--scenario roomclear|nightbreach|rescue|quiet [--squad N] [--deal N] | --link '?words'] [--cpu] [--frames N [--beside]]
//!
//! Stands the level by `game_start::stand` and writes into DIR `n1.bin` (the whole state, by the bridge's writer) and `ns0.bin` (rust/sim's block),
//! the names tools/blitz/diff.ts reads beside the TypeScript start's `c1.bin` and `s0.bin`. With `--frames N` it then runs N frames of a host with
//! an idle hand and says the die's count and a hash of each part of the block every 60th frame.
//!
//! `--beside`: THE SAME FRAMES ON THE TYPESCRIPT'S START, IN RUST. DIR's `c1.bin` and `s0.bin` (the TypeScript start, tools/blitz/start.ts) are
//! imported into a second block and that game is stepped by the same host frame beside this one; after every frame the die's count and every part
//! of the two blocks are compared, and the run stops at the first frame that parts (exit 1). It holds what the path diff cannot see: state the
//! bridge does not carry, and what the import mends as it reads. The two states as the run ends are written (`end-n.bin`, `end-b.bin`).
use game_start::{block, files, json, stand};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let opt = |n: &str| a.iter().position(|x| x == n).and_then(|i| a.get(i + 1)).cloned();
    // `start --lacks <level file>…`: a file a line, what the start does not carry of it (a record or a member nobody reads), or why it could
    // not be read at all (a value of the wrong kind stops the reader with its place)
    if a.get(1).map(|x| x.as_str()) == Some("--lacks") {
        std::panic::set_hook(Box::new(|_| {}));
        for f in &a[2..] {
            let data = match json::parse(&files::level_text(f)) { Ok(d) => d, Err(e) => { println!("{}\tNOT READ\t{}", f, e); continue; } };
            if !data.has("boxes") { println!("{}\tnot a level file\t", f); continue; }
            match std::panic::catch_unwind(|| game_start::level::lacks_of(&data)) {
                Ok(lacks) => println!("{}\t{}\t{}", f, if lacks.is_empty() { "carried" } else { "NOT CARRIED" }, lacks.join(" · ")),
                Err(e) => println!("{}\tNOT READ\t{}", f, e.downcast_ref::<String>().cloned().unwrap_or_default().replace('\n', " ")),
            }
        }
        return;
    }
    let (Some(file), Some(out)) = (a.get(1).filter(|f| !f.starts_with("--")), opt("--out")) else { eprintln!("start <level file> --out DIR [--seed N] [--scenario <kind> [--squad N] [--deal N]] [--cpu] [--frames N [--beside]]"); std::process::exit(64); };
    let seed: u32 = opt("--seed").map(|s| s.parse().expect("--seed")).unwrap_or(20942);
    let link = opt("--link");
    let seats: u32 = opt("--seats").map(|s| s.parse().expect("--seats")).unwrap_or(1);
    let frames: usize = opt("--frames").map(|s| s.parse().expect("--frames")).unwrap_or(0);
    let beside = a.iter().any(|x| x == "--beside");
    let cpu = a.iter().any(|x| x == "--cpu");
    let scenario = opt("--scenario").map(|k| {
        use game::scene_scenario_list::{ScenarioSpec, ScenarioSpecKind as K};
        let kind = match k.as_str() { "roomclear" => K::Roomclear, "nightbreach" => K::Nightbreach, "rescue" => K::Rescue, "quiet" => K::Quiet, k => { eprintln!("start: no scenario kind '{}'", k); std::process::exit(64) } };
        ScenarioSpec { kind, squad: opt("--squad").map(|s| s.parse().expect("--squad")).unwrap_or(4.0), level: None, seed: opt("--deal").map(|s| s.parse().expect("--deal")) }
    });
    let (file, out) = (file.clone(), out.clone());
    // (the constructor's frames are deep in a build with no optimisation: a thread with room)
    let h = std::thread::Builder::new().stack_size(1 << 30).spawn(move || {
        let ch = files::CharacterFiles::read();
        let data = json::parse(&files::level_text(&file)).unwrap_or_else(|e| panic!("{}: {}", file, e));
        let mut s = stand::stand(&data, ch.assets(), &stand::StartOpts { seed, seats, scenario, link, cpu_senses: cpu, ..Default::default() });
        std::fs::create_dir_all(&out).unwrap();
        let len = block::len();
        std::fs::write(format!("{}/n1.bin", out), game::io::export(&s.game)).unwrap();
        std::fs::write(format!("{}/ns0.bin", out), block::bytes(0)).unwrap();
        println!("start: '{}' stood by the Rust start, die seed {}: {} guards, {} boxes, {} doors; die n={}; state in {}/n1.bin", s.level.name, seed, s.game.guards.len(), s.level.boxes.len(), s.game.doors.list.len(), sim::state::state(0).dice.n, out);
        // the TypeScript's start as a second game in rust/sim's slot 1
        let mut beside = if beside {
            let (state, blk) = (std::fs::read(format!("{}/c1.bin", out)).expect("--beside: c1.bin of the TypeScript start"), std::fs::read(format!("{}/s0.bin", out)).expect("--beside: s0.bin"));
            let mut g = Box::new(game::io::import(&state));
            unsafe { let at = sim::state::addr(1) as *mut u8; core::ptr::write_bytes(at, 0, len); core::ptr::copy_nonoverlapping(blk.as_ptr(), at, blk.len().min(len)); }
            game::io::set_slot(&mut g, 1);
            // (the TypeScript's state does not hold its host's Input and camera before a first frame: this host makes its own, as the start does)
            let hb = stand::host_new(&mut g);
            Some((g, hb))
        } else { None };
        for k in 1..=frames {
            stand::frame(&mut s.game, &s.host, 1.0 / 60.0);
            if let Some((gb, hb)) = beside.as_mut() {
                stand::frame(gb, hb, 1.0 / 60.0);
                let (off, na, nb) = (block::parts_that_differ(0, 1), sim::state::state(0).dice.n, sim::state::state(1).dice.n);
                if !off.is_empty() || na != nb || k == frames {
                    // (the two games as the run ends, under names of their own: `n1.bin` stays the start)
                    std::fs::write(format!("{}/end-n.bin", out), game::io::export(&s.game)).unwrap();
                    std::fs::write(format!("{}/end-b.bin", out), game::io::export(gb)).unwrap();
                    std::fs::write(format!("{}/end-ns.bin", out), block::bytes(0)).unwrap();
                    std::fs::write(format!("{}/end-bs.bin", out), block::bytes(1)).unwrap();
                }
                if !off.is_empty() || na != nb {
                    println!("BESIDE: the two games PART at frame {}: the die n={} (Rust start) / {} (TypeScript start); parts of the block that differ: {:?} of {}; the two states in {}/end-n.bin and end-b.bin", k, na, nb, off.iter().map(|&i| block::PART_NAMES[i]).collect::<Vec<_>>(), sim::state::PARTS.len(), out);
                    std::process::exit(1);
                }
                if k == frames { println!("BESIDE: {} frames, the die's count and every part of the block the same after every frame (die n={}, time {:.4})", frames, na, s.game.time); }
            }
            if beside.is_none() && (k % 60 == 0 || k == frames) {
                println!("frame {} time {:.4} die n={} parts {}", k, s.game.time, sim::state::state(0).dice.n, block::part_hashes(0).iter().map(|h| h.to_string()).collect::<Vec<_>>().join(" "));
            }
        }
    }).unwrap();
    std::process::exit(if h.join().is_ok() { 0 } else { 2 });
}
