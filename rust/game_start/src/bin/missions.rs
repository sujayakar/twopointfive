//! MISSIONS: what a change of play does to the room clears, read the same way before and after. Natively, with no page and no TypeScript (README.md, "Missions").
//!
//!   missions [--seeds 1-8] [--secs 75] [--std | --hard] [LEVEL…]      play, and print a row a mission and difficulty, tab between columns (keep it in a file)
//!   missions diff BEFORE AFTER                                        two kept files side by side: a line for each row that differs, and the sums
//!
//! Each mission is stood from its address as a click on its row stands it, Sam unhurt. A commander who is
//! no tactician gives the squad an order every five seconds: "open and clear" the door nearest the nearest enemy, then "move" onto him, by turns. He is the same for
//! every build, which is all that is asked of him: a row is for setting beside the same row of another build, never a claim about how hard a mission is.
//!
//! WHAT A ROW SAYS, summed over the seeds: the squad's men up at the end of those staged · enemies left of those staged · floors cleared · when the first of the squad
//! fell (median, and in how many runs) · when the floor was clear (median) · rounds fired by each side · magazine changes begun by each side · how long open ways onto floor nobody has been on were within 4 m of
//! a man of the squad, and how much of that a gun of theirs was on them (a second a way) · the enemy man-seconds each
//! rule of the list had a man. Eight seeds of the thirteen at both difficulties (208 runs) are three minutes on four cores.
//!
//! WHAT IT DOES NOT SAY: anything of stealth, of Sam's own gun, of a rescue, or of what a commander who waits and looks would meet. On one seed a floor is a coin toss:
//! read eight, and read the sums first.
use game::game::GuardState;
use game::ids::GuardId;
use game::orders::Who;
use game::teams::Team;
use game_start::play::{dist, Opts, Play};
use std::collections::BTreeMap;

const LEVELS: [&str; 13] = ["barracks", "block", "compound", "defuse", "depot", "floodlit", "glasshouse", "hall", "killhouse", "mousehole", "office", "pass", "records"];
/// an order every so many frames, the first half a second in
const ORDER_EVERY: usize = 300;
const ORDER_FIRST: usize = 30;
/// the open ways near the squad are counted every so many frames, within so far of a man
const WAYS_EVERY: usize = 10;
const WAYS_M: f64 = 4.0;

/// what one run came to
#[derive(Clone, Debug, Default, PartialEq)]
struct Run { up: usize, squad: usize, left: usize, enemies: usize, fell: Option<f64>, clear: Option<f64>, rounds: [f64; 2], reloads: [u32; 2], ways: [f64; 2], had: BTreeMap<String, f64> }

fn file_of(level: &str) -> String { format!("{level}.json") }

fn enemies(p: &Play) -> Vec<GuardId> { p.g.guards.iter().copied().filter(|&m| p.g.gd[m].team == Team::Hostile).collect() }

fn order(p: &mut Play, clear: bool) -> bool {
    let Some(lead) = p.mates().into_iter().find(|&m| p.up(m)) else { return false };
    let at = p.at(lead);
    let near = |a: &[f64; 2], b: &[f64; 2], to: [f64; 2]| dist(*a, to).partial_cmp(&dist(*b, to)).unwrap();
    let Some(man) = enemies(p).into_iter().filter(|&m| p.up(m)).map(|m| p.at(m)).min_by(|a, b| near(a, b, at)) else { return false };
    let door = p.g.doors.list.iter().map(|&d| { let c = p.g.dr[d].frame_centre; [c[0], c[2]] }).min_by(|a, b| near(a, b, man));
    let to = if clear { door.unwrap_or(man) } else { man };
    let target = game::orders::squad_orders_target_at(&mut p.g, [to[0], 0.0, to[1]]);
    let _ = game::orders::squad_orders_give(&mut p.g, if clear { "openClear" } else { "move" }, Some(&target), Some(Who::All), None);
    true
}

fn play(level: &str, hard: bool, seed: u32, secs: f64) -> Run {
    let link = format!("?levelFile={level}.json&roomclear&squad4{}", if hard { "&hard" } else { "" });
    let mut p = Play::mission(&file_of(level), &link, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    let mut r = Run { squad: p.mates().len(), enemies: enemies(&p).len(), ..Default::default() };
    let mut was: BTreeMap<u32, (bool, f64)> = BTreeMap::new();
    let t0 = p.time();
    for f in 0..(secs * 60.0) as usize {
        if f % ORDER_EVERY == ORDER_FIRST && !order(&mut p, (f / ORDER_EVERY) % 2 == 0) { break; }
        p.frame();
        let t = p.time() - t0;
        if r.fell.is_none() && p.mates().iter().any(|&m| !p.up(m)) { r.fell = Some(t); }
        for m in p.g.guards.clone() {
            let side = match p.g.gd[m].team { Team::Operative => 0, Team::Hostile => 1, _ => continue };
            let now = (p.reloading(m).is_some(), p.rounds_fired(m));
            let (reloading, fired) = *was.get(&m.0).unwrap_or(&now);
            if now.0 && !reloading { r.reloads[side] += 1; }
            if now.1 > fired { r.rounds[side] += now.1 - fired; }
            was.insert(m.0, now);
            if side == 1 && p.g.gd[m].state != GuardState::Dead { if let Some(rule) = p.rule(m) { *r.had.entry(rule.to_string()).or_default() += 1.0 / 60.0; } }
        }
        if f % WAYS_EVERY == 0 { for (_, on) in p.watched(WAYS_M) { r.ways[0] += WAYS_EVERY as f64 / 60.0; if on { r.ways[1] += WAYS_EVERY as f64 / 60.0; } } }
        if r.clear.is_none() && enemies(&p).into_iter().all(|m| !p.up(m)) { r.clear = Some(t); }
    }
    r.up = p.mates().into_iter().filter(|&m| p.up(m)).count();
    r.left = enemies(&p).into_iter().filter(|&m| p.up(m)).count();
    r
}

fn median(mut v: Vec<f64>) -> String {
    if v.is_empty() { return "-".into(); }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    format!("{:.1}", v[v.len() / 2])
}

const HEAD: [&str; 16] = ["level", "difficulty", "runs", "squad up", "squad", "enemies left", "enemies", "cleared", "first fell s", "fell in", "clear s", "rounds squad/enemy", "reloads squad/enemy", "ways s", "ways watched s", "rules man-secs"];

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
     had.iter().map(|(k, v)| format!("{k} {v:.1}")).collect::<Vec<_>>().join(" · ")].join("\t")
}

/// `1-4,7` as 1 2 3 4 7
fn seeds_of(word: &str) -> Vec<u32> {
    let n = |x: &str| x.parse::<u32>().unwrap_or_else(|_| fail(&format!("--seeds: '{x}' is no number")));
    word.split(',').flat_map(|p| match p.split_once('-') { Some((a, b)) => (n(a)..=n(b)).collect::<Vec<_>>(), None => vec![n(p)] }).collect()
}

fn fail(why: &str) -> ! { eprintln!("missions: {why}"); std::process::exit(2) }

/// the columns that are numbers to be added up
const ADDED: [usize; 8] = [2, 3, 4, 5, 6, 7, 13, 14];

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
    let mut one = false;
    let mut i = 0;
    while i < args.len() {
        let value = || args.get(i + 1).cloned().unwrap_or_else(|| fail(&format!("{} wants a value", args[i])));
        match args[i].as_str() {
            "--seeds" => { seeds = seeds_of(&value()); i += 1; }
            "--secs" => { secs = value().parse().unwrap_or_else(|_| fail("--secs N")); i += 1; }
            "--std" => both = vec![false],
            "--hard" => both = vec![true],
            "--one" => one = true,   // (a child: this row and no other, in this process)
            w if w.starts_with('-') => fail(&format!("no flag '{w}'")),
            l => levels.push(l.to_string()),
        }
        i += 1;
    }
    if levels.is_empty() { levels = LEVELS.iter().map(|l| l.to_string()).collect(); }
    for l in &levels { if !LEVELS.contains(&l.as_str()) { fail(&format!("no mission '{l}' ({})", LEVELS.join(" "))); } }
    if one {
        let runs: Vec<Run> = seeds.iter().map(|&s| play(&levels[0], both[0], s, secs)).collect();
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
                let out = std::process::Command::new(&exe).args(["--one", "--seeds", &words, "--secs", &secs.to_string(), if *hard { "--hard" } else { "--std" }, level]).output().unwrap();
                if !out.status.success() { fail(&format!("{level}{}: {}", if *hard { " hard" } else { "" }, String::from_utf8_lossy(&out.stderr))); }
                *rows[k].lock().unwrap() = String::from_utf8_lossy(&out.stdout).lines().last().unwrap_or("").to_string();
            });
        }
    });
    for r in &rows { println!("{}", r.lock().unwrap()); }
}
