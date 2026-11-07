//! BLASTCOST: what the tick of a wall blast costs the game, natively, against a quiet tick. No page and no wire: the Rust step alone.
//!
//!   blastcost [--runs N] [--one] [--no-dress]
//!
//! Stands The Pass as a room clear with the squad of four, has the red pair set a charge on one outer wall of the bunk room and the blue pair on another, by the game's
//! own order, and blows both in one tick (a player's "simultaneous breach"; `--one` blows only the first). It prints, in ms: the quiet
//! tick's median over the 120 frames before, the blow's call, the tick after it, the worst of the next 60 ticks, for each run and their medians. With the rows that
//! dress the picture up, as a page has them (`--no-dress`: down, as a host that draws nothing).
use game_start::play::{Opts, Play};
use std::time::Instant;

fn ms(t: Instant) -> f64 { t.elapsed().as_secs_f64() * 1000.0 }
fn median(mut v: Vec<f64>) -> f64 { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] }

fn main() {
    let mut runs = 5usize; let mut both = true; let mut dress = true;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() { "--runs" => runs = it.next().unwrap().parse().unwrap(), "--one" => both = false, "--no-dress" => dress = false, x => panic!("what is {x}?") }
    }
    let (mut quiet, mut blow, mut after, mut worst) = (vec![], vec![], vec![], vec![]);
    for run in 0..runs {
        let mut p = Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: 1 + run as u32, dress, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.1);
        game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
        // the bunk room: its east wall by the red pair, its south wall by the blue
        for (at, who) in [([34.12, 0.0, 15.0], game::wallcharge::WallChargeOrderWho::Red), ([31.0, 0.0, 18.12], game::wallcharge::WallChargeOrderWho::Blue)] {
            if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, at, Some(who)) { panic!("run {run}: the charge at {at:?} was refused: {why}"); }
        }
        let want = if both { 2 } else { 1 };
        if p.until(120.0, |p| p.g.wall_charge.set.len() >= want).is_none() {   // (the defenders can stop them: no wall blast to time on this seed)
            println!("run {run}: two minutes and {} of {want} on the wall, so nothing to time", p.g.wall_charge.set.len());
            continue;
        }
        let set: Vec<_> = p.g.wall_charge.set.clone();
        let mut q = vec![];
        for _ in 0..120 { let t = Instant::now(); p.frame(); q.push(ms(t)); }
        // BLASTCOST_PERF=<file>: `perf record` this process from here to the end of the run (the stand is not in it)
        let perf = std::env::var("BLASTCOST_PERF").ok().map(|f| {
            let ch = std::process::Command::new(std::env::var("BLASTCOST_PERF_BIN").unwrap_or_else(|_| String::from("perf"))).args(["record", "-e", "cpu-clock", "-F", "4000", "--call-graph", "dwarf,16384", "-o", &format!("{f}.{run}"), "-p", &std::process::id().to_string()])
                .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().expect("perf");
            std::thread::sleep(std::time::Duration::from_millis(400));
            ch
        });
        let t = Instant::now();
        for c in set.iter().take(want) { game::wallcharge::wall_charge_blow(&mut p.g, Some(*c), Some(true)); }
        let b = ms(t);
        let t = Instant::now(); p.frame(); let a = ms(t);
        let mut w = 0.0f64; let mut trace = vec![];
        for _ in 0..60 { let t = Instant::now(); p.frame(); let e = ms(t); w = w.max(e); trace.push(e); }
        if let Some(mut ch) = perf { let _ = std::process::Command::new("kill").args(["-INT", &ch.id().to_string()]).status(); let _ = ch.wait(); }
        let qm = median(q);
        println!("run {run}: quiet {qm:.2}  blow {b:.2}  next tick {a:.2}  worst of 60 after {w:.2}   ({} pieces in the pool, {} of rubble)", p.g.destr.debris.list.len(), p.g.destr.edge_look.rubble.len());
        if std::env::var("BLASTCOST_TRACE").is_ok() { println!("  the 60 after: {}", trace.iter().map(|e| format!("{e:.1}")).collect::<Vec<_>>().join(" ")); }
        quiet.push(qm); blow.push(b); after.push(a); worst.push(w);
    }
    if quiet.is_empty() { println!("no run had its charges on the wall: nothing timed"); return; }
    println!("median of {}: quiet {:.2}  blow {:.2}  next tick {:.2}  worst after {:.2}", quiet.len(), median(quiet), median(blow), median(after), median(worst));
}
