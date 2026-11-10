//! MISSIONS: what a change of play does to the room clears and to the rescues and the night breach, read the same way before and after. Natively, with no page and no TypeScript (README.md, "Missions").
//!
//!   missions [--seeds 1-8] [--secs 75] [--std | --hard] [--played] [LEVEL…]      play, and print a row a mission and difficulty, tab between columns (keep it in a file)
//!   missions diff BEFORE AFTER                                        two kept files side by side: a line for each row that differs, and the sums
//!
//! Each mission is stood from its address as a click on its row stands it, Sam unhurt. A commander who is
//! no tactician gives the squad an order every four to seven seconds, by a die of his own: "open and clear" a door by one of the enemies nearest them, then "move" onto
//! one, by turns. He is the same for every build, which is all that is asked of him: a row is for setting beside the same row of another build, never a claim about how hard a mission is.
//! `--played`: no commander; the hands-off player (hands_off.rs) plays it to its end, as the gate's `played` rows have him, 240 seconds at most. He looks, waits and
//! fights beside his men, so it is slower (a run is some six seconds) and nearer a player's evening. `--secs` is not asked.
//!
//! WHAT A ROW SAYS, summed over the seeds: the squad's men up at the end of those staged · enemies left of those staged · floors cleared · when the first of the squad
//! fell (median, and in how many runs) · when the floor was clear (median) · rounds fired by each side · magazine changes begun by each side · how long open ways onto floor nobody has been on were within 4 m of
//! a man of the squad, and how much of that a gun of theirs was on them (a second a way) · the enemy man-seconds each
//! rule of the list had a man · how many of the squad's men who fell were shot from behind (120 degrees and more off where they looked) · how many by one of
//! their own · with `--played`, how many runs did not end with the floor taken · how many fell in the line of a door a mate had opened in the last 1.5 s · how many
//! to a man on floor nobody had looked over 0.75 s before, and how many of those in the room he was in himself. Eight seeds of the seventeen at both difficulties (272 runs) are four minutes on four cores.
//!
//! WHAT IT DOES NOT SAY: anything of stealth, of Sam's own gun, of the hostage in a rescue, or of what a commander who waits and looks would meet. On one seed a floor is a coin toss:
//! read eight, and read the sums first.
use game::dice::Die;
use game::ids::{DoorId, GuardId};
use game::orders::Who;
use game::teams::Team;
use game_start::play::{dist, room_clear_files, Opts, Play};
use game_start::hands_off::{HandsOff, Plan};
use game_start::player::Script;
use std::collections::BTreeMap;

/// every room clear of the mission list by its level, and by their ids the rescues and the night breach
fn every_mission() -> Vec<String> {
    use game::scene_scenarios::MissionKind::{Nightbreach, Rescue};
    let clears = room_clear_files().into_iter().map(|f| f.trim_end_matches(".json").to_string());
    clears.chain(game::scene_scenario_list::scenarios().into_iter().filter(|e| matches!(e.kind, Nightbreach | Rescue)).map(|e| e.id)).collect()
}

/// the open ways near the squad are counted every so many frames, within so far of a man
const WAYS_EVERY: usize = 10;
const WAYS_M: f64 = 4.0;

/// what one run came to
#[derive(Clone, Debug, Default, PartialEq)]
struct Run { behind: usize, ours: usize, cap: usize, to: [usize; 3], up: usize, squad: usize, left: usize, enemies: usize, fell: Option<f64>, clear: Option<f64>, rounds: [f64; 2], reloads: [u32; 2], ways: [f64; 2], had: BTreeMap<String, f64> }

fn file_of(level: &str) -> String { format!("{level}.json") }

fn enemies(p: &Play) -> Vec<GuardId> { p.g.guards.iter().copied().filter(|&m| p.g.gd[m].team == Team::Hostile).collect() }

/// The commander: an order every four to seven seconds, onto one of the three men nearest the squad, "open and clear" the door nearest that man (or the next one) or
/// "move" onto him, by turns. His die is his own, so two seeds are two fights: on a fixed beat onto the nearest man the seeds of a mission were one fight over again.
struct Commander { die: Die, next: usize, clear: bool }

impl Commander {
    fn new(seed: u32) -> Commander { Commander { die: Die::new(0xc0deu32.wrapping_add(seed.wrapping_mul(7919))), next: 30, clear: true } }
    fn one_of(&mut self, mut v: Vec<[f64; 2]>, to: [f64; 2], n: usize) -> Option<[f64; 2]> {
        v.sort_by(|a, b| dist(*a, to).partial_cmp(&dist(*b, to)).unwrap());
        v.truncate(n);
        let i = (self.die.roll() * v.len() as f64) as usize;
        v.get(i).copied()
    }
    /// false: nobody is left to order, or to be sent at
    fn order(&mut self, p: &mut Play) -> bool {
        let Some(lead) = p.mates().into_iter().find(|&m| p.up(m)) else { return false };
        let men = enemies(p).into_iter().filter(|&m| p.up(m)).map(|m| p.at(m)).collect();
        let Some(man) = self.one_of(men, p.at(lead), 3) else { return false };
        let doors = p.g.doors.list.iter().map(|&d| { let c = p.g.dr[d].frame_centre; [c[0], c[2]] }).collect();
        let to = if self.clear { self.one_of(doors, man, 2).unwrap_or(man) } else { man };
        let target = game::orders::squad_orders_target_at(&mut p.g, [to[0], 0.0, to[1]]);
        let _ = game::orders::squad_orders_give(&mut p.g, if self.clear { "openClear" } else { "move" }, Some(&target), Some(Who::All), None);
        self.clear = !self.clear;
        true
    }
    fn tick(&mut self, p: &mut Play, f: usize) -> bool {
        if f < self.next { return true; }
        self.next = f + 240 + (self.die.roll() * 180.0) as usize;
        self.order(p)
    }
}

/// WHAT A MAN OF THE SQUAD FELL TO: a door opened by somebody else in the last moments, and he in its line; floor nobody had looked over.
#[derive(Default)]
struct Moments { shut: BTreeMap<DoorId, bool>, opened: Vec<(f64, DoorId, Option<GuardId>)>, on_looked: BTreeMap<GuardId, Vec<(f64, bool)>>, told: usize }

/// a door is open for the first time this long · a man is on floor looked over if he was this long before the round that killed
const DOOR_SECS: f64 = 1.5;
const LOOKED_SECS: f64 = 0.75;

impl Moments {
    fn cell(p: &Play, at: [f64; 2]) -> usize { let n = &p.g.col.nav; (at[1] / n.cell) as usize * n.w as usize + (at[0] / n.cell) as usize }
    fn tick(&mut self, p: &Play, f: usize) {
        let t = p.time();
        for &d in &p.g.doors.list {
            let shut = game::doors::door_angle(&p.g, d).abs() < 0.12;
            if self.shut.insert(d, shut) == Some(true) && !shut {
                let c = p.g.dr[d].frame_centre;
                let by = p.mates().into_iter().filter(|&m| p.up(m)).map(|m| (dist(p.at(m), [c[0], c[2]]), m)).filter(|x| x.0 < 2.0).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).map(|x| x.1);
                self.opened.push((t, d, by));
            }
        }
        if f % 6 != 0 { return; }
        for m in enemies(p) {
            let on = p.g.cleared.looked.get(Self::cell(p, p.at(m))).is_some_and(|&v| v != 0);
            let v = self.on_looked.entry(m).or_default();
            v.push((t, on));
            if v.len() > 40 { v.remove(0); }
        }
    }
    /// of the squad's men who have fallen since it was last asked: in the line of a door a mate opened · shot from floor nobody had looked over, and of that how many from the room they were in
    fn fell(&mut self, p: &Play, names: &[String]) -> [usize; 3] {
        let mut n = [0; 3];
        let all = p.recap_deaths();
        for d in all.iter().skip(self.told).filter(|d| names.contains(&d.name)) {
            let (Some(from), me) = (d.by_at, p.mates().into_iter().find(|&m| p.g.gd[m].callsign == d.name)) else { continue };
            let l = dist(from, d.at).max(1e-6);
            if self.opened.iter().any(|&(t, door, by)| {
                let c = p.g.dr[door].frame_centre;
                let u = ((c[0] - from[0]) * (d.at[0] - from[0]) + (c[2] - from[1]) * (d.at[1] - from[1])) / (l * l);
                let q = [from[0] + (d.at[0] - from[0]) * u, from[1] + (d.at[1] - from[1]) * u];
                d.t - t < DOOR_SECS && d.t >= t && by.is_some() && by != me && u > 0.0 && u < 1.0 && dist(q, [c[0], c[2]]) < p.g.dr[door].def.width / 2.0 + 0.1
            }) { n[0] += 1; }
            let Some(k) = enemies(p).into_iter().find(|&k| p.id_of(k) == d.by) else { continue };
            let Some(&(_, on)) = self.on_looked.get(&k).and_then(|v| v.iter().rev().find(|x| x.0 <= d.t - LOOKED_SECS)) else { continue };
            if on { continue; }
            n[1] += 1;
            let room = |at: [f64; 2]| p.g.cleared.room_of_cell.as_ref().and_then(|r| r.get(Self::cell(p, at)).copied()).unwrap_or(0);
            if room(from) != 0 && room(from) == room(d.at) { n[2] += 1; }
        }
        self.told = all.len();
        n
    }
}

/// the seconds after which the hands-off player stops
const CAP: f64 = 240.0;

fn play(level: &str, hard: bool, seed: u32, secs: f64, played: bool) -> Run {
    let (file, link) = match game::scene_scenario_list::all_missions().iter().find(|e| e.id == level) {
        Some(e) => (e.level.clone().unwrap_or_else(|| file_of("office")), game::scene_scenario_list::pick_search(e, "")),
        None => (file_of(level), format!("?levelFile={level}.json&roomclear&squad4")),
    };
    let mut p = Play::mission(&file, &format!("{link}{}", if hard { "&hard" } else { "" }), &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    let mut r = Run { squad: p.mates().len(), enemies: enemies(&p).len(), ..Default::default() };
    let mut was: BTreeMap<u32, (bool, f64)> = BTreeMap::new();
    let t0 = p.time();
    let mut owes = game_start::owes::Owes::new();
    let mut commander = Commander::new(seed);
    let mut moments = Moments::default();
    let names: Vec<String> = p.mates().iter().map(|&m| p.g.gd[m].callsign.clone()).collect();
    let mut him = played.then(|| HandsOff::new(p.g.seats[0], Plan { seed, cap: CAP, ..Default::default() }));
    for f in 0..((if played { CAP + 5.0 } else { secs }) * 60.0) as usize {
        match &mut him {
            Some(h) => { if h.over() { break; } p.played_by(h); }
            None => { if !commander.tick(&mut p, f) { break; } p.frame(); }
        }
        owes.tick(&mut p.g);
        moments.tick(&p, f);
        for (n, x) in r.to.iter_mut().zip(moments.fell(&p, &names)) { *n += x; }
        let t = p.time() - t0;
        if r.fell.is_none() && p.mates().iter().any(|&m| !p.up(m)) { r.fell = Some(t); }
        for m in p.g.guards.clone() {
            let side = match p.g.gd[m].team { Team::Operative => 0, Team::Hostile => 1, _ => continue };
            let now = (p.reloading(m).is_some(), p.rounds_fired(m));
            let (reloading, fired) = *was.get(&m.0).unwrap_or(&now);
            if now.0 && !reloading { r.reloads[side] += 1; }
            if now.1 > fired { r.rounds[side] += now.1 - fired; }
            was.insert(m.0, now);
            if side == 1 && !game::condition::down(&p.g, p.g.gd[m].ch) { if let Some(rule) = p.rule(m) { *r.had.entry(rule.to_string()).or_default() += 1.0 / 60.0; } }
        }
        if f % WAYS_EVERY == 0 { for (_, on) in p.watched(WAYS_M) { r.ways[0] += WAYS_EVERY as f64 / 60.0; if on { r.ways[1] += WAYS_EVERY as f64 / 60.0; } } }
        if r.clear.is_none() && enemies(&p).into_iter().all(|m| !p.up(m)) { r.clear = Some(t); }
    }
    r.behind = p.recap_deaths().iter().filter(|d| names.contains(&d.name) && d.off.abs() >= 120.0).count();
    r.ours = owes.friendly as usize;
    r.cap = him.as_ref().map_or(0, |h| (h.end() != Some(game_start::hands_off::End::Taken)) as usize);
    r.up = p.mates().into_iter().filter(|&m| p.up(m)).count();
    r.left = enemies(&p).into_iter().filter(|&m| p.up(m)).count();
    r
}

fn median(mut v: Vec<f64>) -> String {
    if v.is_empty() { return "-".into(); }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    format!("{:.1}", v[v.len() / 2])
}

const HEAD: [&str; 22] = ["level", "difficulty", "runs", "squad up", "squad", "enemies left", "enemies", "cleared", "first fell s", "fell in", "clear s", "rounds squad/enemy", "reloads squad/enemy", "ways s", "ways watched s", "rules man-secs", "behind", "by ours", "not taken", "a mate's door", "unlooked floor", "of their room"];

fn row(level: &str, hard: bool, runs: &[Run]) -> String {
    let sum = |f: &dyn Fn(&Run) -> f64| runs.iter().map(f).sum::<f64>();
    let mut had: BTreeMap<&str, f64> = BTreeMap::new();
    for r in runs { for (k, v) in &r.had { *had.entry(k).or_default() += v; } }
    let fell: Vec<f64> = runs.iter().filter_map(|r| r.fell).collect();
    let clear: Vec<f64> = runs.iter().filter_map(|r| r.clear).collect();
    [level.to_string(), if hard { "hard" } else { "standard" }.into(), runs.len().to_string(), sum(&|r| r.up as f64).to_string(), sum(&|r| r.squad as f64).to_string(), sum(&|r| r.left as f64).to_string(),
     sum(&|r| r.enemies as f64).to_string(), runs.iter().filter(|r| r.left == 0).count().to_string(), median(fell.clone()), fell.len().to_string(), median(clear),
     format!("{}/{}", sum(&|r| r.rounds[0]), sum(&|r| r.rounds[1])), format!("{}/{}", sum(&|r| r.reloads[0] as f64), sum(&|r| r.reloads[1] as f64)),
     format!("{:.0}", sum(&|r| r.ways[0])), format!("{:.0}", sum(&|r| r.ways[1])),
     had.iter().map(|(k, v)| format!("{k} {v:.1}")).collect::<Vec<_>>().join(" · "), sum(&|r| r.behind as f64).to_string(), sum(&|r| r.ours as f64).to_string(), sum(&|r| r.cap as f64).to_string(),
     sum(&|r| r.to[0] as f64).to_string(), sum(&|r| r.to[1] as f64).to_string(), sum(&|r| r.to[2] as f64).to_string()].join("\t")
}

/// `1-4,7` as 1 2 3 4 7
fn seeds_of(word: &str) -> Vec<u32> {
    let n = |x: &str| x.parse::<u32>().unwrap_or_else(|_| fail(&format!("--seeds: '{x}' is no number")));
    word.split(',').flat_map(|p| match p.split_once('-') { Some((a, b)) => (n(a)..=n(b)).collect::<Vec<_>>(), None => vec![n(p)] }).collect()
}

fn fail(why: &str) -> ! { eprintln!("missions: {why}"); std::process::exit(2) }

/// the columns that are numbers to be added up
const ADDED: [usize; 14] = [2, 3, 4, 5, 6, 7, 13, 14, 16, 17, 18, 19, 20, 21];

fn diff(before: &str, after: &str) {
    let read = |f: &str| -> BTreeMap<(String, String), Vec<String>> {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| fail(&format!("{f}: {e}")));
        text.lines().map(|l| l.split('\t').map(String::from).collect::<Vec<_>>()).filter(|c| c.len() == HEAD.len() && c[0] != HEAD[0]).map(|c| ((c[0].clone(), c[1].clone()), c)).collect()
    };
    let (a, b) = (read(before), read(after));
    if a.keys().ne(b.keys()) { fail("the two files are not of the same missions"); }
    let mut same = 0;
    for (k, x) in &a {
        let y = &b[k];
        let parts: Vec<String> = (2..HEAD.len()).filter(|&i| x[i] != y[i]).map(|i| format!("{} {} → {}", HEAD[i], x[i], y[i])).collect();
        if parts.is_empty() { same += 1; } else { println!("{} {}: {}", k.0, k.1, parts.join(" · ")); }
    }
    for d in ["standard", "hard"] {
        let sum = |m: &BTreeMap<(String, String), Vec<String>>, i: usize| m.iter().filter(|(k, _)| k.1 == d).map(|(_, r)| r[i].parse::<f64>().unwrap_or(0.0)).sum::<f64>();
        if sum(&a, 2) > 0.0 { println!("ALL {d}: {}", ADDED.iter().map(|&i| format!("{} {} → {}", HEAD[i], sum(&a, i), sum(&b, i))).collect::<Vec<_>>().join(" · ")); }
    }
    println!("{same} of {} rows the same", a.len());
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("diff") {
        match &args[1..] { [a, b] => return diff(a, b), _ => fail("diff BEFORE AFTER") }
    }
    let (mut seeds, mut secs, mut both, mut levels) = (seeds_of("1-8"), 75.0, vec![false, true], Vec::new());
    let (mut one, mut played) = (false, false);
    let mut i = 0;
    while i < args.len() {
        let value = || args.get(i + 1).cloned().unwrap_or_else(|| fail(&format!("{} wants a value", args[i])));
        match args[i].as_str() {
            "--seeds" => { seeds = seeds_of(&value()); i += 1; }
            "--secs" => { secs = value().parse().unwrap_or_else(|_| fail("--secs N")); i += 1; }
            "--std" => both = vec![false],
            "--hard" => both = vec![true],
            "--played" => played = true,
            "--one" => one = true,   // (a child: this row and no other, in this process)
            w if w.starts_with('-') => fail(&format!("no flag '{w}'")),
            l => levels.push(l.to_string()),
        }
        i += 1;
    }
    let every = every_mission();
    if levels.is_empty() { levels = every.clone(); }
    for l in &levels { if !every.contains(l) { fail(&format!("no mission '{l}' ({})", every.join(" "))); } }
    if one {
        let runs: Vec<Run> = seeds.iter().map(|&s| play(&levels[0], both[0], s, secs, played)).collect();
        println!("{}", row(&levels[0], both[0], &runs));
        return;
    }
    // a game a process at a time: a row is a child, as many at once as there are cores
    println!("{}", HEAD.join("\t"));
    let todo: Vec<(String, bool)> = both.iter().flat_map(|&h| levels.iter().map(move |l| (l.clone(), h))).collect();
    let words = seeds.iter().map(u32::to_string).collect::<Vec<_>>().join(",");
    let next = std::sync::atomic::AtomicUsize::new(0);
    let rows: Vec<std::sync::Mutex<String>> = todo.iter().map(|_| Default::default()).collect();
    let exe = std::env::current_exe().unwrap();
    std::thread::scope(|sc| {
        for _ in 0..std::thread::available_parallelism().map_or(4, |n| n.get()) {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some((level, hard)) = todo.get(k) else { break };
                let mut child = std::process::Command::new(&exe);
                child.args(["--one", "--seeds", &words, "--secs", &secs.to_string(), if *hard { "--hard" } else { "--std" }, level]);
                if played { child.arg("--played"); }
                let out = child.output().unwrap();
                if !out.status.success() { fail(&format!("{level}{}: {}", if *hard { " hard" } else { "" }, String::from_utf8_lossy(&out.stderr))); }
                *rows[k].lock().unwrap() = String::from_utf8_lossy(&out.stdout).lines().last().unwrap_or("").to_string();
            });
        }
    });
    for r in &rows { println!("{}", r.lock().unwrap()); }
}
