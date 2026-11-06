//! WHAT A HOST DOES ROUND A STEP OF THE RUST GAME, in one place for every host that replays a TypeScript run beside it: the native runner (bin `mission`)
//! and the wasm face (rust/game_face) call these and nothing of their own, so a mission stepped through wasm is stepped by the same lines as one stepped
//! natively. Nothing here touches a file or a clock: bytes and lines in, a line of the log out.
//!
//! The order round a tick (tools/blitz/BRIDGE.md, "What a Rust host calls round a step"):
//!   a start  `start` (c1.bin, s0.bin) · `lesson` (lesson.json, a tutorial mission's)
//!   before   `host_half` (h<k>.bin) · `give` (each line of gives.txt for this tick) · `write_before` (each line of hostwrites.txt for this tick) · `meter`
//!            (a line of meter.txt, a tutorial mission's) · `upload`
//!   the tick `step`
//!   after    `write_after` (each line of hostwrites.txt for this tick) · the compare (`part_hashes`) · `lights` · `view`
use game::ids::Id;
use game::io::Io;

/// The block's parts by name, in src/sim/state.ts SIM_PARTS' order. post.txt's first line carries the TypeScript's list and the two must agree.
pub const NAMES: [&str; 13] = ["head", "dice", "doors", "falls", "props", "slabs", "items", "men", "lights", "weather", "smoke", "poses", "panes"];
/// POSES: a dead man's ragdoll drive holds a pose the TypeScript keeps OUTSIDE the pool for good, and a Rust pose lives in the pool, so from the first death
/// the pool's bytes differ by that pose and say nothing more. That part is reported when it first parts and is not the alarm.
pub const POSES: usize = 11;

/// one part's hash: its words up to the last that is not zero (a parked block of the TypeScript's may be shorter than the Rust slot; the zeros say nothing)
pub fn part_hash(w: &[u32]) -> u32 {
    let mut n = w.len();
    while n > 0 && w[n - 1] == 0 { n -= 1; }
    let mut h = 0x811c9dc5u32;
    for x in &w[..n] { h = (h ^ *x).wrapping_mul(0x01000193); }
    h
}

/// the hash of each part of the block in rust/sim's slot `slot`, in `NAMES`' order
pub fn part_hashes(slot: usize) -> [u32; 13] {
    let len = core::mem::size_of::<sim::state::State>();
    let w = unsafe { std::slice::from_raw_parts(sim::state::addr(slot) as *const u32, len / 4) };
    let mut out = [0u32; 13];
    for (i, p) in sim::state::PARTS.iter().enumerate() { out[i] = part_hash(&w[p.off / 4..(p.off + p.len) / 4]); }
    out
}

/// A START: the state as the first tick opens (c1.bin) and rust/sim's block then (s0.bin), stood in rust/sim's slot `slot`. The bridge does not write
/// `Game.sim`, and the block is copied by bytes by whoever holds the slot: here. Answers the game and the lines for the log.
pub fn start(state: &[u8], block: &[u8], slot: usize) -> (Box<game::game::Game>, Vec<String>) {
    let mut g = Box::new(game::io::import(state));
    let mut said = Vec::new();
    let len = core::mem::size_of::<sim::state::State>();
    unsafe {
        let at = sim::state::addr(slot) as *mut u8;
        core::ptr::write_bytes(at, 0, len);
        core::ptr::copy_nonoverlapping(block.as_ptr(), at, block.len().min(len));
    }
    g.sim = slot;
    // (the box mirror's world of rust/sim's is a number the bytes carry, and it is the TypeScript game's: this game's world is its own slot's)
    if g.col.mirror.slot >= 0.0 { g.col.mirror.slot = slot as f64; }
    // BRIDGE STAND-IN: THE LEVEL'S SCRIPT. A script is code, hung on the Level by the level's name as it is loaded (scene/levelScripts.ts), and the bridge
    // carries neither which script a beat's body is (BeatDef.fn_ arrives as the type's first arm) nor the define pass's locals (Manifest.vars). The level's
    // name is carried, so the define pass is run again here, as the Director's constructor runs it; the diff compares what it declares with the TypeScript's.
    {
        use game::scene_level::LevelScriptFn as S;
        let script = match g.m.scene_level_load.level_names.as_deref() { Some("tut-sky") => Some(S::TutSky), Some("tut-team") => Some(S::TutTeam), Some("tut-queue") => Some(S::TutQueue), Some("tut-solo") => Some(S::TutSolo), Some("tut-road") => Some(S::TutRoad), Some("tut-kit") => Some(S::TutKit), Some("tut-squad") => Some(S::TutSquad), Some("tut-quiet") => Some(S::TutQuiet), Some("tut-pair") => Some(S::TutPair), _ => None };
        if let Some(s) = script {
            g.level.script = Some(s);
            if g.director.manifest.is_some() { g.director.manifest = Some(game::director::manifest_of(s).expect("the script's define pass")); said.push(format!("the level's script, defined again after the import: {:?}", s)); }
        }
    }
    (g, said)
}

/// BRIDGE STAND-IN: A LESSON'S TESTS (lesson.json, written by mission.ts). The bridge does not carry `LessonStep.done` (a record told apart by its `is`
/// word), so an imported lesson's every test is the type's first arm and no step ever goes by. The file is the level's own `scenarios.lesson.steps[..].done`.
#[derive(Debug, Clone)]
enum J { Null, Bool(bool), Num(f64), Str(String), Arr(Vec<J>), Obj(Vec<(String, J)>) }
fn json(s: &[u8], p: &mut usize) -> J {
    let ws = |p: &mut usize| while *p < s.len() && (s[*p] as char).is_whitespace() { *p += 1; };
    ws(p);
    match s[*p] {
        b'{' => { *p += 1; let mut o = Vec::new(); loop { ws(p); if s[*p] == b'}' { *p += 1; break; } if s[*p] == b',' { *p += 1; continue; } let k = match json(s, p) { J::Str(k) => k, x => panic!("lesson.json: a key {:?}", x) }; ws(p); assert!(s[*p] == b':'); *p += 1; o.push((k, json(s, p))); } J::Obj(o) }
        b'[' => { *p += 1; let mut a = Vec::new(); loop { ws(p); if s[*p] == b']' { *p += 1; break; } if s[*p] == b',' { *p += 1; continue; } a.push(json(s, p)); } J::Arr(a) }
        b'"' => { *p += 1; let mut o = Vec::new(); while s[*p] != b'"' { if s[*p] == b'\\' { *p += 1; o.push(match s[*p] { b'n' => b'\n', b't' => b'\t', c => c }); } else { o.push(s[*p]); } *p += 1; } *p += 1; J::Str(String::from_utf8(o).unwrap()) }
        b't' => { *p += 4; J::Bool(true) } b'f' => { *p += 5; J::Bool(false) } b'n' => { *p += 4; J::Null }
        _ => { let a = *p; while *p < s.len() && matches!(s[*p], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') { *p += 1; } J::Num(std::str::from_utf8(&s[a..*p]).unwrap().parse().unwrap()) }
    }
}
fn lesson_test(j: &J) -> game::scene_scenarios::LessonTestKind {
    use game::scene_scenarios::{LessonTestKind as K, Rect};
    let J::Obj(o) = j else { return K::Unknown { is: String::new() } };
    let get = |k: &str| o.iter().find(|(n, _)| n == k).map(|(_, v)| v);
    let num = |k: &str| match get(k) { Some(J::Num(x)) => Some(*x), _ => None };
    let text = |k: &str| match get(k) { Some(J::Str(x)) => Some(x.clone()), _ => None };
    let flag = |k: &str| match get(k) { Some(J::Bool(x)) => Some(*x), _ => None };
    let v3 = |k: &str| match get(k) { Some(J::Arr(a)) if a.len() == 3 => { let f = |x: &J| if let J::Num(n) = x { *n } else { f64::NAN }; Some([f(&a[0]), f(&a[1]), f(&a[2])]) } _ => None };
    let rect = |k: &str| match get(k) { Some(J::Obj(r)) => { let f = |n: &str| r.iter().find(|(m, _)| m == n).and_then(|(_, v)| if let J::Num(x) = v { Some(*x) } else { None }).unwrap_or(f64::NAN); Some(Rect { x0: f("x0"), x1: f("x1"), z0: f("z0"), z1: f("z1") }) } _ => None };
    let list = |k: &str| match get(k) { Some(J::Arr(a)) => a.clone(), _ => Vec::new() };
    let is = text("is").unwrap_or_default();
    let s = |k: &str| text(k).unwrap_or_default();
    let unknown = || K::Unknown { is: is.clone() };
    match is.as_str() {
        "in" => rect("area").map(|area| K::In { area }).unwrap_or_else(unknown),
        "near" => v3("at").map(|at| K::Near { at, r: num("r") }).unwrap_or_else(unknown),
        "flag" => K::Flag { flag: s("flag"), value: flag("value") },
        "slot" => K::Slot { slot: s("slot"), drawn: flag("drawn") },
        "pace" => K::Pace { below: num("below"), above: num("above") },
        "meter" => K::Meter { which: s("which"), below: num("below"), above: num("above") },
        "stage" => K::Stage { stage: s("stage") },
        "door" => K::Door { door: s("door"), state: s("state") },
        "target" => K::Target { name: s("name"), on: flag("on").unwrap_or(false) },
        "men" => K::Men { down: num("down") },
        "did" => K::Did { what: s("what"), on: text("on"), n: num("n") },
        "downed" => K::Downed { how: s("how"), n: num("n") },
        "body" => rect("area").map(|area| K::Body { area }).unwrap_or_else(unknown),
        "calm" => K::Calm { route: text("route") },
        "loaded" => K::Loaded, "reload" => K::Reload,
        "launcher" => K::Launcher { on: s("on"), fired: flag("fired").unwrap_or(false) },
        "burst" => K::Burst { n: num("n") },
        "threw" => K::Threw { kind: text("kind"), quick: flag("quick").unwrap_or(false), area: rect("area") },
        "orderTo" => K::OrderTo { who: s("who") },
        "stackOn" => K::StackOn { door: s("door"), who: text("who"), set: flag("set").unwrap_or(false) },
        "pairsOn" => K::PairsOn { doors: num("doors") },
        "word" => K::Word { doors: num("doors") },
        "taken" => K::Taken { room: s("room") },
        "following" => K::Following,
        "mate" => K::Mate { area: rect("area"), at: v3("at"), r: num("r"), by: num("by"), away: num("away") },
        "pair" => K::Pair { what: s("what"), n: num("n") },
        "cue" => K::Cue { name: s("name") },
        "view" => K::View { turned: num("turned"), zoomed: num("zoomed"), panned: num("panned") },
        "squad" => rect("area").map(|area| K::Squad { area, n: num("n") }).unwrap_or_else(unknown),
        "stacks" => K::Stacks { doors: list("doors").iter().filter_map(|x| if let J::Str(d) = x { Some(d.clone()) } else { None }).collect(), n: num("n"), set: flag("set").unwrap_or(false) },
        "watching" => v3("at").map(|at| K::Watching { at, r: num("r"), n: num("n") }).unwrap_or_else(unknown),
        "all" => K::All { of: list("of").iter().map(lesson_test).collect() },
        "any" => K::Any { of: list("of").iter().map(lesson_test).collect() },
        _ => unknown(),
    }
}

/// BRIDGE STAND-IN: the lesson's tests from the bytes of lesson.json, over a game just started. Answers the line for the log, or None where the level has no lesson.
pub fn lesson(g: &mut game::game::Game, txt: &[u8]) -> Option<String> {
    let mut p = 0;
    if let (J::Arr(tests), Some(l)) = (json(txt, &mut p), g.level.scenarios.as_mut().and_then(|s| s.lesson.as_mut())) {
        assert!(tests.len() == l.steps.len(), "lesson.json has {} tests and the level {} steps", tests.len(), l.steps.len());
        for (st, j) in l.steps.iter_mut().zip(tests.iter()) { st.done = lesson_test(j); }
        return Some(format!("the lesson's {} tests, handed over (the bridge does not carry a step's `done`)", tests.len()));
    }
    None
}

/// HARNESS STAND-IN: a lesson's harness feeds the player's light meter itself before every tick (a line of meter.txt)
pub fn meter(g: &mut game::game::Game, v: f64) { let pl = g.player; g.pl[pl].visibility = v; }

/// HARNESS STAND-IN: THE HOST'S HALF BETWEEN TWO TICKS (the bytes of h<k>.bin). `Game.update` round the step and the headless host's own asks are not run on
/// this side, so what they wrote is taken as tick k opens: the record the host sampled (`cmd_buf`) and the host's own camera and Input (every row that is no
/// seat's). Nothing else of the TypeScript's state is taken. `first`: the tick the start was taken at, whose record the start itself carries.
pub fn host_half(g: &mut game::game::Game, hb: &[u8], first: bool) -> game::cmd::PlayerCmd {
    let mut r = game::io::R::new(hb);
    let cmd_in = <Option<game::cmd::PlayerCmd> as Io>::r(&mut r);
    let nc = <u32 as Io>::r(&mut r) as usize;
    let cams: Vec<game::camera::FollowCamera> = (0..nc).map(|_| <game::camera::FollowCamera as Io>::r(&mut r)).collect();
    let ni = <u32 as Io>::r(&mut r) as usize;
    let inputs: Vec<game::input::Input> = (0..ni).map(|_| <game::input::Input as Io>::r(&mut r)).collect();
    assert!(r.p == hb.len(), "the host's half: {} bytes left over", hb.len() - r.p);
    if first { return g.cmd_buf.clone().expect("the start holds no record of the tick's input"); }
    let seat_cams: Vec<game::ids::CameraId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].cam).collect();
    for c in g.tab.camera.ids().collect::<Vec<_>>() { if !seat_cams.contains(&c) && c.ix() < cams.len() { g.tab.camera[c] = cams[c.ix()].clone(); } }
    let seat_ins: Vec<game::ids::InputId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].input).collect();
    for i in g.tab.input.ids().collect::<Vec<_>>() { if !seat_ins.contains(&i) && i.ix() < inputs.len() { g.tab.input[i] = inputs[i.ix()].clone(); } }
    g.cmd_buf = cmd_in.clone();
    cmd_in.expect("the TypeScript state holds no record of the tick's input")
}

/// HARNESS STAND-IN: THE TEST DRIVER'S ORDERS. The scripted player gives its orders through the menu's own call between ticks; mission.ts writes each as a line
/// of gives.txt, `<tick> <id> <who> <kind> <x> <y> <z> [door <place> <side> book<id> | man <place>]`, and the same call is made here before that tick.
/// Answers the line for the log.
pub fn give(g: &mut game::game::Game, k: usize, l: &str) -> String {
    use game::orders::{OrderTarget as T, Who};
    let w: Vec<&str> = l.split(' ').collect();
    if l.contains("UNSUPPORTED") || w.len() < 7 { return format!("  NOTE tick {}: an order of the driver's this runner cannot give: {}", k, l); }
    let at = [w[4].parse::<f64>().unwrap(), w[5].parse::<f64>().unwrap(), w[6].parse::<f64>().unwrap()];
    let who = match w[2] { "-" => None, "all" => Some(Who::All), "red" => Some(Who::Red), "blue" => Some(Who::Blue), x => match x.parse::<f64>() { Ok(n) => Some(Who::Num(n)), Err(_) => { return format!("  NOTE tick {}: a `who` this runner cannot give: {}", k, l); } } };
    let t = match w[3] {
        "floor" => { let serial = g.serial(); T::Floor(game::orders::OrderTargetFloor { at, wall: None, thing: None, serial }) }
        "door" => T::Door(game::orders::OrderTargetDoor { at, door: game::ids::DoorId::from_ix(w[8].parse().unwrap()), side: w[9].parse().unwrap() }),
        "none" => T::None(game::orders::OrderTargetNone { at, man: if w.len() > 8 { Some(g.guards[w[8].parse::<usize>().unwrap()]) } else { None } }),
        _ => T::Foe(game::orders::OrderTargetFoe { at, foe: g.guards[w[8].parse::<usize>().unwrap()] }),
    };
    let why = game::orders::squad_orders_give(g, w[1], Some(&t), who, None);
    format!("  the driver's order before tick {}: {} -> {:?}", k, l, why)
}

/// is `l` a line of gives.txt or hostwrites.txt for tick `k` (its first word)
pub fn line_of(l: &str, k: usize) -> bool { l.split(' ').next().and_then(|x| x.parse::<usize>().ok()) == Some(k) }

/// HARNESS STAND-IN, BEFORE THE TICK: a line of hostwrites.txt that is the test driver's own question that writes the game (`g.fight.roomAt(p)` between
/// ticks floods a room and keeps it). Answers the line for the log, or None where the line is not a write made before a tick.
pub fn write_before(g: &mut game::game::Game, k: usize, l: &str) -> Option<String> {
    let w: Vec<&str> = l.split(' ').collect();
    // HARNESS STAND-IN: a lesson's scripted player picks a row of the nearby list between ticks (`Game.selectVerb`), by the bridge's words for the two enums
    if w.get(1) == Some(&"select_verb") {
        let word = |s: &str| -> Option<Vec<u8>> { s.parse::<u16>().ok().map(|x| x.to_le_bytes().to_vec()) };
        let kind = <game::game::InteractableKind as Io>::r(&mut game::io::R::new(&word(w[2]).expect("a kind")));
        let verb = word(w[3]).map(|b| <game::game::Verb as Io>::r(&mut game::io::R::new(&b)));
        let ok = game::game::game_select_verb(g, kind, verb);
        return Some(format!("  the lesson's player before tick {}: selectVerb({} {}) -> {}", k, w[4], w[5], ok));
    }
    if w.get(1) == Some(&"room_at") {
        let p = [w[2].parse::<f64>().unwrap(), w[3].parse::<f64>().unwrap(), w[4].parse::<f64>().unwrap()];
        let r = game::roomfight::room_fight_room_at(g, p);
        return Some(format!("  the driver's question before tick {}: roomAt({:?}) -> {:?} ({} rooms)", k, p, r, g.fight.rooms.len()));
    }
    None
}

/// HOST STEP, AFTER THE TICK: a line of hostwrites.txt that is the autosave's. Answers the lines for the log, or None where the line is not a write made
/// after a tick.
pub fn write_after(g: &mut game::game::Game, k: usize, l: &str) -> Option<String> {
    let mut w = l.splitn(3, ' '); let (_, what, rest) = (w.next(), w.next().unwrap_or(""), w.next().unwrap_or(""));
    if what != "chron_save" { return None; }
    let mut said = String::new();
    if checkpoint(g, rest.trim_matches('"')) { said = format!("  the host's authored checkpoint after tick {}: the level's program stopped for the take\n", k); }
    Some(format!("{}  the host's checkpoint after tick {}: {}", said, k, rest))
}

/// HOST STEP: the host saved, and says so. The autosave controller is the host's, and its one write into the game is the chronicle's row, the tick's last
/// thing. On a tutorial mission the checkpoint is an authored one: the save controller stops the level's program for the take and has the frame start it
/// again on the next tick (savestate.ts:991); the answer is whether it did.
pub fn checkpoint(g: &mut game::game::Game, label: &str) -> bool {
    let mut stopped = false;
    if game::rails::rails_live(g) { if let Some(r) = g.director.run { if !g.tab.director_run[r].ended { game::director::director_stop(g, Some(false)); game::rails::rails_resume(g); stopped = true; } } }
    let _ = game::chronicle::chron_checkpoint(g, game::chronicle::ChronCheckpointWhat::Save, Some(label));
    stopped
}

/// HOST STEP: where the headless host listed the box world again between two ticks (`engine.world.upload()`). The listing is also where the rows of the
/// frames gone by are freed (`boxes_frame_reset`, inside `host_world_upload`): a host that never lists keeps every frame box as a row for good.
pub fn upload(g: &mut game::game::Game) { game::host_api::b_01::host_world_upload(g); }

/// HOST STEP AFTER THE TICK (the headless host's `engine.lights.update(dt, game.time)`): every flash's envelope and retirement, which the next tick's light reads
pub fn lights(g: &mut game::game::Game, dt: f64) { let now = g.time; game::render_lights::light_set_update(g, dt, now); }

/// HOST STEP AFTER THE TICK (game.ts hostAfter): a tutorial mission staged again opens on its first view, set with the host's camera; a return on its
/// first line off a rails level. The host's camera is the row the director was handed, else the first that is no seat's.
pub fn view(g: &mut game::game::Game) {
    let seat_cams: Vec<game::ids::CameraId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].cam).collect();
    let cam = g.director.cam.or_else(|| g.tab.camera.ids().find(|c| !seat_cams.contains(c)));
    if let Some(cam) = cam { game::rails::rails_host_view(g, cam); }
}

/// THE HOST'S HALF OF A GAME OF SEVERAL SEATS (the bytes of h<k>.bin as tools/blitz/seats.ts writes them): the count of seats and a record a seat, then
/// every camera and Input row. A game of two is stepped from its seats' records and from nothing the Game's own `cmd_buf` holds, so the records are read
/// at the first tick too. The cameras and Inputs that are no seat's are the scripted hands' own; they are taken as `host_half` takes them.
pub fn host_half_seats(g: &mut game::game::Game, hb: &[u8]) -> Vec<game::cmd::PlayerCmd> {
    let mut r = game::io::R::new(hb);
    let n = <u32 as Io>::r(&mut r) as usize;
    let cmds: Vec<game::cmd::PlayerCmd> = (0..n).map(|_| <Option<game::cmd::PlayerCmd> as Io>::r(&mut r).expect("a seat's record")).collect();
    let nc = <u32 as Io>::r(&mut r) as usize;
    let cams: Vec<game::camera::FollowCamera> = (0..nc).map(|_| <game::camera::FollowCamera as Io>::r(&mut r)).collect();
    let ni = <u32 as Io>::r(&mut r) as usize;
    let inputs: Vec<game::input::Input> = (0..ni).map(|_| <game::input::Input as Io>::r(&mut r)).collect();
    assert!(r.p == hb.len(), "the host's half: {} bytes left over", hb.len() - r.p);
    let seat_cams: Vec<game::ids::CameraId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].cam).collect();
    for c in g.tab.camera.ids().collect::<Vec<_>>() { if !seat_cams.contains(&c) && c.ix() < cams.len() { g.tab.camera[c] = cams[c.ix()].clone(); } }
    let seat_ins: Vec<game::ids::InputId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].input).collect();
    for i in g.tab.input.ids().collect::<Vec<_>>() { if !seat_ins.contains(&i) && i.ix() < inputs.len() { g.tab.input[i] = inputs[i.ix()].clone(); } }
    cmds
}

/// the tick of a game of several seats, a record a seat
pub fn step_seats(g: &mut game::game::Game, dt: f64, cmds: &[game::cmd::PlayerCmd]) { game::game::game_step_seats(g, dt, cmds); }

/// the tick
pub fn step(g: &mut game::game::Game, dt: f64, cmd: &game::cmd::PlayerCmd) { game::game::game_step(g, dt, cmd); }
