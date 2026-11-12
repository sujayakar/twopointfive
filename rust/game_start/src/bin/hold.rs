//! HOLD: is this build the same game as that one? Natively, with no page and no wire (README.md, "Hold").
//!
//!   hold keep DIR [--hand NAME|FILE]… [--seed N]… [--secs N] [LEVEL[:clear]|MISSION]…     play, and keep what the game was after every frame
//!   hold check DIR [--but PART,…]                                                 play the same again on this build, and compare
//!   hold draw DIR [as keep]                                                       play, DRAW every frame, and say what it cost and whether it changed anything
//!   hold steps DIR [--write] [as keep]                                            play, and say what each step of the tick cost and which parts of the game it wrote (`--write`: into steps.txt)
//!
//! `keep` stands each level by the Rust start (`:clear` starts a room clear on it; a mission's id of the game's list stands that mission), plays it for N seconds (90) under each HAND and each
//! seed (1), and writes for every frame the state's words (game::state_words: every part of the game that a host does not hand in)
//! folded to one number, and one byte of each part. `check` builds nothing: it reads DIR's list of runs and its copies of the hands, plays
//! them on the build it is, and says for each run `the same after every frame`, or the first FRAME that differs and the PARTS that differ
//! in it by their names. Exit 0 only if every run is the same. With no level named: the 21 of DEFAULT_LEVELS (13 as room clears); with no hand: every hand
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
//! WHERE is `nearest` (the nearest man standing who is not of Sam's squad, as that frame finds him), `ahead` (four metres on toward him from the squad's first man), `door` (the door nearest that
//! man: the way to him; with nobody left, the door nearest Sam), `opendoor` (the same of the doors that stand open), `sam`, or `X,Z`.
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
use game_start::hand::{act, hands_dir, others, read_hand};
use game_start::parts;
use game_start::play::{Opts, Play};
use game_start::steps::{self, Steps};
use std::io::{Read, Write};

/// What is played with no name given: thirteen room clears, and the missions of the list whose men are not a room clear's (the rescues and the night breach, and four quiet nights)
const DEFAULT_LEVELS: [&str; 21] = ["barracks:clear", "block:clear", "compound:clear", "defuse:clear", "depot:clear", "floodlit:clear", "glasshouse:clear",
    "hall:clear", "killhouse:clear", "mousehole:clear", "records:clear", "pass:clear", "snatch:clear",
    "hostage-rescue", "rescue-consulate", "rescue-registry", "night-breach", "bazaar-quiet", "lights-out-two", "quiet-crossing-two", "police-post-quiet"];
const MAGIC: &[u8; 8] = b"hold 2\n\0";

struct Run { level: String, clear: bool, hand: String, seed: u32, secs: f64 }
impl Run {
    fn tag(&self) -> String { format!("{}{}.{}.{}", self.level, if self.clear { "-clear" } else { "" }, self.hand, self.seed) }
    fn line(&self) -> String { format!("{} {} {} {} {}", self.level, if self.clear { "clear" } else { "plain" }, self.hand, self.seed, self.secs) }
    /// stood: a mission of the game's list by its id, on its own link as a click on its row stands it; or a level by its name, its room clear with it if it says so
    fn stand(&self) -> Play { self.stand_as(false) }
    /// …with the dressing rows up, as a page stands a game
    fn stand_as(&self, dress: bool) -> Play {
        let o = Opts { seed: self.seed, dress, ..Default::default() };
        if let Some(e) = game::scene_scenario_list::all_missions().iter().find(|e| e.id == self.level) {
            return Play::mission(e.level.as_deref().unwrap_or("office.json"), &game::scene_scenario_list::pick_search(e, ""), &o);
        }
        let file = if self.level.ends_with(".json") || std::path::Path::new(&self.level).exists() { self.level.clone() } else { format!("{}.json", self.level) };
        Play::level(&file, self.clear, &o)
    }
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
const DRAWN: [(&str, u32); 13] = [("fog", game_draw::want::fog), ("view", game_draw::want::view), ("sky", game_draw::want::sky), ("boxes", game_draw::want::boxes), ("lights", game_draw::want::lights), ("men", game_draw::want::men), ("rims", game_draw::want::rims), ("events", game_draw::want::events), ("fires", game_draw::want::fires), ("boxes and marks", game_draw::want::boxes | game_draw::want::decals), ("air", game_draw::want::air), ("the level's look", game_draw::want::level_look), ("snow", game_draw::want::snow)];

/// ONE RUN DRAWN, in this process
fn drawn(dir: &str, r: &Run) -> Result<String, String> {
    let frames = (r.secs * 60.0).round() as usize;
    let hand = read_hand(&std::fs::read_to_string(format!("{}/{}.hand", dir, r.hand)).map_err(|e| format!("{}/{}.hand: {}", dir, r.hand, e))?, frames);
    let mut p = r.stand();
    p.g.god_mode = hand.unhurt;
    p.g.fog_of_war.on = r.clear;
    let mut d: Vec<game_draw::Drawn> = DRAWN.iter().map(|_| Default::default()).collect();
    let (mut cost, mut next) = (vec![(0.0f64, 0.0f64); DRAWN.len() + 1], 0usize);
    let (mut boxes, mut men, mut lights) = (0, 0, 0);
    for k in 0..frames {
        while next < hand.at.len() && hand.at[next].0 <= k { let _ = act(&mut p, &hand.at[next].1); next += 1; }
        let mut between = 0.0;
        let mut wrong = None;
        p.frame_shown(|p| {
        let before = if k % 60 == 0 { Some(p.words()) } else { None };
        for (i, (_, want)) in DRAWN.iter().enumerate() {
            let t = std::time::Instant::now();
            d[i].take(&p.g, *want);
            let us = t.elapsed().as_secs_f64() * 1e6;
            cost[i].0 += us;
            cost[i].1 = cost[i].1.max(us);
            let focus = p.g.sets[p.g.seat].aim_point;
            let t = std::time::Instant::now();
            d[i].frame(&p.g, 0.5, focus);
            d[i].frame(&p.g, 1.0, focus);
            between += t.elapsed().as_secs_f64() * 1e6 / 2.0;
            if d[i].table().is_empty() { wrong = Some(String::from("no section lies anywhere")); }
        }
        let n = DRAWN.len();
        cost[n].0 += between;
        cost[n].1 = cost[n].1.max(between);
        boxes = boxes.max(d[3].boxes.geo.len());
        lights = lights.max(d[4].lights.rows.len());
        men = men.max(d[5].men.men.len());
        let h = d[3].boxes.head[0];
        if h.count as usize != d[3].boxes.geo.len() || h.visible as usize + h.visible_cut as usize != d[3].boxes.visible.len() { wrong = Some(format!("frame {}: the boxes' head does not say what their lists hold", k)); }
        if let Some(b) = before { if b != p.words() { wrong = Some(format!("frame {}: drawing the game changed it", k)); } }
        });
        if let Some(why) = wrong { return Err(why); }
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
    let names = parts::names();
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
    let mut p = r.stand();
    p.g.god_mode = hand.unhurt;
    let (mut w, mut words, mut row) = (game::bytes::W::new(), Vec::new(), vec![0u8; 8 + n]);
    let (mut next, mut done, mut undone) = (0usize, 0usize, 0usize);
    let mut did = vec![0usize; hand.lines.len()];
    let mut why_not = vec![String::new(); hand.lines.len()];
    for k in 0..frames {
        while next < hand.at.len() && hand.at[next].0 <= k { match act(&mut p, &hand.at[next].1) { Ok(()) => { done += 1; did[hand.at[next].2] += 1; } Err(why) => { undone += 1; why_not[hand.at[next].2] = why; } } next += 1; }
        p.frame();
        parts::words(&mut p.g, &mut w, &mut words);
        for (i, x) in words.iter().enumerate() { row[8 + i] = (*x ^ (*x >> 8) ^ (*x >> 16) ^ (*x >> 24)) as u8; }
        row[..8].copy_from_slice(&parts::fold(&words).to_le_bytes());
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

/// ONE RUN BY ITS STEPS, in this process: for each step of the tick its microseconds a frame, its worst, and the parts it was seen to write (asked every eighth frame: it is slow to ask)
fn stepped(dir: &str, r: &Run) -> Result<String, String> {
    let frames = (r.secs * 60.0).round() as usize;
    let hand = read_hand(&std::fs::read_to_string(format!("{}/{}.hand", dir, r.hand)).map_err(|e| format!("{}/{}.hand: {}", dir, r.hand, e))?, frames);
    let mut p = r.stand_as(true);
    p.g.god_mode = hand.unhurt;
    let mut steps = Steps::new();
    let mut next = 0;
    for k in 0..frames {
        while next < hand.at.len() && hand.at[next].0 <= k { let _ = act(&mut p, &hand.at[next].1); next += 1; }
        steps.frame(&mut p, k % 8 == 0);
    }
    Ok(steps.steps.iter().map(|s| format!("step\t{}\t{:.2}\t{:.0}\t{}.", s.name, steps.mean(s), s.worst, steps.wrote(s).map(|p| format!("{p} ")).collect::<String>())).collect::<Vec<_>>().join("\n"))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let usage = || -> ! { eprintln!("hold keep DIR [--hand NAME|FILE]… [--seed N]… [--secs N] [LEVEL[:clear]|MISSION]…\nhold check DIR [--but PART,…]\nhold draw DIR [as keep]\nhold steps DIR [--write] [as keep]"); std::process::exit(64) };
    let (Some(verb), Some(dir)) = (a.get(1).map(|s| s.as_str()), a.get(2)) else { usage() };
    // (a child of this tool: one run, in a process of its own)
    if verb == "one" {
        let (r, check) = (Run::of(&a[4]), a[3] == "check");
        let but: Vec<String> = a.get(5).map(|s| s.split(',').filter(|x| !x.is_empty()).map(String::from).collect()).unwrap_or_default();
        let dir = dir.clone();
        // (the start's frames are deep: a thread with room)
        let verb = a[3].clone();
        let h = std::thread::Builder::new().stack_size(1 << 30).spawn(move || match verb.as_str() { "draw" => drawn(&dir, &r), "steps" => stepped(&dir, &r), _ => one(&dir, &r, check, &but) }).unwrap();
        match h.join() { Ok(Ok(line)) => { println!("{}", line); return; } Ok(Err(why)) => { println!("{}", why); std::process::exit(1); } Err(_) => std::process::exit(2) }
    }
    // (`check`'s own words: the parts left out of the comparison on purpose)
    let mut but: Vec<String> = Vec::new();
    let mut write = false;
    if verb == "check" {
        let mut i = 3;
        while i < a.len() {
            if a[i] != "--but" { usage(); }
            but.extend(a.get(i + 1).unwrap_or_else(|| usage()).split(',').filter(|x| !x.is_empty()).map(String::from));
            i += 2;
        }
    }
    let runs: Vec<Run> = match verb {
        "keep" | "draw" | "steps" => {
            let (mut hands, mut seeds, mut levels, mut secs) = (Vec::new(), Vec::new(), Vec::new(), 90.0);
            let mut i = 3;
            while i < a.len() {
                match a[i].as_str() {
                    "--hand" => { hands.push(a.get(i + 1).cloned().unwrap_or_else(|| usage())); i += 1; }
                    "--seed" => { seeds.push(a.get(i + 1).and_then(|s| s.parse::<u32>().ok()).unwrap_or_else(|| usage())); i += 1; }
                    "--secs" => { secs = a.get(i + 1).and_then(|s| s.parse::<f64>().ok()).unwrap_or_else(|| usage()); i += 1; }
                    "--write" if verb == "steps" => write = true,
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
            if verb != "keep" && std::path::Path::new(&format!("{}/runs.txt", dir)).exists() { eprintln!("{} holds kept runs: draw into a folder of its own", dir); std::process::exit(64); }
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
    // every run a child, as many at once as there are cores and the next as one ends, reported in the list's order
    let me = std::env::current_exe().unwrap();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut bad = 0;
    let mut good: Vec<usize> = Vec::new();
    let mut table: Vec<(String, f64, f64, Vec<String>)> = Vec::new();
    std::thread::scope(|all| {
        for _ in 0..std::thread::available_parallelism().map_or(4, |n| n.get()) {
            let (tx, next, me, runs, but) = (tx.clone(), &next, &me, &runs, &but);
            all.spawn(move || loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if k >= runs.len() { return }
                let _ = tx.send((k, std::process::Command::new(me).arg("one").arg(dir).arg(verb).arg(runs[k].line()).arg(but.join(",")).output().unwrap()));
            });
        }
        let mut came: std::collections::BTreeMap<usize, std::process::Output> = std::collections::BTreeMap::new();
        for k in 0..runs.len() {
            while !came.contains_key(&k) { let (i, o) = rx.recv().expect("the children's threads"); came.insert(i, o); }
            let o = came.remove(&k).unwrap();
            let said = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let line = match o.status.code() {
                Some(0) => { good.push(k); if verb == "check" { format!("the same after every frame · {}", said) } else { said } }
                Some(1) => { bad += 1; said }
                _ => { bad += 1; format!("STOPPED: {}", String::from_utf8_lossy(&o.stderr).lines().filter(|l| !l.trim().is_empty()).take(3).collect::<Vec<_>>().join(" | ")) }
            };
            let (rows, line) = line.split_once("step\t").map_or((String::new(), line.clone()), |_| (line.clone(), String::from("stepped")));
            for row in rows.lines() {
                let w: Vec<&str> = row.split('\t').collect();
                let i = match table.iter().position(|s| s.0 == w[1]) { Some(i) => i, None => { table.push((w[1].into(), 0.0, 0.0, Vec::new())); table.len() - 1 } };
                let s = &mut table[i];
                s.1 += w[2].parse::<f64>().unwrap();
                s.2 = s.2.max(w[3].parse().unwrap());
                for part in w[4].split_whitespace().filter(|x| *x != ".") { if !s.3.iter().any(|x| x == part) { s.3.push(part.into()); } }
            }
            println!("{:<34} {}", runs[k].tag(), line);
        }
    });
    // (a run that could not be played is not in the list `check` plays: a level that does not stand as asked is said once, here)
    if verb == "keep" { std::fs::write(format!("{}/runs.txt", dir), good.iter().map(|&k| runs[k].line() + "\n").collect::<String>()).unwrap(); }
    if write { steps::write_file(&table.iter().map(|(n, _, _, p)| (n.clone(), { let mut p = p.clone(); p.sort(); p })).collect::<Vec<_>>()); }
    for (name, us, worst, parts) in &mut table {
        parts.sort();
        println!("{:<20} {:>8.1} us a frame, {:>6.0} at worst · {}", name, *us / good.len().max(1) as f64, worst, parts.join(" "));
    }
    let what = if verb == "steps" { String::from("stepped") } else if verb == "draw" { String::from("drawn, and none of them another game for it") } else if verb != "check" { String::from("kept") } else if but.is_empty() { String::from("the same after every frame") } else { format!("the same after every frame in every part but {}", but.join(", ")) };
    println!("hold {}: {} of {} runs {}", verb, runs.len() - bad, runs.len(), what);
    std::process::exit(if bad > 0 { 1 } else { 0 });
}
