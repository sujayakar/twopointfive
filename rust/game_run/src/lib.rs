//! WHAT A HOST DOES ROUND A STEP OF THE RUST GAME, in one place for every host that replays a TypeScript run beside it: the native runner (bin `mission`)
//! and the wasm face (rust/game_face) call these and nothing of their own, so a mission stepped through wasm is stepped by the same lines as one stepped
//! natively. Nothing here touches a file or a clock: bytes and lines in, a line of the log out.
//!
//! The order round a tick (tools/blitz/BRIDGE.md, "What a Rust host calls round a step"):
//!   a start  `start` (c1.bin, s0.bin)
//!   before   `host_half` (h<k>.bin) · `give` (each line of gives.txt for this tick) · `write_before` (each line of hostwrites.txt for this tick) · `meter`
//!            (a line of meter.txt, a tutorial mission's) · `upload`
//!            … or, where the TypeScript side wrote ONE ORDERED RECORD (between.txt), `between` for each of the tick's lines as they stand: the orders,
//!            the writes, the rules the driver called and the meter in the driver's own order, in place of `give`, `write_before` and `meter`
//!   the tick `step`
//!   after    `write_after` (each line of hostwrites.txt for this tick) · the compare (`part_hashes`) · `lights` · `view` · `follow`
use game::ids::Id;
use game::io::Io;

/// The block's parts by name, in src/sim/state.ts SIM_PARTS' order. post.txt's first line carries the TypeScript's list and the two must agree.
pub const NAMES: [&str; 13] = ["head", "dice", "doors", "falls", "props", "slabs", "items", "men", "lights", "weather", "smoke", "poses", "panes"];
/// the pose pool's place among the parts. It is in the alarm like the rest: a pose of its own (a ragdoll's, a function's scratch) is outside the pool on
/// both sides, and a start carries which sets are at home (`PoseSet.home` on the wire).
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
    let mut said: Vec<String> = Vec::new();
    let len = core::mem::size_of::<sim::state::State>();
    unsafe {
        let at = sim::state::addr(slot) as *mut u8;
        core::ptr::write_bytes(at, 0, len);
        core::ptr::copy_nonoverlapping(block.as_ptr(), at, block.len().min(len));
    }
    // (the one way to say which of rust/sim's slots a game stands in: the game's own word and the box mirror's)
    game::io::set_slot(&mut g, slot);
    // (the level's script and its define pass are the import's own now: io::deal_scripts)
    if let Some(sc) = g.level.script { said.push(format!("the level's script, as the import dealt it: {:?}", sc)); }
    (g, said)
}

/// HARNESS STAND-IN: a lesson's harness feeds the player's light meter itself before every tick (a line of meter.txt)
pub fn meter(g: &mut game::game::Game, v: f64) { let pl = g.player; g.pl[pl].visibility = v; }

/// HARNESS STAND-IN: THE HOST'S HALF BETWEEN TWO TICKS (the bytes of h<k>.bin: export.ts `hostHalf`). `Game.update` round the step and the headless host's
/// own asks are not run on this side, so what they wrote is taken as tick k opens, by the bridge's own reader (`io::take_host_half`): a record a SEAT,
/// then every camera and Input row that is no seat's. Nothing else of the TypeScript's state is taken. Answers the records in seat order.
pub fn host_records(g: &mut game::game::Game, hb: &[u8]) -> Vec<game::cmd::PlayerCmd> { game::io::take_host_half(g, hb) }

/// … for a host of ONE seat: the first seat's record of the same bytes (`first` is no longer read: the bytes hold the record at the first tick too)
pub fn host_half(g: &mut game::game::Game, hb: &[u8], _first: bool) -> game::cmd::PlayerCmd { host_records(g, hb).remove(0) }

/// A LESSON'S TESTS are the import's own now (`LessonStep.done` is carried): nothing is handed over, and the answer is no line. Kept for a host that still calls it.
pub fn lesson(_g: &mut game::game::Game, _txt: &[u8]) -> Option<String> { None }

/// HARNESS STAND-IN: THE TEST DRIVER'S ORDERS. The scripted player gives its orders through the menu's own call between ticks; mission.ts writes each as a line
/// of gives.txt, `<tick> <id> <who> <kind> <x> <y> <z> [door <place> <side> book<id> | man <place>] [seat<N>]`, and the same call is made here before that tick.
/// A line that ends `seat<N>` (seats.ts, a game of two: the co-op check's orders) is given as THAT seat, through its own order book (`seats::with_seat`);
/// a line without one is the Game's as it stands between ticks. Answers the line for the log.
pub fn give(g: &mut game::game::Game, k: usize, l: &str) -> String {
    use game::orders::{OrderTarget as T, Who};
    let mut w: Vec<&str> = l.split(' ').collect();
    let seat: Option<usize> = match w.last().and_then(|x| x.strip_prefix("seat")) { Some(n) => { let n = n.parse::<usize>().ok(); w.pop(); n } None => None };
    // HARNESS STAND-IN: a write of the driver's that stands in the orders' own sequence (mission.ts: the scripted commander of `--wall` gives itself a wall
    // charge where the kit issued none, then gives the order on the wall): `<k> !wall_carried <n | null>`, `<k> !wall_raised <n>`
    if w.get(1).map_or(false, |x| x.starts_with('!')) {
        let n = w.get(2).and_then(|x| x.parse::<f64>().ok());
        match w[1] {
            "!wall_carried" => game::wallcharge::wall_charge_set_carried(g, n),
            "!wall_raised" => game::wallcharge::wall_charge_raise_carried(g, n.unwrap_or(0.0)),
            // A SCRIPTED CASE'S WRITE (mission.ts `--case`): the write a rule of the TypeScript makes, made here through the Rust game's own setter; with
            // BLITZ_CASE_RAW set, made as the Rust made it BEFORE the mend (no setter: the array's serial not bumped, the orbit's flag left up)
            "!path_new" => {
                let gd = g.guards[n.unwrap_or(0.0) as usize];
                if std::env::var_os("BLITZ_CASE_RAW").is_some() { g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; }
                else { game::game::guard_set_path(g, gd, Vec::new()); game::guards::guard_set_path_goal(g, gd, None); }
            }
            "!path_goal_same" => {
                let gd = g.guards[n.unwrap_or(0.0) as usize];
                if std::env::var_os("BLITZ_CASE_RAW").is_none() { let p = g.gd[gd].path_goal; game::guards::guard_set_path_goal(g, gd, p); }
            }
            "!f" => { game::nightbreach::night_breach_order(g); }
            // (`--case keeper`: a man a press left to keep his room HEARS A FIGHT outside it, as the room fight writes it when a loud event reaches
            //  him, roomfight.ts:661: `<k> !keeper_hears <guard> <x> <y> <z>`)
            // (`--case keeper-left`: where no press of the run leaves a keeper, the press's own two writes are made for one man with a mate of his room
            //  up: pressure.ts:668 `st.stay.set(keeper, T)` and :698 `st.sent.add(room)`: `<k> !keeper_left <guard> <T's place in the fight's rooms>`)
            "!keeper_left" => {
                let gd = g.guards[n.unwrap_or(0.0) as usize];
                let t = w[3].parse::<usize>().ok().and_then(|i| g.fight.rooms.get(i).copied());
                let d = g.fight.defs.iter().copied().find(|d| g.tab.roomfight_def[*d].gd == gd);
                match (d, t) {
                    (Some(d), Some(t)) => { let room = g.tab.roomfight_def[d].room; let st = game::pressure::state_of(g); st.stay.set(d, t); if let Some(r) = room { st.sent.add(r); } }
                    _ => return format!("  NOTE tick {}: the case's man or room is not of the room fight here: {}", k, l),
                }
            }
            "!keeper_hears" => {
                let gd = g.guards[n.unwrap_or(0.0) as usize];
                let at = [w[3].parse::<f64>().unwrap(), w[4].parse::<f64>().unwrap(), w[5].parse::<f64>().unwrap()];
                let now = g.time;
                match g.fight.defs.iter().copied().find(|d| g.tab.roomfight_def[*d].gd == gd) {
                    Some(d) => { let row = &mut g.tab.roomfight_def[d]; row.noise_at = Some(at); row.noise_t = now; row.noise_out = true; }
                    None => return format!("  NOTE tick {}: the case's man is no defender of the room fight here: {}", k, l),
                }
            }
            _ => return format!("  NOTE tick {}: a write of the driver's this runner does not know: {}", k, l),
        }
        return format!("  the driver's write before tick {}: {}", k, l);
    }
    if l.contains("UNSUPPORTED") || w.len() < 7 { return format!("  NOTE tick {}: an order of the driver's this runner cannot give: {}", k, l); }
    if let Some(n) = seat { if n >= g.seats.len() { return format!("  NOTE tick {}: an order for seat {} in a game of {}: {}", k, n, g.seats.len(), l); } }
    let at = [w[4].parse::<f64>().unwrap(), w[5].parse::<f64>().unwrap(), w[6].parse::<f64>().unwrap()];
    let who = match w[2] { "-" => None, "all" => Some(Who::All), "red" => Some(Who::Red), "blue" => Some(Who::Blue), x => match x.parse::<f64>() { Ok(n) => Some(Who::Num(n)), Err(_) => { return format!("  NOTE tick {}: a `who` this runner cannot give: {}", k, l); } } };
    let t = match w[3] {
        "floor" => {
            // (a floor target that carries a wall: ` wall <index> <box> <alongX> <u> <side> <ok> <outer> x<why as hex>`, mission.ts wallTail)
            let wall = if w.get(7) == Some(&"wall") && w.len() >= 16 {
                let n = |i: usize| w[i].parse::<f64>().unwrap();
                let hex = &w[15][1..];
                let why = String::from_utf8((0..hex.len() / 2).map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap()).collect()).unwrap();
                Some(game::wallcharge::WallSpot { index: n(8), box_: game::ids::BoxId::from_ix(w[9].parse().unwrap()), along_x: w[10] == "1", u: n(11), side: n(12), ok: w[13] == "1", why, outer: match w[14] { "-1" => None, x => Some(x == "1") } })
            } else { None };
            let serial = g.serial(); T::Floor(game::orders::OrderTargetFloor { at, wall, thing: None, serial })
        }
        "door" => T::Door(game::orders::OrderTargetDoor { at, door: game::ids::DoorId::from_ix(w[8].parse().unwrap()), side: w[9].parse().unwrap() }),
        "none" => T::None(game::orders::OrderTargetNone { at, man: if w.len() > 8 { Some(g.guards[w[8].parse::<usize>().unwrap()]) } else { None } }),
        _ => T::Foe(game::orders::OrderTargetFoe { at, foe: g.guards[w[8].parse::<usize>().unwrap()] }),
    };
    let why = match seat {
        Some(n) => { let sid = g.seats[n]; game::seats::with_seat(g, sid, &mut |v: &mut game::game::Game| game::orders::squad_orders_give(v, w[1], Some(&t), who, None)) }
        None => game::orders::squad_orders_give(g, w[1], Some(&t), who, None),
    };
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
    // HARNESS STAND-IN: the scripted commander blows a set wall charge by the module's own call between ticks (wallcharge-commander.ts `C.blow(c)`)
    if w.get(1) == Some(&"wall_blow") {
        let ix = w[2].parse::<f64>().unwrap();
        let c = g.wall_charge.set.iter().copied().find(|c| g.tab.wallcharge_set_wall_charge[*c].wall.index == ix);
        let r = game::wallcharge::wall_charge_blow(g, c, None);
        return Some(format!("  the driver before tick {}: blows the wall charge on wall {} ({}) -> {:?}", k, ix, if c.is_some() { "set" } else { "NOT SET HERE" }, r));
    }
    // HARNESS STAND-IN: the scripted player gives the hostages (rescueend.ts `word`) or the man in hand (snatchout.ts `word`) the word by the module's own
    // call between ticks: `player` they follow the player, `none` they stay where they stand
    if w.get(1) == Some(&"rescue_word") || w.get(1) == Some(&"snatch_word") {
        let by = match w.get(2) { Some(&"player") => Some(g.pl[g.player].ch), Some(&"none") => None, x => return Some(format!("  NOTE tick {}: a word to follow somebody this runner cannot give: {:?}", k, x)) };
        let ok = if w[1] == "rescue_word" { game::rescueend::rescue_end_word(g, by) } else { game::snatchout::snatch_out_word(g, by) };
        return Some(format!("  the driver before tick {}: {}({}) -> {}", k, w[1], w[2], ok));
    }
    // HARNESS STAND-IN: the scripted player puts the highlight on a row of the nearby list between ticks (it writes `Player.nearSel` and calls
    // pickHover: playthrough.ts:865, and `Game.selectVerb` on a mission that is no lesson)
    if w.get(1) == Some(&"near_sel") {
        let i = w[2].parse::<i32>().expect("a row");
        let pl = g.player; g.pl[pl].near_sel = i; game::player::pick_hover(g);
        return Some(format!("  the driver before tick {}: the highlight on row {} of {} ({:?})", k, i, g.nearby.len(), g.hover.as_ref().map(|h| h.line1.clone())));
    }
    if w.get(1) == Some(&"room_at") {
        let p = [w[2].parse::<f64>().unwrap(), w[3].parse::<f64>().unwrap(), w[4].parse::<f64>().unwrap()];
        let r = game::roomfight::room_fight_room_at(g, p);
        return Some(format!("  the driver's question before tick {}: roomAt({:?}) -> {:?} ({} rooms)", k, p, r, g.fight.rooms.len()));
    }
    None
}

/// HARNESS STAND-IN: A RULE OF THE GAME THE TEST DRIVER CALLS BETWEEN TWO TICKS, by its name (a `call` line of between.txt). The scripted player asks
/// where a player who read a grenade's cue would go before every step (tools/qa/playthrough.ts `fragStepOff(g)`); the ask makes the flee's guess of the
/// grenade's rest and fills the memo of the walls its burst would open, which the tick's own flee then reads, so it is made here too. A rule this
/// does not know is a NOTE in the log, never silence. It belongs on the generated call surface, by the rule's own name.
pub fn call(g: &mut game::game::Game, k: usize, rule: &str) -> String {
    let all: Vec<&str> = rule.split(' ').collect();
    let mut w = rule.split(' ');
    let (rule, arg) = (w.next().unwrap_or(""), w.next());
    match rule {
        // (`--case sticky`: one sticky camera leaves the player's tube by the module's own call, stickycam.ts fireCam, from and toward the points the
        //  TypeScript side chose: `call sticky_fire <from x y z> <target x y z>`. The count in the tube is the caller's to take and is left alone.)
        "sticky_fire" => {
            let n: Vec<f64> = all[1..].iter().filter_map(|x| x.parse::<f64>().ok()).collect();
            if n.len() != 6 { return format!("  NOTE tick {}: sticky_fire takes six numbers: {:?}", k, all); }
            let pl = g.player;
            let c = game::stickycam::fire_cam(g, pl, [n[0], n[1], n[2]], [n[3], n[4], n[5]]);
            format!("  the case's call before tick {}: a sticky camera fired ({:?}); {} out", k, c, g.pl[pl].cams.len())
        }
        // (a lesson's player asks the words of a set door charge: the answer is kept a quarter second and the tick's own ask reads it, doortools.ts closeAt)
        "door_close_words" => match arg.and_then(|x| x.parse::<usize>().ok()).and_then(|i| g.door_tools.set.get(i).copied()) {
            Some(c) => { let said = game::doortools::door_tools_close_words(g, c); format!("  the driver's ask before tick {}: door_close_words -> {:?}", k, said) }
            None => format!("  NOTE tick {}: the driver asked the words of a set charge this game does not hold: {:?}", k, arg),
        },
        "frag_step_off" => { let r = game::frag::frag_step_off(g, None); if r.is_some() { format!("  the driver's ask before tick {}: frag_step_off -> {:?}", k, r) } else { String::new() } }
        // (BLITZ_CALL, mission.ts: a rule of the game called before a tick on both sides, to hold the Rust game's restart to the TypeScript's)
        "restart_encounter" => { game::game::game_restart_encounter(g); format!("  the rule called before tick {}: restart_encounter; {} guards, time {}", k, g.guards.len(), g.time) }
        "reset_guards" => { game::game::game_reset_guards(g); format!("  the rule called before tick {}: reset_guards (Game.resetGuards); {} guards, time {}", k, g.guards.len(), g.time) }
        "qrf_panel" => { let r = game::qrf::request_qrf(g, game::qrf::QrfReason::Panel); format!("  the rule called before tick {}: request_qrf(panel) -> {}", k, r) }
        // (the response team staged as tools/qa/qrf-squad-check.ts stages it: mission.ts `qrf_stage` writes the same rows)
        "qrf_stage" => {
            let p = g.ch[g.pl[g.player].ch].pos;
            g.tune.qrf = true; g.tune.qrf_eta = 14.0; g.tune.qrf_size = 4.0; g.tune.perimeter = false; g.tune.qrf_squad = true;
            g.alarm.placed = true; g.alarm.pos = [p[0], p[1], p[2]]; g.alarm.raised_at = g.time;
            let r = game::qrf::request_qrf(g, game::qrf::QrfReason::Panel);
            format!("  the rule called before tick {}: the response team staged, request_qrf(panel) -> {}", k, r)
        }
        "restart" => { game::game::game_restart(g, None); format!("  the rule called before tick {}: restart (the player's own); {} guards, time {}", k, g.guards.len(), g.time) }
        _ => format!("  NOTE tick {}: a rule the driver called that this runner does not know: {}", k, rule),
    }
}

/// HARNESS STAND-IN: ONE LINE OF THE DRIVER'S ORDERED RECORD (between.txt, mission.ts `between`): `<k> give …` an order, `<k> write …` a write made
/// before the tick, `<k> call <rule>` a rule called, `<k> meter <v>` the player's light meter. A host replays a tick's lines as they stand, so an order
/// given after a write is given after it here. `off` names the kinds or rules left out (BLITZ_NO_BETWEEN, to see by a run what one is for).
/// Answers the line for the log (empty: nothing worth a line).
pub fn between(g: &mut game::game::Game, k: usize, l: &str, off: &[String]) -> String {
    let mut w = l.splitn(3, ' ');
    let (tick, kind, rest) = (w.next().unwrap_or(""), w.next().unwrap_or(""), w.next().unwrap_or(""));
    if off.iter().any(|o| o == kind || (kind == "call" && rest.split(' ').next() == Some(o.as_str())) || (kind == "write" && rest.split(' ').next() == Some(o.as_str()))) { return String::new(); }
    let line = format!("{} {}", tick, rest);
    match kind {
        "give" => give(g, k, &line),
        "write" => write_before(g, k, &line).unwrap_or_else(|| format!("  NOTE tick {}: a write of the driver's this runner does not make before a tick: {}", k, line)),
        "call" => call(g, k, rest),
        "meter" => { if let Ok(v) = rest.parse::<f64>() { meter(g, v); } String::new() }
        _ => format!("  NOTE tick {}: a line of the driver's record this runner does not know: {}", k, l),
    }
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

/// HOST STEP BEFORE THE TICK (game.ts update, its first lines: `if (this.restartCut) { this.restartCut = false; … }`): the player's own restart
/// (`Game.restart`: Shift+Enter, Enter when down, a card's button) leaves a flag for the host's next frame, which takes the seat's cut and puts it on the
/// host's camera before that tick's record reads the view. The camera and the seat's mark come with the host's half; the flag is the game's and is put
/// down here, as the host's frame puts it down.
pub fn restart_cut(g: &mut game::game::Game) { if g.restart_cut { g.restart_cut = false; } }

/// HOST STEP AS A TICK OPENS, after the host's half is taken (game.ts hostAfter: `me.view.viewProj = cam.viewProj`): between two ticks the view's matrix
/// IS the host camera's live matrix (the TypeScript holds the camera's own array, which the host's `cam.update` goes on writing after the tick), so whoever
/// asks on-screen-or-not from outside a tick (an order the driver gives: the man's line is a row of the log only when he is off the screen) reads the
/// camera as the host left it. The Rust view holds a copy, taken here from the host's camera row once the host's half has brought it. For a host of ONE
/// seat that goes through `Game.update`; a game of several seats stepped by `stepSeats` alone (seats-run.ts) never hands the host's matrix back.
pub fn view_live(g: &mut game::game::Game) {
    if g.seats.len() != 1 { return; }
    let seat_cams: Vec<game::ids::CameraId> = g.seats.iter().map(|s| g.tab.cmd_seat[*s].cam).collect();
    let cam = g.director.cam.or_else(|| g.tab.camera.ids().find(|c| !seat_cams.contains(c)));
    if let Some(cam) = cam {
        let vp = g.tab.camera[cam].view_proj;
        let me = g.seats[0];
        game::seats::with_seat(g, me, &mut |g| g.view_proj = Some(game::game::GameViewProj::Float32Array(vp.to_vec())));
    }
}

/// HOST STEP AFTER THE TICK (the headless host's `cam.update(dt, view.followPos(), …)`, for each seat a host shows): the host's camera asks the game where
/// to look. On a tutorial's sky seat that ask is not idle: `rails.look()` eases the view's centre toward the men still up by the time since it was last
/// asked and stamps it, so a game nobody asks between ticks holds another centre (the ninth digit while the tick asks too, a stale stamp where it does not).
pub fn follow(g: &mut game::game::Game) {
    for s in g.seats.clone() { game::seats::with_seat(g, s, &mut |g| { let _ = game::game::game_follow_pos(g, None); }); }
}

/// the tick, a record a seat (a game of one seat: `game_step`, which is `stepSeats(dt, [record])`)
pub fn step_seats(g: &mut game::game::Game, dt: f64, cmds: &[game::cmd::PlayerCmd]) {
    if cmds.len() == 1 { game::game::game_step(g, dt, &cmds[0]); } else { game::game::game_step_seats(g, dt, cmds); }
}

/// the tick of a game of one seat
pub fn step(g: &mut game::game::Game, dt: f64, cmd: &game::cmd::PlayerCmd) { game::game::game_step(g, dt, cmd); }
