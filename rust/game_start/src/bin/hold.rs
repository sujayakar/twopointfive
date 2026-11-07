//! HOLD: is this build the same game as that one? Natively, with no page, no TypeScript and no wire (README.md, "Hold").
//!
//!   hold keep DIR [--hand NAME|FILE]… [--seed N]… [--secs N] [LEVEL[:clear]]…     play, and keep what the game was after every frame
//!   hold check DIR [--but PART,…]                                                 play the same again on this build, and compare
//!   hold draw DIR [as keep]                                                       play, DRAW every frame, and say what it cost and whether it changed anything
//!
//! `keep` stands each level by the Rust start (`:clear` starts a room clear on it), plays it for N seconds (90) under each HAND and each
//! seed (1), and writes for every frame the state's words (game::state_words: every part of the game that a host does not hand in)
//! and a hash of each part of rust/sim's block, folded to one number, and one byte of each part. `check` builds nothing: it reads DIR's list of runs and its copies of the hands, plays
//! them on the build it is, and says for each run `the same after every frame`, or the first FRAME that differs and the PARTS that differ
//! in it by their names. Exit 0 only if every run is the same. With no level named: the 24 of DEFAULT_LEVELS (14 as room clears); with no hand: every hand
//! of `hands/`. A run is a process of its own (a game is one a process), four at a time.
//!
//! A HAND is a small file of what is done to the game from outside, a line each (hands/*.hand; `#` starts a comment):
//!   unhurt                              Sam cannot be hurt (the page's own option), so a run does not end with him
//!   at T <what>                         once, T seconds in
//!   every N from T <what>               every N seconds from T on
//! and <what> is one of
//!   order ID WHERE all|red|blue         the menu's own call: an order's id ("move", "stack", "fallIn") on what stands at WHERE
//!   fire WHERE                          Sam's rifle comes up, the cursor goes to WHERE, one round
//!   frag WHERE | flash WHERE            a live can lands at WHERE
//! WHERE is `nearest` (the nearest man standing who is not of Sam's squad, as that frame finds him), `door` (the door nearest that
//! man: the way to him), `sam`, or `X,Z`.
//! A line that cannot be done (nobody is left, the menu refuses the order) is counted and the run goes on; a line that was NEVER done on
//! a floor is named in that run's line with the game's own reason, so a hand is not trusted there for what it only names.
//!
//! PARTS ARE COMPARED BY NAME. A kept file carries the names of the parts it was kept with, so a build that has gained a part, lost one
//! or re-ordered them is still compared, on the parts both have: the run's line then says which parts left the state and which are new.
//! `--but PART,…` leaves named parts out of the comparison on purpose, for a change that is MEANT to move one part and nothing else (a
//! record that changes shape, a table that is numbered anew): every other part must still be the same after every frame, and the line says
//! what was not compared. A part named that neither build has is an error, so a misspelt name does not pass for a comparison.
//!
//! `draw` keeps nothing in DIR but the hands it plays (a folder of its own): after every frame it makes every section of what the picture is told (rust/game_draw),
//! as a host does that draws at twice the tick's rate, and once a second it holds the game's own words to what they were before. A section that panics, or that the game is another for, fails the run; the last line of a run is
//! what each part cost, in microseconds a frame, its mean and its worst.
//!
//! WHAT IT PROVES: two builds that write the same bytes played the same game on these floors under these hands, part for part.
//! WHAT IT DOES NOT: anything the hands do not reach. Each hand's file says what it reaches and what it does not.
use game::game::GuardState;
use game_start::play::{dist, Button, Opts, Play};
use std::io::{Read, Write};

const DEFAULT_LEVELS: [&str; 22] = ["barracks:clear", "block:clear", "block-squads:clear", "compound:clear", "defuse:clear", "depot:clear", "floodlit:clear", "glasshouse:clear",
    "hall:clear", "killhouse:clear", "mousehole:clear", "records:clear", "pass:clear", "snatch:clear",
    "office", "consulate", "registry", "bazaar", "lights-out", "crossing", "police-post", "library-hall"];
const MAGIC: &[u8; 8] = b"hold 2\n\0";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Place { Nearest, Door, Sam, At(f64, f64) }
#[derive(Clone, Debug)]
enum What { Order(String, Place, game::orders::Who), Fire(Place), Can(game::game::ThrowKind, Place), KeyDown(&'static str), KeyUp(&'static str), Point(Place), Press, Release }
/// `at`: (the frame, what is done, which line of the file it is of); `lines`: the file's lines that do something, as written
struct Hand { unhurt: bool, at: Vec<(usize, What, usize)>, lines: Vec<String> }

fn hands_dir() -> std::path::PathBuf { std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hands") }

/// a hand's file read for a run of `frames` frames: every line as the frames it happens on, in the file's order within a frame
fn read_hand(text: &str, frames: usize) -> Hand {
    let mut h = Hand { unhurt: false, at: Vec::new(), lines: Vec::new() };
    let place = |w: &str| -> Place {
        match w { "nearest" => Place::Nearest, "door" => Place::Door, "sam" => Place::Sam, xz => { let mut p = xz.split(',').map(|v| v.parse::<f64>().unwrap_or_else(|_| panic!("a place is nearest, door, sam or X,Z: '{}'", xz))); Place::At(p.next().unwrap(), p.next().expect("X,Z")) } }
    };
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() { continue; }
        let w: Vec<&str> = line.split_whitespace().collect();
        let bad = || -> ! { panic!("line {} of the hand is not one this tool knows: '{}'", n + 1, line) };
        if w == ["unhurt"] { h.unhurt = true; continue; }
        let secs = |v: &str| -> usize { (v.parse::<f64>().unwrap_or_else(|_| bad()) * 60.0).round() as usize };
        let (first, step, rest) = match w[0] {
            "at" if w.len() > 2 => (secs(w[1]), 0, &w[2..]),
            "every" if w.len() > 4 && w[2] == "from" => (secs(w[3]), secs(w[1]).max(1), &w[4..]),
            _ => bad(),
        };
        let what = match rest {
            ["order", id, at, who] => What::Order(id.to_string(), place(at), match *who { "all" => game::orders::Who::All, "red" => game::orders::Who::Red, "blue" => game::orders::Who::Blue, _ => bad() }),
            ["fire", at] => What::Fire(place(at)),
            ["frag", at] => What::Can(game::game::ThrowKind::Frag, place(at)),
            ["flash", at] => What::Can(game::game::ThrowKind::Flash, place(at)),
            _ => bad(),
        };
        h.lines.push(rest.join(" "));
        let of = h.lines.len() - 1;
        let mut f = first;
        while f < frames {
            match &what {
                // (the rifle comes up over 0.8 s, the cursor rests 0.3 s on the place, the trigger is down two frames)
                What::Fire(p) => for (d, x) in [(0, What::KeyDown("Digit2")), (6, What::KeyUp("Digit2")), (48, What::Point(*p)), (66, What::Press), (68, What::Release)] { h.at.push((f + d, x, of)); },
                x => h.at.push((f, x.clone(), of)),
            }
            if step == 0 { break; }
            f += step;
        }
    }
    h.at.sort_by_key(|x| x.0);   // (a stable sort: lines of one frame stay in the file's order)
    h
}

/// the men standing who are not of Sam's squad, nearest to Sam first
fn others(p: &Play) -> Vec<game::ids::GuardId> {
    let sam = p.sam_at();
    let mut v: Vec<game::ids::GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative && p.g.gd[gd].state != GuardState::Dead && p.g.gd[gd].down_kind.is_none()).collect();
    v.sort_by(|x, y| dist(p.at(*x), sam).partial_cmp(&dist(p.at(*y), sam)).unwrap());
    v
}
fn place_of(p: &Play, at: Place) -> Option<[f64; 2]> {
    match at {
        Place::Sam => Some(p.sam_at()),
        Place::At(x, z) => Some([x, z]),
        Place::Nearest => others(p).first().map(|&m| p.at(m)),
        Place::Door => {
            let him = others(p).first().map(|&m| p.at(m))?;
            let mut doors: Vec<[f64; 2]> = p.g.doors.list.iter().map(|&d| { let c = p.g.dr[d].frame_centre; [c[0], c[2]] }).collect();
            doors.sort_by(|a, b| dist(*a, him).partial_cmp(&dist(*b, him)).unwrap());
            doors.first().copied()
        }
    }
}

/// one line of a hand done to the game; Err says why it could not be (the game's own refusal of an order, or nobody to do it to)
fn act(p: &mut Play, what: &What) -> Result<(), String> {
    let nobody = || String::from("nobody is left");
    match what {
        What::Order(id, at, who) => {
            let at = place_of(p, *at).ok_or_else(nobody)?;
            let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
            match game::orders::squad_orders_give(&mut p.g, id, Some(&t), Some(who.clone()), None) { Some(why) => Err(why), None => Ok(()) }
        }
        What::Can(kind, at) => { let at = place_of(p, *at).ok_or_else(nobody)?; game::game::game_drop_throwable(&mut p.g, *kind, [at[0], 0.0, at[1]]); Ok(()) }
        What::KeyDown(k) => { p.key_down(k); Ok(()) }
        What::KeyUp(k) => { p.key_up(k); Ok(()) }
        What::Point(at) => { let at = place_of(p, *at).ok_or_else(nobody)?; p.point_at(at); Ok(()) }
        What::Press => { p.press(Button::Left); Ok(()) }
        What::Release => { p.release(Button::Left); Ok(()) }
        What::Fire(_) => unreachable!("read_hand writes a shot out as its keys"),
    }
}

struct Run { level: String, clear: bool, hand: String, seed: u32, secs: f64 }
impl Run {
    fn tag(&self) -> String { format!("{}{}.{}.{}", self.level, if self.clear { "-clear" } else { "" }, self.hand, self.seed) }
    fn line(&self) -> String { format!("{} {} {} {} {}", self.level, if self.clear { "clear" } else { "plain" }, self.hand, self.seed, self.secs) }
    fn of(line: &str) -> Run { let w: Vec<&str> = line.split_whitespace().collect(); Run { level: w[0].into(), clear: w[1] == "clear", hand: w[2].into(), seed: w[3].parse().unwrap(), secs: w[4].parse().unwrap() } }
}

/// A kept file's head: the magic, how many parts, then their names (a length, and the names a line each). Returns the names and where the frames begin.
fn kept_head(kept: &[u8], file: &str) -> Result<(Vec<String>, usize), String> {
    if kept.len() >= 8 && &kept[..6] == b"hold 1" { return Err(format!("{} was kept by an older build of this tool, which wrote no part names: keep again", file)); }
    if kept.len() < 16 || &kept[..8] != MAGIC { return Err(format!("{} is not a file of this tool", file)); }
    let n = u32::from_le_bytes(kept[8..12].try_into().unwrap()) as usize;
    let l = u32::from_le_bytes(kept[12..16].try_into().unwrap()) as usize;
    if kept.len() < 16 + l { return Err(format!("{} is cut short in its head", file)); }
    let names: Vec<String> = String::from_utf8_lossy(&kept[16..16 + l]).split('\n').map(String::from).collect();
    if names.len() != n { return Err(format!("{} says {} parts and names {}", file, n, names.len())); }
    Ok((names, 16 + l))
}

/// the parts of a frame's drawing that are timed, by the sections they are asked with
const DRAWN: [(&str, u32); 6] = [("fog", game_draw::want::fog), ("view", game_draw::want::view), ("sky", game_draw::want::sky), ("boxes", game_draw::want::geo), ("lights", game_draw::want::lights), ("men", game_draw::want::men)];

/// ONE RUN DRAWN, in this process
fn drawn(dir: &str, r: &Run) -> Result<String, String> {
    let frames = (r.secs * 60.0).round() as usize;
    let hand = read_hand(&std::fs::read_to_string(format!("{}/{}.hand", dir, r.hand)).map_err(|e| format!("{}/{}.hand: {}", dir, r.hand, e))?, frames);
    let file_of = if r.level.ends_with(".json") || std::path::Path::new(&r.level).exists() { r.level.clone() } else { format!("{}.json", r.level) };
    let mut p = Play::level(&file_of, r.clear, &Opts { seed: r.seed, ..Default::default() });
    p.g.god_mode = hand.unhurt;
    p.g.fog_of_war.on = r.clear;
    let mut d: Vec<game_draw::Drawn> = DRAWN.iter().map(|_| Default::default()).collect();
    let (mut cost, mut next) = (vec![(0.0f64, 0.0f64); DRAWN.len() + 1], 0usize);
    let (mut boxes, mut men, mut lights) = (0, 0, 0);
    for k in 0..frames {
        while next < hand.at.len() && hand.at[next].0 <= k { let _ = act(&mut p, &hand.at[next].1); next += 1; }
        p.frame();
        let before = if k % 60 == 0 { Some(p.words()) } else { None };
        let mut between = 0.0;
        for (i, (_, want)) in DRAWN.iter().enumerate() {
            let t = std::time::Instant::now();
            d[i].take(&p.g, *want);
            let us = t.elapsed().as_secs_f64() * 1e6;
            cost[i].0 += us;
            cost[i].1 = cost[i].1.max(us);
            let focus = p.g.aim_point;
            let t = std::time::Instant::now();
            d[i].frame(&p.g, 0.5, focus);
            d[i].frame(&p.g, 1.0, focus);
            between += t.elapsed().as_secs_f64() * 1e6 / 2.0;
            if d[i].table().is_empty() { return Err(String::from("no section lies anywhere")); }
        }
        let n = DRAWN.len();
        cost[n].0 += between;
        cost[n].1 = cost[n].1.max(between);
        boxes = boxes.max(d[3].boxes.geo.len());
        lights = lights.max(d[4].lights.rows.len());
        men = men.max(d[5].men.men.len());
        let h = d[3].boxes.head[0];
        if h.count as usize != d[3].boxes.geo.len() || h.visible as usize + h.visible_cut as usize != d[3].boxes.visible.len() { return Err(format!("frame {}: the boxes' head does not say what their lists hold", k)); }
        if let Some(b) = before { if b != p.words() { return Err(format!("frame {}: drawing the game changed it", k)); } }
    }
    let said: Vec<String> = DRAWN.iter().map(|x| x.0).chain(["between ticks"]).zip(&cost).map(|(n, c)| format!("{n} {:.0} ({:.0})", c.0 / frames as f64, c.1)).collect();
    Ok(format!("{} frames · at most {} boxes, {} lights, {} men · microseconds a frame, mean (worst): {}", frames, boxes, lights, men, said.join(", ")))
}

/// ONE RUN, in this process. `check` false: write DIR's file. True: compare with it, frame by frame and part by part by name (the parts of
/// `but` left out), and stop at the first frame that differs.
fn one(dir: &str, r: &Run, check: bool, but: &[String]) -> Result<String, String> {
    let frames = (r.secs * 60.0).round() as usize;
    let hand = read_hand(&std::fs::read_to_string(format!("{}/{}.hand", dir, r.hand)).map_err(|e| format!("{}/{}.hand: {}", dir, r.hand, e))?, frames);
    let file = format!("{}/{}.h", dir, r.tag());
    // the parts: the game's own (game::state_words), then rust/sim's block (`sim.dice`, `sim.poses`, …)
    let names: Vec<String> = game::state_words::STATE_NAMES.iter().map(|s| s.to_string()).chain(game_start::block::PART_NAMES.iter().map(|s| format!("sim.{}", s))).collect();
    let n = names.len();
    let mut kept: Vec<u8> = Vec::new();
    // (then: the kept file's part names, where its frames begin and how wide one is; pairs: the parts compared, as (its place now, its place then))
    let (mut then, mut at0, mut pairs): (Vec<String>, usize, Vec<(usize, usize)>) = (Vec::new(), 0, Vec::new());
    let mut note = String::new();
    if check {
        std::fs::File::open(&file).and_then(|mut f| f.read_to_end(&mut kept)).map_err(|e| format!("{}: {}", file, e))?;
        (then, at0) = kept_head(&kept, &file)?;
        if let Some(x) = but.iter().find(|b| !names.contains(b) && !then.contains(b)) { return Err(format!("--but names '{}', which is a part of neither build", x)); }
        for (i, nm) in names.iter().enumerate() { if but.contains(nm) { continue; } if let Some(j) = then.iter().position(|t| t == nm) { pairs.push((i, j)); } }
        let gone: Vec<&str> = then.iter().filter(|t| !names.contains(t)).map(|t| t.as_str()).collect();
        let new: Vec<&str> = names.iter().filter(|t| !then.contains(t)).map(|t| t.as_str()).collect();
        if !gone.is_empty() || !new.is_empty() || !but.is_empty() {
            note = format!(" · COMPARED {} PARTS BY NAME", pairs.len());
            if !but.is_empty() { note += &format!(" · not compared, as asked: {}", but.join(", ")); }
            if !gone.is_empty() { note += &format!(" · left the state: {}", gone.join(", ")); }
            if !new.is_empty() { note += &format!(" · new: {}", new.join(", ")); }
        }
    }
    let whole = check && note.is_empty() && then == names;
    let wide = 8 + then.len();
    let mut out = if check { None } else {
        let mut f = std::io::BufWriter::new(std::fs::File::create(&file).map_err(|e| format!("{}: {}", file, e))?);
        let blob = names.join("\n");
        f.write_all(MAGIC).unwrap(); f.write_all(&(n as u32).to_le_bytes()).unwrap(); f.write_all(&(blob.len() as u32).to_le_bytes()).unwrap(); f.write_all(blob.as_bytes()).unwrap();
        Some(f)
    };
    let file_of = if r.level.ends_with(".json") || std::path::Path::new(&r.level).exists() { r.level.clone() } else { format!("{}.json", r.level) };
    let mut p = Play::level(&file_of, r.clear, &Opts { seed: r.seed, ..Default::default() });
    p.g.god_mode = hand.unhurt;
    let (mut w, mut words, mut row) = (game::io::W::new(), Vec::new(), vec![0u8; 8 + n]);
    let (mut next, mut done, mut undone) = (0usize, 0usize, 0usize);
    let mut did = vec![0usize; hand.lines.len()];
    let mut why_not = vec![String::new(); hand.lines.len()];
    for k in 0..frames {
        while next < hand.at.len() && hand.at[next].0 <= k { match act(&mut p, &hand.at[next].1) { Ok(()) => { done += 1; did[hand.at[next].2] += 1; } Err(why) => { undone += 1; why_not[hand.at[next].2] = why; } } next += 1; }
        p.frame();
        w.b.clear();
        // (the game's own parts, then rust/sim's block a part at a time)
        game::state_words::state_words(&mut p.g, &mut w, &mut words);
        words.extend(game_start::block::part_hashes(0));
        let mut all = 0xcbf2_9ce4_8422_2325u64;
        for (i, x) in words.iter().enumerate() { all = (all ^ *x as u64).wrapping_mul(0x0000_0100_0000_01b3); row[8 + i] = (*x ^ (*x >> 8) ^ (*x >> 16) ^ (*x >> 24)) as u8; }
        row[..8].copy_from_slice(&all.to_le_bytes());
        match &mut out {
            Some(f) => f.write_all(&row).unwrap(),
            None => {
                let at = at0 + k * wide;
                if kept.len() < at + wide { return Err(format!("{} holds {} frames and this run is {}", file, (kept.len() - at0) / wide, frames)); }
                let was = &kept[at..at + wide];
                // (the same parts in the same order and none left out: the frame's one number is compared too. Else each part's byte, by name)
                let same = if whole { was == &row[..] } else { pairs.iter().all(|&(i, j)| was[8 + j] == row[8 + i]) };
                if !same {
                    let parts: Vec<&str> = pairs.iter().filter(|&&(i, j)| was[8 + j] != row[8 + i]).map(|&(i, _)| names[i].as_str()).collect();
                    return Err(format!("FIRST DIFFERS AT FRAME {} ({:.2} s) in {}{}", k + 1, (k + 1) as f64 / 60.0, if parts.is_empty() { String::from("a part whose kept byte happens to agree") } else { parts.join(", ") }, note));
                }
            }
        }
    }
    if let Some(f) = &mut out { f.flush().unwrap(); }
    let up = others(&p).len();
    // (a line of the hand that was never once done on this floor reached nothing here: said, so that a hand is not trusted for what it only names)
    let never: Vec<String> = hand.lines.iter().enumerate().filter(|(i, _)| did[*i] == 0).map(|(i, l)| format!("{} ({})", l, why_not[i].trim_end_matches('.'))).collect();
    let never = if never.is_empty() { String::new() } else { format!(" · NEVER DONE HERE: {}", never.join("; ")) };
    Ok(format!("{} frames · {} of the hand's lines done, {} could not be{never} · {} others standing of {} men · {} lines said · {}{note}", frames, done, undone, up, p.g.guards.len(), p.said.len(), game::control::control_tally(&p.g)))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let usage = || -> ! { eprintln!("hold keep DIR [--hand NAME|FILE]… [--seed N]… [--secs N] [LEVEL[:clear]]…\nhold check DIR [--but PART,…]\nhold draw DIR [as keep]"); std::process::exit(64) };
    let (Some(verb), Some(dir)) = (a.get(1).map(|s| s.as_str()), a.get(2)) else { usage() };
    // (a child of this tool: one run, in a process of its own)
    if verb == "one" {
        let (r, check) = (Run::of(&a[4]), a[3] == "check");
        let but: Vec<String> = a.get(5).map(|s| s.split(',').filter(|x| !x.is_empty()).map(String::from).collect()).unwrap_or_default();
        let dir = dir.clone();
        // (the start's frames are deep: a thread with room)
        let draw = a[3] == "draw";
        let h = std::thread::Builder::new().stack_size(1 << 30).spawn(move || if draw { drawn(&dir, &r) } else { one(&dir, &r, check, &but) }).unwrap();
        match h.join() { Ok(Ok(line)) => { println!("{}", line); return; } Ok(Err(why)) => { println!("{}", why); std::process::exit(1); } Err(_) => std::process::exit(2) }
    }
    // (`check`'s own words: the parts left out of the comparison on purpose)
    let mut but: Vec<String> = Vec::new();
    if verb == "check" {
        let mut i = 3;
        while i < a.len() {
            if a[i] != "--but" { usage(); }
            but.extend(a.get(i + 1).unwrap_or_else(|| usage()).split(',').filter(|x| !x.is_empty()).map(String::from));
            i += 2;
        }
    }
    let runs: Vec<Run> = match verb {
        "keep" | "draw" => {
            let (mut hands, mut seeds, mut levels, mut secs) = (Vec::new(), Vec::new(), Vec::new(), 90.0);
            let mut i = 3;
            while i < a.len() {
                match a[i].as_str() {
                    "--hand" => { hands.push(a.get(i + 1).cloned().unwrap_or_else(|| usage())); i += 1; }
                    "--seed" => { seeds.push(a.get(i + 1).and_then(|s| s.parse::<u32>().ok()).unwrap_or_else(|| usage())); i += 1; }
                    "--secs" => { secs = a.get(i + 1).and_then(|s| s.parse::<f64>().ok()).unwrap_or_else(|| usage()); i += 1; }
                    l if !l.starts_with("--") => levels.push(l.to_string()),
                    _ => usage(),
                }
                i += 1;
            }
            if hands.is_empty() {
                let mut all: Vec<String> = std::fs::read_dir(hands_dir()).expect("the hands' folder").filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).filter_map(|f| f.strip_suffix(".hand").map(String::from)).collect();
                all.sort();
                hands = all;
            }
            if seeds.is_empty() { seeds.push(1); }
            if levels.is_empty() { levels = DEFAULT_LEVELS.iter().map(|s| s.to_string()).collect(); }
            std::fs::create_dir_all(dir).unwrap();
            if verb == "draw" && std::path::Path::new(&format!("{}/runs.txt", dir)).exists() { eprintln!("{} holds kept runs: draw into a folder of its own", dir); std::process::exit(64); }
            // a hand is copied into DIR under its name, so that `check` plays the hand that was kept whatever the file says by then
            let mut names = Vec::new();
            for h in &hands {
                let path = if std::path::Path::new(h).exists() { std::path::PathBuf::from(h) } else { hands_dir().join(format!("{}.hand", h)) };
                let name = path.file_stem().unwrap().to_string_lossy().into_owned();
                let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {}", path.display(), e));
                read_hand(&text, 1);   // (a line this tool does not know stops here, before anything is played)
                std::fs::write(format!("{}/{}.hand", dir, name), text).unwrap();
                names.push(name);
            }
            let mut runs = Vec::new();
            for l in &levels { for h in &names { for &s in &seeds {
                let (level, clear) = match l.strip_suffix(":clear") { Some(n) => (n.to_string(), true), None => (l.clone(), false) };
                runs.push(Run { level, clear, hand: h.clone(), seed: s, secs });
            } } }
            if verb == "keep" { std::fs::write(format!("{}/runs.txt", dir), runs.iter().map(|r| r.line() + "\n").collect::<String>()).unwrap(); }
            runs
        }
        "check" => std::fs::read_to_string(format!("{}/runs.txt", dir)).unwrap_or_else(|e| panic!("{}/runs.txt: {} (keep first)", dir, e)).lines().filter(|l| !l.trim().is_empty()).map(Run::of).collect(),
        _ => usage(),
    };
    // every run a child, four at a time, reported in the list's order
    let me = std::env::current_exe().unwrap();
    let jobs = std::thread::available_parallelism().map_or(4, |n| n.get()).min(4);
    let (mut bad, mut i) = (0, 0);
    let mut good: Vec<usize> = Vec::new();
    while i < runs.len() {
        let batch: Vec<(usize, std::process::Child)> = (i..runs.len().min(i + jobs)).map(|k| (k, std::process::Command::new(&me).arg("one").arg(dir).arg(verb).arg(runs[k].line()).arg(but.join(",")).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap())).collect();
        for (k, c) in batch {
            let o = c.wait_with_output().unwrap();
            let said = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let line = match o.status.code() {
                Some(0) => { good.push(k); if verb == "check" { format!("the same after every frame · {}", said) } else { said } }
                Some(1) => { bad += 1; said }
                _ => { bad += 1; format!("STOPPED: {}", String::from_utf8_lossy(&o.stderr).lines().filter(|l| !l.trim().is_empty()).take(3).collect::<Vec<_>>().join(" | ")) }
            };
            println!("{:<34} {}", runs[k].tag(), line);
        }
        i += jobs;
    }
    // (a run that could not be played is not in the list `check` plays: a level that does not stand as asked is said once, here)
    if verb == "keep" { std::fs::write(format!("{}/runs.txt", dir), good.iter().map(|&k| runs[k].line() + "\n").collect::<String>()).unwrap(); }
    let what = if verb == "draw" { String::from("drawn, and none of them another game for it") } else if verb != "check" { String::from("kept") } else if but.is_empty() { String::from("the same after every frame") } else { format!("the same after every frame in every part but {}", but.join(", ")) };
    println!("hold {}: {} of {} runs {}", verb, runs.len() - bad, runs.len(), what);
    std::process::exit(if bad > 0 { 1 } else { 0 });
}
