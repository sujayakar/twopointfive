//! THE ROWS: what each holds, its break, and how it is played (README.md, "The gate", has them in a table). All stand as the page and the server stand a game: seed 9, `dress`, through the
//! game's own start on the mission's own link. The list of missions is the game's.
use crate::pins::{Pins, Start};
use game::cmd::PlayerCmd;
use game::game::GuardState;
use game::scene_scenario_list::{all_missions, pick_search};
use game::scene_scenarios::ScenarioEntry;
use game_start::hand::{act, read_hand, Hand};
use game_start::moment::Moment;
use game_start::parts;
use game_start::play::{Opts, Play};
use game_start::{files, json};
use std::collections::BTreeMap;
use std::time::Instant;

/// the seed every row stands on
pub const SEED: u32 = 9;
/// seconds of a run, of a run of another level between two of it, and of the moments' turns
pub const RUN_SECS: usize = 40;
const BETWEEN_SECS: usize = 10;
const MOMENTS: [usize; 5] = [3, 9, 15, 21, 27];
/// the moments that are also loaded into a fresh slot
const LOADS: [usize; 2] = [9, 21];
const AFTER_MOMENT_SECS: usize = 3;
/// the idle seconds of a second start, and of the mission stood between the two
const IDLE_SECS: usize = 5;
const OTHER_SECS: usize = 2;
const SEATS_SECS: usize = 30;
/// a moment of a game of two, and how far its records are played from there
const SEATS_MOMENT: (usize, usize) = (12, 18);
const PAR_SECS: usize = 30;
/// the least a fight of two does in its seconds, at about half of the least the three runs give (a man walked 4 to 8 metres, the fights fired 2 to 8 rounds, the hands differed on two ticks of five)
const MEN_WALK: f64 = 2.0;
const FIGHT_FIRES: f64 = 1.0;
const HANDS_DIFFER_ONE_IN: usize = 5;
/// parts a line names, at most
const NAMED: usize = 12;

/// THE RUNS: a mission and a hand. Between them they reach the office's night breach, a room clear in the rain under bangs, a compound's doors, a glasshouse's kit, the kill house and a
/// quiet night under push, a rescue under push and snow under bangs.
pub const RUNS: [(&str, &str); 8] = [("night-breach", "doors"), ("depot-four", "bangs"), ("compound-four", "doors"), ("glasshouse-four", "kit"), ("killhouse-four", "push"), ("bazaar-quiet", "push"), ("rescue-registry", "push"), ("pass-four", "bangs")];
/// a game of two is held on these
const SEATS: [&str; 3] = ["room-clear-four", "compound-four", "depot-four"];
/// THE PARTS THAT ARE THE CLOCK'S OWN: a level with a par block and the same level without it may differ here, and nowhere else
const CLOCK_ONLY: [(&str, &str); 2] = [
    ("par", "the clock's own record, which runs from the squad's first order"),
    ("level", "the level's own record holds the block as the file has it, from the stand on"),
];

#[derive(Clone, Copy)]
pub struct Flags { pub brk: Option<&'static str>, pub pin: bool }
impl Flags {
    /// is this the break that was asked for
    pub fn breaks(&self, name: &str) -> bool { self.brk == Some(name) }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Job { Start(String, u32), Run(usize), Seats(String), Par(String) }
impl Job {
    /// which is started first, the longest kinds before the others: the lower, the sooner
    pub fn rank(&self) -> u32 { match self { Job::Run(_) => 0, Job::Seats(_) => 1, Job::Par(_) => 2, Job::Start(..) => 3 } }
}

#[derive(Clone)]
pub struct Row { pub name: String, pub job: Job }
impl Row {
    pub fn pinned(&self) -> bool { self.column().is_some() || self.series().is_some() }
    /// its column of the pins' starts
    pub fn column(&self) -> Option<String> { match &self.job { Job::Start(id, seats) => Some(column(id, *seats)), _ => None } }
    /// its line of the pins' series
    pub fn series(&self) -> Option<String> { match &self.job { Job::Run(_) if self.name.starts_with("play ") => Some(self.name.clone()), Job::Seats(_) => Some(self.name.clone()), _ => None } }
}
fn column(id: &str, seats: u32) -> String { format!("{}/{}", id, seats) }
fn run_name(k: usize) -> String { format!("{} {}", RUNS[k].0, RUNS[k].1) }
fn start_name(id: &str, seats: u32) -> String { format!("start {}, {}", id, if seats == 1 { "one player" } else { "two players" }) }
fn file_of(e: &ScenarioEntry) -> String { e.level.clone().unwrap_or_else(|| String::from("office.json")) }
fn has_par(e: &ScenarioEntry) -> bool { let t = files::level_text(&file_of(e)); t.contains("\"par\"") && json::parse(&t).is_ok_and(|d| d.get("scenarios").has("par")) }

/// every row, in the rows' order
pub fn all() -> Vec<Row> {
    let list = all_missions();
    let mut out = Vec::new();
    for e in &list { for seats in [1, 2] { out.push(Row { name: start_name(&e.id, seats), job: Job::Start(e.id.clone(), seats) }); } }
    for kind in ["play", "again", "save", "load", "draw"] { for k in 0..RUNS.len() { out.push(Row { name: format!("{} {}", kind, run_name(k)), job: Job::Run(k) }); } }
    for id in SEATS { out.push(Row { name: format!("seats {}", id), job: Job::Seats(id.to_string()) }); }
    for e in list.iter().filter(|e| has_par(e)) { out.push(Row { name: format!("par {}", e.id), job: Job::Par(e.id.clone()) }); }
    out
}
/// ONE FAULT A CHECK: the row, the break's name, and a piece of the check's own words its failure must hold
pub fn breaks() -> Vec<(String, &'static str, &'static str)> {
    let (run, moved) = (run_name(5), run_name(1));
    let start = start_name("tut-road", 1);
    let (seats, par) = (format!("seats {}", SEATS[0]), String::from("par killhouse-four"));
    vec![
        (start.clone(), "pin", "the start moved"), (start, "second", "a second start"),
        (format!("play {}", run), "pin", "parts from its pin"),
        (format!("again {}", run), "second", "the second run"),
        (format!("save {}", run), "block", "put back, parts"),
        (format!("load {}", run), "block", "loaded into"),
        // (a run whose statics move: the mirror has something to skip)
        (format!("draw {}", moved), "told", "drawing the game changed it"), (format!("draw {}", moved), "mark", "the page's"), (format!("draw {}", moved), "head", "the boxes' head says"), (format!("draw {}", moved), "opened", "just opened"),
        (seats.clone(), "pin", "parts from its pin"), (seats.clone(), "fresh", "a fresh game"), (seats.clone(), "moment", "a moment of the fight"), (seats, "idle", "the two players' hands"),
        (par.clone(), "moves", "the clock moved"), (par, "noclock", "the staging"),
    ]
}

/// a child's rows
pub fn one(names: &[String], f: Flags) {
    let rows = all();
    let mine: Vec<&Row> = names.iter().map(|n| rows.iter().find(|r| &r.name == n).unwrap_or_else(|| panic!("no row is called '{}'", n))).collect();
    let pins = Pins::read_file();
    match &mine[0].job {
        Job::Start(id, seats) => start_row(&mine[0].name, id, *seats, &pins, f),
        Job::Run(k) => run_rows(*k, &mine.iter().map(|r| r.name.split(' ').next().unwrap()).collect::<Vec<_>>(), &pins, f),
        Job::Seats(id) => seats_row(&mine[0].name, id, &pins, f),
        Job::Par(id) => par_row(&mine[0].name, id, f),
    }
}

/// the row's line, to the parent
pub fn say(name: &str, secs: f64, fail: &[String], note: &str) {
    let words = if fail.is_empty() { note.to_string() } else { fail.join("; ") };
    println!("row\t{}\t{}\t{:.2}\t{}", name, if fail.is_empty() { "PASS" } else { "FAIL" }, secs, words.replace(['\n', '\t'], " "));
}

fn mission(id: &str) -> ScenarioEntry { all_missions().into_iter().find(|e| e.id == id).unwrap_or_else(|| panic!("no mission is called '{}'", id)) }
fn opts(seed: u32, seats: u32) -> Opts { Opts { seed, seats, dress: true, may_refuse: true, ..Default::default() } }
fn stand(e: &ScenarioEntry, seed: u32, seats: u32) -> Play { Play::mission(&file_of(e), &pick_search(e, ""), &opts(seed, seats)) }

/// a refusal as the pins hold it: one line
fn clean(why: &str) -> String { why.replace(['\n', '\t'], " ").trim().to_string() }
/// names, at most so many of them
fn named(list: &[&str]) -> String {
    if list.len() <= NAMED { return list.join(", "); }
    format!("{} and {} more", list[..NAMED].join(", "), list.len() - NAMED)
}
/// which parts of the pin and of the game differ, and which are gone and new, by name
pub fn moved_parts(pin: &BTreeMap<String, u32>, now: &BTreeMap<String, u32>) -> String {
    let order = parts::names();
    let differ: Vec<&str> = order.iter().filter(|n| matches!((pin.get(*n), now.get(*n)), (Some(a), Some(b)) if a != b)).map(|n| n.as_str()).collect();
    let gone: Vec<&str> = pin.keys().filter(|n| !now.contains_key(*n)).map(|n| n.as_str()).collect();
    let new: Vec<&str> = now.keys().filter(|n| !pin.contains_key(*n)).map(|n| n.as_str()).collect();
    let mut said = Vec::new();
    if !differ.is_empty() { said.push(named(&differ)); }
    if !gone.is_empty() { said.push(format!("parts gone: {}", named(&gone))); }
    if !new.is_empty() { said.push(format!("new parts: {}", named(&new))); }
    said.join("; ")
}
fn by_name(words: &[u32]) -> BTreeMap<String, u32> { parts::names().into_iter().zip(words.iter().copied()).collect() }
pub fn word_of(words: &[u32]) -> u32 { let f = parts::fold(words); (f ^ (f >> 32)) as u32 }
fn hex(words: &[u32]) -> String { words.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(" ") }

// ---- start

/// THE START IS THE PINNED START, AND A SECOND START IN ONE PROCESS IS THE FIRST.
/// BREAKS: `pin` a word of the pin made wrong · `second` the second start's seed is one more.
fn start_row(name: &str, id: &str, seats: u32, pins: &Pins, f: Flags) {
    let t = Instant::now();
    let (e, col) = (mission(id), column(id, seats));
    let mut p = stand(&e, SEED, seats);
    let refused = p.refused.as_deref().map(clean);
    let first = p.words();
    if f.pin {
        match &refused {
            Some(why) => println!("pin\tstart\t{}\trefused\t{}", col, why),
            None => println!("pin\tstart\t{}\tparts\t{}", col, parts::names().iter().zip(&first).map(|(n, w)| format!("{}={:08x}", n, w)).collect::<Vec<_>>().join(" ")),
        }
        return say(name, t.elapsed().as_secs_f64(), &[], "");
    }
    let mut fail = Vec::new();
    match (pins.starts.get(&col), &refused) {
        (None, _) => fail.push(String::from("no pin: `gate pin` records it")),
        (Some(Start::Refused(a)), Some(b)) => if a != b { fail.push(format!("refused in other words: pinned '{}', now '{}' (pins.txt)", a, b)); },
        (Some(Start::Refused(a)), None) => fail.push(format!("it stands now, and is pinned as refused: '{}' (pins.txt)", a)),
        (Some(Start::Parts(_)), Some(b)) => fail.push(format!("it is refused now, and is pinned as standing: '{}' (pins.txt)", b)),
        (Some(Start::Parts(m)), None) => {
            let mut now = by_name(&first);
            if f.breaks("pin") { now.values_mut().next().map(|w| *w ^= 1); }
            let said = moved_parts(m, &now); if !said.is_empty() { fail.push(format!("the start moved in {} (pins.txt)", said)); }
        }
    }
    if refused.is_some() { return say(name, t.elapsed().as_secs_f64(), &fail, "refused as pinned"); }
    // the second start: what the first was, half a second at a time; another mission, in this process; and the first again
    let idle = |p: &mut Play| -> Vec<Vec<u32>> { (0..IDLE_SECS * 2).map(|_| { p.run(0.5); p.words() }).collect() };
    let was = idle(&mut p);
    drop(p);
    let (other, mut q) = before_it(&e, seats);
    q.run(OTHER_SECS as f64);
    drop(q);
    let mut again = stand(&e, if f.breaks("second") { SEED + 1 } else { SEED }, seats);
    let names = parts::names();
    let second = again.words();
    if second != first { fail.push(format!("a second start in one process, after {}, is another start in {}", other.id, named(&parts::differ(&names, &first, &second)))); }
    else if let Some((k, w)) = idle(&mut again).iter().zip(&was).enumerate().find(|(_, (a, b))| a != b).map(|(k, (a, b))| (k, parts::differ(&names, b, a))) {
        fail.push(format!("a second start in one process, after {}, parts {:.1} s on in {}", other.id, (k + 1) as f64 / 2.0, named(&w)));
    }
    say(name, t.elapsed().as_secs_f64(), &fail, "");
}
/// the mission before this one on the list that stands for that many players (the first's is the last on the list), stood
fn before_it(e: &ScenarioEntry, seats: u32) -> (ScenarioEntry, Play) {
    let list = all_missions();
    let at = list.iter().position(|x| x.id == e.id).unwrap();
    for back in 1..list.len() {
        let x = &list[(at + list.len() - back) % list.len()];
        let p = stand(x, SEED, seats);
        if p.refused.is_none() { return (x.clone(), p); }
    }
    panic!("no other mission stands for {} players", seats);
}

// ---- a run: play, again, save, draw

fn hand_of(name: &str, frames: usize) -> Hand {
    let file = game_start::hand::hands_dir().join(format!("{}.hand", name));
    read_hand(&std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {}", file.display(), e)), frames)
}
/// a run's mission stood, the hand's own word on Sam said
pub fn stand_run(k: usize, seed: u32) -> (Play, Hand) {
    let frames = RUN_SECS * 60;
    let hand = hand_of(RUNS[k].1, frames);
    let mut p = stand(&mission(RUNS[k].0), seed, 1);
    if let Some(why) = &p.refused { panic!("the run's mission is refused: {}", why); }
    p.g.god_mode = hand.unhurt;
    (p, hand)
}
/// one frame of a run: the hand's lines that fall due (a line the game refuses is counted by the caller's `did`), then the host's frame
fn frame_of(p: &mut Play, hand: &Hand, k: usize, next: &mut usize, did: &mut [usize]) {
    while *next < hand.at.len() && hand.at[*next].0 <= k {
        if act(p, &hand.at[*next].1).is_ok() { did[hand.at[*next].2] += 1; }
        *next += 1;
    }
    p.frame();
}

/// a moment of a run, what a host keeps beside it, and every frame's words of the seconds after it
struct Kept { at: usize, moment: Moment, next: usize, cursor: Option<[f64; 3]>, words: Vec<Vec<u32>> }

/// THE RUN IS PLAYED ONCE FOR ALL ITS ROWS, in a child
fn run_rows(k: usize, kinds: &[&str], pins: &Pins, f: Flags) {
    let name = |kind: &str| format!("{} {}", kind, run_name(k));
    let frames = RUN_SECS * 60;
    let t = Instant::now();
    let names = parts::names();
    let (mut p, hand) = stand_run(k, SEED);
    let mut did = vec![0; hand.lines.len()];
    let (mut w, mut row, mut next) = (game::bytes::W::new(), Vec::new(), 0);
    let (mut series, mut seconds): (Vec<u32>, Vec<Vec<u32>>) = (Vec::new(), Vec::new());
    let mut kept: Vec<Kept> = Vec::new();
    let keeps = (kinds.contains(&"save") || kinds.contains(&"load")) && !f.pin;
    for i in 0..frames {
        if keeps && MOMENTS.contains(&(i / 60)) && i % 60 == 0 { kept.push(Kept { at: i, moment: Moment::of(&p.g), next, cursor: p.cursor, words: Vec::new() }); }
        frame_of(&mut p, &hand, i, &mut next, &mut did);
        parts::words(&mut p.g, &mut w, &mut row);
        if let Some(m) = kept.last_mut() { if i < m.at + AFTER_MOMENT_SECS * 60 { m.words.push(row.clone()); } }
        if i % 60 == 59 { series.push(word_of(&row)); seconds.push(row.clone()); }
    }
    if kinds.contains(&"play") {
        if f.pin { println!("pin\tseries\t{}\t{}", name("play"), hex(&series)); return say(&name("play"), t.elapsed().as_secs_f64(), &[], ""); }
        let mut fail = Vec::new();
        let now = if f.breaks("pin") { series.iter().enumerate().map(|(i, w)| if i == 0 { w ^ 1 } else { *w }).collect() } else { series.clone() };
        match pins.series.get(&name("play")) {
            None => fail.push(String::from("no pin: `gate pin` records it")),
            Some(a) => if a != &now {
                let at = a.iter().zip(&now).position(|(x, y)| x != y).unwrap_or(a.len().min(series.len())) + 1;
                fail.push(format!("the run parts from its pin in second {} (pins.txt): `hold keep` on the build before and `hold check` on this one name the part", at));
            },
        }
        let never: Vec<&str> = hand.lines.iter().zip(&did).filter(|(_, d)| **d == 0).map(|(l, _)| l.as_str()).collect();
        say(&name("play"), t.elapsed().as_secs_f64(), &fail, &if never.is_empty() { String::new() } else { format!("never done here: {}", never.join("; ")) });
    }
    if kinds.contains(&"save") { save_row(&name("save"), &mut p, &hand, &mut kept, &names, f); }
    drop(p);
    if kinds.contains(&"load") { load_row(&name("load"), k, &hand, &mut kept, &names, f); }
    if kinds.contains(&"again") { again_row(&name("again"), k, &seconds, &names, f); }
    if kinds.contains(&"draw") { crate::draw::row(&name("draw"), k, &series, f); }
}

/// THE GAME PUT BACK IS THE GAME THAT WENT ON: each moment stands again, the even ones in the run's own slot and the odd ones in slot 1, and is played under the same hand from its frame on:
/// every part of every frame the same as the run's had. A host's hand is outside the game: where its next line is, and where its cursor lies, go back with the game.
/// BREAK: a moment whose block bytes are the run's end's.
fn save_row(name: &str, p: &mut Play, hand: &Hand, kept: &mut [Kept], names: &[String], f: Flags) {
    let t = Instant::now();
    let (mut w, mut row) = (game::bytes::W::new(), Vec::new());
    let mut did = vec![0; hand.lines.len()];
    let home = p.g.sim;
    let end = game_start::block::bytes(home).to_vec();
    let mut fail = Vec::new();
    'moments: for (m, k) in kept.iter_mut().enumerate() {
        if f.breaks("block") { k.moment.block.clone_from(&end); }
        p.g = k.moment.stands(if m % 2 == 0 { home } else { 1 });
        p.cursor = k.cursor;
        let mut next = k.next;
        for (i, was) in k.words.iter().enumerate() {
            frame_of(p, hand, k.at + i, &mut next, &mut did);
            parts::words(&mut p.g, &mut w, &mut row);
            if &row != was { fail.push(format!("the moment of {} s in slot {}, put back, parts at frame {} of it ({:.2} s on) in {}", k.at / 60, p.g.sim, i + 1, (i + 1) as f64 / 60.0, named(&parts::differ(names, was, &row)))); break 'moments; }
        }
    }
    say(name, t.elapsed().as_secs_f64(), &fail, "");
}

/// A LOAD IS THE GAME THAT WENT ON: the value a save keeps, the run's game gone and another mission stood and played in the process, stood as a page stands a kept game in a slot nobody was in, and played
/// under the hand from its kept place: every part of every frame the run's. (A fault this road has had: a calm guard's torch came on for three seconds.)
/// BREAK: `block` one byte of the block's die is another.
fn load_row(name: &str, k: usize, hand: &Hand, kept: &mut [Kept], names: &[String], f: Flags) {
    let t = Instant::now();
    let (mut w, mut row) = (game::bytes::W::new(), Vec::new());
    let mut did = vec![0; hand.lines.len()];
    let mut fail = Vec::new();
    let other = mission(RUNS[(k + 1) % RUNS.len()].0);
    let mut slot = sim::state::GAMES;
    'loads: for m in kept.iter_mut().filter(|m| LOADS.contains(&(m.at / 60))) {
        if f.breaks("block") { m.moment.block[sim::state::PARTS[1].off] ^= 1; }
        let mut q = stand(&other, SEED, 1);
        q.run(OTHER_SECS as f64);
        slot -= 1;
        // (as `face_stand` takes a kept game: its copy of the box world said rows of another slot)
        q.g = m.moment.laid(slot);
        game::scene_collision::unstand(&mut q.g);
        q.host = game_start::stand::host_of(&q.g).expect("a stood game was handed its host's rows");
        q.cursor = m.cursor;
        let mut next = m.next;
        for (i, was) in m.words.iter().enumerate() {
            frame_of(&mut q, hand, m.at + i, &mut next, &mut did);
            parts::words(&mut q.g, &mut w, &mut row);
            if &row != was { fail.push(format!("the moment of {} s, loaded into slot {}, parts at frame {} of it ({:.2} s on) in {}", m.at / 60, slot, i + 1, (i + 1) as f64 / 60.0, named(&parts::differ(names, was, &row)))); break 'loads; }
        }
    }
    say(name, t.elapsed().as_secs_f64(), &fail, "");
}

/// THE RUN STOOD AGAIN IN ONE PROCESS IS THE RUN: twice more, a run of another level between the two; the same folded word every second.
/// BREAK: the second run's die is drawn once before its first frame.
fn again_row(name: &str, k: usize, first: &[Vec<u32>], names: &[String], f: Flags) {
    let t = Instant::now();
    let (mut w, mut row) = (game::bytes::W::new(), Vec::new());
    let frames = RUN_SECS * 60;
    let mut fail = Vec::new();
    for (time, ordinal) in [(1, "second"), (2, "third")] {
        let (mut p, hand) = stand_run(k, SEED);
        if f.breaks("second") && time == 1 { game::math_dice::sim_random(&mut p.g); }
        let (mut next, mut did) = (0, vec![0; hand.lines.len()]);
        for i in 0..frames {
            frame_of(&mut p, &hand, i, &mut next, &mut did);
            if i % 60 != 59 { continue; }
            parts::words(&mut p.g, &mut w, &mut row);
            let at = i / 60;
            if word_of(&row) != word_of(&first[at]) { fail.push(format!("the {} run in one process parts in second {} in {}", ordinal, at + 1, named(&parts::differ(names, &first[at], &row)))); break; }
        }
        drop(p);
        if time == 1 && fail.is_empty() {
            // (the next run on the list: another level, its own hand)
            let other = (k + 1) % RUNS.len();
            let (mut q, hand) = stand_run(other, SEED);
            let (mut next, mut did) = (0, vec![0; hand.lines.len()]);
            for i in 0..BETWEEN_SECS * 60 { frame_of(&mut q, &hand, i, &mut next, &mut did); }
        }
        if !fail.is_empty() { break; }
    }
    say(name, t.elapsed().as_secs_f64(), &fail, "");
}

// ---- seats

const KEYS: [&str; 20] = ["KeyG", "KeyF", "KeyE", "KeyQ", "KeyB", "KeyC", "KeyR", "KeyV", "KeyT", "KeyH", "KeyN", "KeyX", "KeyZ", "Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Space", "Tab"];

/// the two players' hands for tick `i`: each man walks at the nearest enemy with the cursor on him, a burst every four seconds, a key of the game's every second and a half
fn drive(p: &mut Play, i: usize, held: &mut [Option<&'static str>; 2]) {
    let canvas = p.host.canvas;
    let seats = p.g.seats.clone();
    for (hi, &s) in seats.iter().enumerate() {
        let (input, cam) = (p.g.tab.cmd_seat[s].input, p.g.tab.cmd_seat[s].cam);
        for k in ["KeyW", "KeyA", "KeyS", "KeyD"] { p.g.tab.input[input].keys.delete(String::from(k)); }
        let man = p.g.ch[p.g.pl[game::cmd::seat_man(&p.g, s)].ch].pos;
        let flat = |a: [f64; 3]| game::m::hyp2(a[0] - man[0], a[2] - man[2]);
        let mut foes: Vec<[f64; 3]> = p.g.guards.iter().filter(|&&gd| p.g.gd[gd].state != GuardState::Dead && p.g.gd[gd].team != game::teams::Team::Operative).map(|&gd| p.g.ch[p.g.gd[gd].ch].pos).collect();
        foes.sort_by(|a, b| flat(*a).total_cmp(&flat(*b)));
        let a = i as f64 / 200.0 + hi as f64;
        let t = foes.first().copied().unwrap_or([man[0] + game::m::sin(a) * 8.0, 0.0, man[2] + game::m::cos(a) * 8.0]);
        let (dx, dz) = (t[0] - man[0], t[2] - man[2]);
        let d = game::m::hyp2(dx, dz).max(1e-9);
        let b = game::camera::follow_camera_planar_basis(&p.g, cam);
        let (along, across) = ((dx * b.fwd[0] + dz * b.fwd[2]) / d, (dx * b.right[0] + dz * b.right[2]) / d);
        const T: f64 = 0.3827;
        let keys = &mut p.g.tab.input[input].keys;
        if d > 3.0 {
            if along > T { keys.add(String::from("KeyW")); }
            if along < -T { keys.add(String::from("KeyS")); }
            if across > T { keys.add(String::from("KeyD")); }
            if across < -T { keys.add(String::from("KeyA")); }
        }
        let (sx, sy, front) = game::camera::follow_camera_project(&p.g, cam, [t[0], 1.2, t[2]], canvas.client_width, canvas.client_height, None, None);
        let hand = &mut p.g.tab.input[input];
        if front { hand.mouse_x = sx; hand.mouse_y = sy; }
        let ph = (i + hi * 37) % 240;
        if ph > 120 && ph < 200 { hand.buttons = ((hand.buttons as u32) | 1) as f64; if ph == 121 { hand.clicked = ((hand.clicked as u32) | 1) as f64; } } else { hand.buttons = ((hand.buttons as u32) & !1) as f64; }
        let beat = i + hi * 53;
        if beat % 90 == 0 { let k = KEYS[(beat / 90) % KEYS.len()]; hand.keys.add(String::from(k)); hand.pressed.add(String::from(k)); held[hi] = Some(k); }
        else if beat % 90 == 6 { if let Some(k) = held[hi].take() { hand.keys.delete(String::from(k)); } }
    }
}
/// what a host with two hands does round a step: each seat's camera after its man, the end of its input's frame
fn lens_and_keys(p: &mut Play) {
    for s in p.g.seats.clone() {
        let (input, cam) = (p.g.tab.cmd_seat[s].input, p.g.tab.cmd_seat[s].cam);
        let (follow, aim) = game::seats::with_seat(&mut p.g, s, &mut |g| (game::game::game_follow_pos(g, None), g.aim_point));
        game::camera::follow_camera_update(&mut p.g, cam, game_start::play::DT, follow, Some(aim));
        game::input::input_end_frame(&mut p.g, input);
    }
}

/// A FIGHT OF TWO IS ITS RECORDS: scripted hands make each tick's two commands, which cross as text and are stepped on as they came back. Fresh games that play the records with nobody sampling,
/// the machine showing seat 0 and then seat 1, are the same game; so is a moment of it put back with the records played from there. The series is pinned.
/// The staging is asked as facts: the two players' hands are not one hand, each man walks and the fight fires.
/// BREAKS: `pin` a word of the series made wrong · `fresh` a fresh game's replay hands each seat the other's records · `moment` the moment's does · `idle` nobody's hands do anything.
fn seats_row(name: &str, id: &str, pins: &Pins, f: Flags) {
    let t = Instant::now();
    let e = mission(id);
    let names = parts::names();
    let mut p = stand(&e, SEED, 2);
    if let Some(why) = &p.refused { panic!("a game of two is refused on {}: {}", id, why); }
    let (ticks, half) = (SEATS_SECS * 60, 30);
    let (mut records, mut rows, mut series): (Vec<[String; 2]>, Vec<Vec<u32>>, Vec<u32>) = (Vec::new(), Vec::new(), Vec::new());
    let (mut same_hands, mut walked, mut was) = (0, [0.0f64; 2], [[0.0f64; 3]; 2]);
    let mut held = [None, None];
    let mut moment = None;
    for i in 0..ticks {
        if i == SEATS_MOMENT.0 * 60 { moment = Some(Moment::of(&p.g)); }
        if !f.breaks("idle") { drive(&mut p, i, &mut held); }
        let cmds = p.net_sample();
        if (&cmds[0].keys, &cmds[0].pressed, cmds[0].buttons) == (&cmds[1].keys, &cmds[1].pressed, cmds[1].buttons) { same_hands += 1; }
        let text = [game::cmd::cmd_text(&cmds[0]), game::cmd::cmd_text(&cmds[1])];
        let back: Vec<PlayerCmd> = text.iter().map(|x| game::cmd::cmd_from_text(x)).collect();
        records.push(text);
        p.net_step(&back, 0);
        lens_and_keys(&mut p);
        for (s, seat) in p.g.seats.iter().enumerate() {
            let at = p.g.ch[p.g.pl[game::cmd::seat_man(&p.g, *seat)].ch].pos;
            if i > 0 { walked[s] += (at[0] - was[s][0]).hypot(at[2] - was[s][2]); }
            was[s] = at;
        }
        if i % half == half - 1 { let w = p.words(); if i % 60 == 59 { series.push(word_of(&w)); } rows.push(w); }
    }
    if f.pin { println!("pin\tseries\t{}\t{}", name, hex(&series)); return say(name, t.elapsed().as_secs_f64(), &[], ""); }
    let rounds: f64 = p.g.fa.iter().map(|(_, gun)| gun.shots_fired).sum();
    let mut fail = Vec::new();
    let now = if f.breaks("pin") { series.iter().enumerate().map(|(i, w)| if i == 0 { w ^ 1 } else { *w }).collect() } else { series.clone() };
    match pins.series.get(name) {
        None => fail.push(String::from("no pin: `gate pin` records it")),
        Some(a) => if a != &now { fail.push(format!("the fight parts from its pin in second {} (pins.txt)", a.iter().zip(&now).position(|(x, y)| x != y).unwrap_or(a.len().min(now.len())) + 1)); },
    }
    if (ticks - same_hands) * HANDS_DIFFER_ONE_IN < ticks { fail.push(format!("the two players' hands are the same on {} of {} ticks", same_hands, ticks)); }
    if walked[0] < MEN_WALK || walked[1] < MEN_WALK { fail.push(format!("the staging: the men walked {:.0} and {:.0} metres, and a fight of two has them walk {} at least", walked[0], walked[1], MEN_WALK)); }
    if rounds < FIGHT_FIRES { fail.push(format!("the staging: the fight fired {} rounds, and it has {} at least", rounds, FIGHT_FIRES)); }
    let mut seen = series.clone();
    seen.sort_unstable();
    seen.dedup();
    if seen.len() * 2 <= series.len() { fail.push(format!("only {} of {} seconds have different words: nothing is happening", seen.len(), series.len())); }
    // the records played, by a machine that shows a seat, from the tick `from` to `to`
    let replay = |p: &mut Play, shown: usize, from: usize, to: usize, swap: bool| -> Option<String> {
        for i in from..to {
            let c: Vec<PlayerCmd> = records[i].iter().map(|x| game::cmd::cmd_from_text(x)).collect();
            if swap { p.net_step(&[c[1].clone(), c[0].clone()], shown); } else { p.net_step(&c, shown); }
            if i % half == half - 1 {
                let w = p.words();
                let was = &rows[i / half];
                if &w != was { return Some(format!("parts {:.1} s in, in {}", (i + 1) as f64 / 60.0, named(&parts::differ(&names, was, &w)))); }
            }
        }
        None
    };
    if let Some(why) = moment.take().and_then(|m| { p.g = m.stands(p.g.sim); replay(&mut p, 0, SEATS_MOMENT.0 * 60, SEATS_MOMENT.1 * 60, f.breaks("moment")) }) { fail.push(format!("a moment of the fight put back {}", why)); }
    drop(p);
    for shown in [0, 1] {
        let mut q = stand(&e, SEED, 2);
        if let Some(why) = replay(&mut q, shown, 0, ticks, f.breaks("fresh")) { fail.push(format!("a fresh game played from the records, showing seat {}, {}", shown, why)); }
    }
    say(name, t.elapsed().as_secs_f64(), &fail, &format!("the men walked {:.0} and {:.0} metres, {} rounds", walked[0], walked[1], rounds));
}

// ---- par

/// THE CLOCK MOVES NOBODY: the mission under `push`, and the same level with its par block taken out of the file's tree before the stand, differ in nothing but what is the clock's own.
/// `level` is exempt from side against side only: each side's word stands as it stood at the stand. `par` must differ.
/// BREAKS: `moves` a die drawn once on the clocked side only · `noclock` the block is out on both sides.
fn par_row(name: &str, id: &str, f: Flags) {
    let t = Instant::now();
    let e = mission(id);
    let names = parts::names();
    let data = json::parse(&files::level_text(&file_of(&e))).unwrap_or_else(|why| panic!("{}: {}", file_of(&e), why));
    let mut bare = data.clone();
    bare.set_member("scenarios", data.get("scenarios").without("par"));
    let link = pick_search(&e, "");
    let play = |d: &json::Json, planted: bool| -> (Vec<Vec<u32>>, bool) {
        let frames = PAR_SECS * 60;
        let hand = hand_of("push", frames);
        let mut p = Play::mission_of(d, &link, &opts(SEED, 1));
        p.g.god_mode = hand.unhurt;
        if planted { game::math_dice::sim_random(&mut p.g); }
        let (mut next, mut did) = (0, vec![0; hand.lines.len()]);
        let mut rows = vec![p.words()];
        for i in 0..frames {
            frame_of(&mut p, &hand, i, &mut next, &mut did);
            if i % 30 == 29 { rows.push(p.words()); }
        }
        (rows, p.par_started().is_some())
    };
    let (clocked, ran) = play(if f.breaks("noclock") { &bare } else { &data }, f.breaks("moves"));
    let (plain, other) = play(&bare, false);
    let mut fail = Vec::new();
    if !ran || other { fail.push(format!("the staging: a clock ran with the block {} and without it {}", ran, other)); }
    for (k, (a, b)) in clocked.iter().zip(&plain).enumerate() {
        let moved: Vec<&str> = parts::differ(&names, a, b).into_iter().filter(|n| !CLOCK_ONLY.iter().any(|c| c.0 == *n)).collect();
        if !moved.is_empty() { fail.push(format!("{:.1} s in, the clock moved {}", k as f64 / 2.0, named(&moved))); break; }
    }
    let at = |part: &str| names.iter().position(|n| n == part).unwrap_or_else(|| panic!("no part is called '{}'", part));
    if [&clocked, &plain].iter().any(|side| side.iter().any(|r| r[at("level")] != side[0][at("level")])) { fail.push(String::from("the level's own record moved as the game played")); }
    if !clocked.iter().zip(&plain).any(|(a, b)| a[at("par")] != b[at("par")]) { fail.push(String::from("the staging: the clock's own part never differs")); }
    say(name, t.elapsed().as_secs_f64(), &fail, "");
}
