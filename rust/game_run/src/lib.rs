//! WHAT A HOST DOES ROUND A STEP OF THE RUST GAME, in one place for every host that replays a TypeScript run beside it: the native runner (bin `mission`)
//! and the wasm face (rust/game_face) call these and nothing of their own, so a mission stepped through wasm is stepped by the same lines as one stepped
//! natively. Nothing here touches a file or a clock: bytes and lines in, a line of the log out.
//!
//! The order round a tick (tools/blitz/BRIDGE.md, "What a Rust host calls round a step"):
//!   before   `host_half` (h<k>.bin) · `give` (each line of gives.txt for this tick) · `write_before` (each line of hostwrites.txt for this tick) · `upload`
//!   the tick `step`
//!   after    `write_after` (each line of hostwrites.txt for this tick) · the compare (`part_hashes`) · `lights`
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
/// `Game.sim`, and the block is copied by bytes by whoever holds the slot: here.
pub fn start(state: &[u8], block: &[u8], slot: usize) -> Box<game::game::Game> {
    let mut g = Box::new(game::io::import(state));
    let len = core::mem::size_of::<sim::state::State>();
    unsafe {
        let at = sim::state::addr(slot) as *mut u8;
        core::ptr::write_bytes(at, 0, len);
        core::ptr::copy_nonoverlapping(block.as_ptr(), at, block.len().min(len));
    }
    g.sim = slot;
    // (the box mirror's world of rust/sim's is a number the bytes carry, and it is the TypeScript game's: this game's world is its own slot's)
    if g.col.mirror.slot >= 0.0 { g.col.mirror.slot = slot as f64; }
    g
}

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
    if w.get(1) == Some(&"room_at") {
        let p = [w[2].parse::<f64>().unwrap(), w[3].parse::<f64>().unwrap(), w[4].parse::<f64>().unwrap()];
        let r = game::roomfight::room_fight_room_at(g, p);
        return Some(format!("  the driver's question before tick {}: roomAt({:?}) -> {:?} ({} rooms)", k, p, r, g.fight.rooms.len()));
    }
    None
}

/// HOST STEP, AFTER THE TICK: a line of hostwrites.txt that is the autosave's. The autosave controller is the host's, and its one write into the game is
/// the chronicle's row, the tick's last thing. Answers the line for the log, or None where the line is not a write made after a tick.
pub fn write_after(g: &mut game::game::Game, k: usize, l: &str) -> Option<String> {
    let mut w = l.splitn(3, ' '); let (_, what, rest) = (w.next(), w.next().unwrap_or(""), w.next().unwrap_or(""));
    if what == "chron_save" { checkpoint(g, rest.trim_matches('"')); return Some(format!("  the host's checkpoint after tick {}: {}", k, rest)); }
    None
}

/// HOST STEP: the host saved, and says so: the chronicle's row of an autosave
pub fn checkpoint(g: &mut game::game::Game, label: &str) { let _ = game::chronicle::chron_checkpoint(g, game::chronicle::ChronCheckpointWhat::Save, Some(label)); }

/// HOST STEP: where the headless host listed the box world again between two ticks (`engine.world.upload()`)
pub fn upload(g: &mut game::game::Game) { game::host_api::b_01::host_world_upload(g); }

/// HOST STEP AFTER THE TICK (the headless host's `engine.lights.update(dt, game.time)`): every flash's envelope and retirement, which the next tick's light reads
pub fn lights(g: &mut game::game::Game, dt: f64) { let now = g.time; game::render_lights::light_set_update(g, dt, now); }

/// the tick
pub fn step(g: &mut game::game::Game, dt: f64, cmd: &game::cmd::PlayerCmd) { game::game::game_step(g, dt, cmd); }
