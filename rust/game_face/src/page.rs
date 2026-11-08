//! THE FACE'S EXPORTS FOR A HOST THAT SHOWS THE GAME (the page, src/host/rustgame.ts): the Rust game steps, and after each step only what the picture
//! reads goes back, as the filtered export (`game::puppet::export_puppet`, tools/blitz/PUPPET.md) with the step's out events on its tail.
//!
//!   face_start(p, n, slot, seed, seats, flags) -> h   A GAME STARTED HERE by the game's own start (rust/game_start) from the level file's text, the
//!                                      address's words and the character's files, handed as one run of bytes; no state crosses
//!   face_new_at(state, n, slot) -> h   a game from a start (the bridge's bytes) OVER THE BLOCK THAT ALREADY STANDS in rust/sim's slot `slot`: the host's
//!                                      own code stood the level through this same module's `rust/sim` exports, so the block is not handed over and not
//!                                      copied anywhere else. The host says which slot (src/sim/state.ts holds the slots); the game is pinned to it.
//!   face_puppet(h, start)              the filtered export at face_out. `start` = 1: the long list and every row (after face_new_at, and it fills the
//!                                      memo of what was last sent); 0: what changed since the last call. The out events ride on its tail, and the
//!                                      game's list of them is emptied here: they are a step's output, read once.
//!   face_hand(h, seat, p, pn, keys, n) WHAT A PLAYER DID, for the next step: `HAND` numbers (`pn` bytes; src/draw/hand.ts says them in the same order) and his keys' codes as
//!                                      text, those held and, after a line feed, those that went down. A seat nobody hands is as it last was, with
//!                                      nothing pressed.
//!   face_recover(h, newest) -> h | 0   a game lost to a panic stands again from the best kept of it, or from its run's first step (`face_recover_what(h)`: from what)
//!   face_commands(h, p, n) -> 1 | 0    WHAT A PLAYER OR A PANEL DID THAT IS NOT INPUT, before a step: the typed list (game_run::command), done in its order.
//!                                      0 = a number no command has (said at face_out)
//!   face_draw(h, want) -> p            WHAT THE LAST TICK LEFT TO BE DRAWN (the crate game_draw): the sections of `want` made again from the game as it
//!                                      stands, which is not written. `p` is a table of two words a section, its address and its rows; they lie here
//!                                      until the next call. Once a tick.
//!   face_frame(h, a, x, y, z) -> p     A FRAME IS DRAWN, `a` of the way from the tick before to the last (1 = where the last left everything), looking at
//!                                      x y z: the rows that move stand there, and the table again
//!   face_saves(h)                      this run's saves as a card lays them out, at face_out: the mission's clock (f64), would one by hand be made now (u32), the key of the one to go back to (text), how many,
//!                                      then each: its key and label (text), 1 by hand and 2 regretted (u32), the clock it was made at and which came after which (f64)
//!   face_saves_told(h) -> n            what the save controller tells the player, at face_out: its tag while it is up, then a line for each thing it has said since it was last asked (0: nothing of either)
//!   face_knobs(h)                      every knob a person may turn, at face_out: its name's length (its top bit: a yes or no), the name, what it is turned to (f64)
//!   face_draw_name(h, i)               a name that crosses by its number (a sound's), at face_out
//!   face_draw_look(h, p, n)                 what the page says of its look (game_draw::LOOK numbers), once and when it changes
//!   face_draw_cut(h)                   the picture goes on from where the game stands now, and blends from nothing before it (ticks were stepped that were not drawn)
//!   face_draw_layout() -> stamp        the stamp of the sections' declarations (and their text at face_out): a page whose offsets are of another is refused
//!   face_poke(h, what, v) -> 1 | 0     a host's switch written between ticks, by number: 0 god mode · 1 the AI on · 2 infinite ammunition · 3 the
//!                                      player unseen · 4 the launcher's held button let go unsent (the pause card: host/loop.ts). `v` != 0 = on.
//!   face_mem() -> bytes                the module's memory now (a host holds its typed arrays to this: a memory that grew has detached them)
//!
//! A GAME KEPT (a checkpoint, the start a restart goes back to). The host's save controller (src/game/savestate.ts) decides when and under which label; what
//! it keeps is the Rust game's own, held here and named by a number the controller's slot holds:
//!   face_keep(h, how) -> k             how 0: THE GAME'S OWN VALUE (a clone of the whole `Game`, every field the bridge does not carry with it: a closure
//!                                      in flight, the counters) and its block of rust/sim's. Nothing is lost by it, and it lives only
//!                                      as long as this module. how 1: THE BRIDGE'S BYTES (`io::export`) and the block: what a file holds.
//!   face_keep_given(s, n, b, bn) -> k  a kept game from a file's two parts
//!   face_kept_state(k) · face_kept_block(k)   at face_out: the bridge's bytes of kept `k` (a clone is exported now) · its block
//!   face_kept_drop(k) · face_kept_n()  let go · how many are held
//!   face_back(h, k) -> 1 | 0           game `h` IS kept `k` again, in the slot `h` stands in (the kept one stays kept). A clone is cloned back; bytes go
//!                                      through the start's own lines (`game_run::start`: `io::import`, so what the bridge does not carry starts blank).
//!                                      What `face_puppet` last sent is forgotten (memo.rs `forget`): the host applies the whole state next.
//!   face_loaded(h, label, n, loads, saves)   STAND-IN: WHAT THE SAVE CONTROLLER DOES TO THE GAME ONCE A SAVE IN MEMORY IS PUT BACK (savestate.ts loadMem,
//!                                      the lines after the snapshot is restored: :1124 to :1134). A ring of the hand's that merely stood at the save is
//!                                      put away, a gun's setup is not up, the squad's ring is put away, the run's counts of loads and saves are the
//!                                      abandoned moment's (handed in: the kept game holds the saved moment's), the chronicle takes its row, and a
//!                                      tutorial's program is to be started again. After `face_back`, before the next step. Belongs in rust/game's
//!                                      savestate (`checkpoints_load_mem`, an empty body today: the controller is the host's).
//!   face_saves_in(h, p, n)             THE CONTROLLER'S SHELF AS THE TICK READS IT (`g.saves`: Enter once down asks for this run's newest checkpoint,
//!                                      player.ts:493): the run's name and each slot's row. u32 count-prefixed utf-8 for a text; the run, u32 rows, then
//!                                      each: key, slot (u8: 0 auto, 1 manual), n (f64), label, saved_at, mission_t (f64), stage, its run, a byte and
//!                                      the regret's words where the byte is 1.
//!   face_saves_asked(h) -> bits        what the tick asked of the controller, read once: 1 a save in memory is due back (its key at face_out) ·
//!                                      2 the player restarted (`game_restart` ran: the host's camera owes a cut)
//!
//! AFTER A PANIC. The module aborts on a panic and the host sees a trap; the game of that call is gone (lib.rs `with` takes it out for the call, so no
//! borrow is left behind), and every game KEPT is still whole. The host goes on in the same instance:
//!   face_moment(h) -> k | 0 · face_moment_back(h, k) -> 1 | 0 · face_moment_drop(k)   moment.rs: A MOMENT OF A GAME, whole, for a walk of a tape to come back to
//!   face_stand(k, slot) -> h | 0       A NEW GAME that is kept `k`, standing in `slot` (as `face_back`, with no game to stand it in). Its serial counter
//!                                      starts no lower than the highest any export has shown the host (`serial_seen`): a number means one thing.
//!   face_debug_panic(h, how)           A PLANTED PANIC, for the check of that road (the page's `?rustpanic=<tick>`): how 0 inside the borrow of game `h`,
//!                                      as a rule of the tick would (the game is lost); how 1 outside any game (nothing is lost but the call).
use std::cell::RefCell;
use std::rc::Rc;

/// a game kept: its own value, or the bridge's bytes of it; and its block
pub(super) enum Kept { Own(Box<game::game::Game>, Vec<u8>), Bytes(Vec<u8>, Vec<u8>) }

/// a game the save controller keeps: its number, which came after which, its label, and whether the floor came down straight after it
#[derive(Clone)]
pub(super) struct Shelved { pub(super) k: u32, at: f64, label: String, regret: bool }

thread_local! {
    /// the games kept, by number less one
    static KEPT: RefCell<Vec<Option<Rc<Kept>>>> = const { RefCell::new(Vec::new()) };
    /// the games the save controller keeps, by the game they are of and its key: here and not with the game, so a game lost to a panic leaves them
    static SHELF: RefCell<std::collections::BTreeMap<(u32, String), Shelved>> = const { RefCell::new(std::collections::BTreeMap::new()) };
}
fn block_len() -> usize { core::mem::size_of::<sim::state::State>() }
pub(super) fn block_of(slot: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, block_len()).to_vec() } }
fn keep(k: Kept) -> u32 { keep_rc(Rc::new(k)) }
fn keep_rc(k: Rc<Kept>) -> u32 {
    KEPT.with(|v| { let mut v = v.borrow_mut(); match v.iter().position(|x| x.is_none()) { Some(i) => { v[i] = Some(k); i as u32 + 1 } None => { v.push(Some(k)); v.len() as u32 } } })
}

/// A GAME STARTED HERE, by the game's own start (rust/game_start `stand`), from what the host hands in one run of bytes (`handed.rs`: the level
/// file's text, the address's words, the character's files). No state crosses: the level is read, the men are made and the mission is staged on
/// this side, as the TypeScript start does them (rust/game_start/LEVELS.md holds the two to each other, mission by mission).
/// `slot`: the slot of rust/sim to stand in, whose block is made anew; below 0 = a free one. `flags`: 1 = the rows that dress the picture are
/// up (a page). Answers the game, or 0 where no slot is free.
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_start(p: *const u8, n: usize, slot: i32, seed: u32, seats: u32, flags: u32) -> u32 {
    super::hook();
    let slot = if slot < 0 { match super::free_slot() { Some(s) => s, None => return 0 } } else { slot as usize };
    if slot >= sim::state::GAMES { return 0; }
    let o = game_start::stand::StartOpts { seed, seats, slot, dress: flags & 1 != 0, ..Default::default() };
    let stood = game_start::handed::stand_handed(super::bytes(p, n), &o, &mut |g, b| if let Err(why) = game_run::command::apply(g, b) { panic!("face_start: {why}") });
    let h = super::put(super::Held::new(stood.game));
    crate::memo::forget(h);
    h
}

/// # Safety
/// `state`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_new_at(state: *const u8, n: usize, slot: usize) -> u32 {
    super::hook();
    if slot >= sim::state::GAMES { return 0; }
    // (the start's own lines take a block as bytes and write it into the slot: handed the slot's own bytes, the block stands as it stood)
    let len = core::mem::size_of::<sim::state::State>();
    let block = std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, len).to_vec();
    let (g, said) = game_run::start(super::bytes(state, n), &block, slot);
    super::out(said.join("\n").into_bytes());
    let h = super::put(super::Held::new(g));
    crate::memo::forget(h);
    h
}

#[no_mangle]
pub extern "C" fn face_mem() -> usize {
    core::arch::wasm32::memory_size(0) * 65536
}

/// the numbers of a hand: buttons clicked wheel, cursor x y, the view's size in CSS pixels and in its own, the view-projection and its inverse, the eye, where it
/// looks, the look-at, yaw pitch distance, field of view, aspect, how far it leads the aim, and the distance it is on its way to
const HAND: usize = 57;
/// # Safety
/// `p`, `pn` and `keys`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_hand(h: u32, seat: usize, p: *const f64, pn: usize, keys: *const u8, n: usize) {
    use game::cmd::{PlayerCmd, PlayerCmdView};
    if pn != HAND * 8 { panic!("face_hand: {} bytes, and a hand is {} numbers", pn, HAND); }
    let v = std::slice::from_raw_parts(p, HAND);
    let text = super::text(keys, n);
    let mut lines = text.split('\n');
    let mut codes = || lines.next().unwrap_or("").split(' ').filter(|k| !k.is_empty()).map(String::from).collect::<Vec<String>>();
    let three = |i: usize| [v[i], v[i + 1], v[i + 2]];
    let cmd = PlayerCmd {
        keys: codes(), pressed: codes(), buttons: v[0], clicked: v[1], wheel: v[2], mx: v[3], my: v[4], w: v[5], h: v[6], pw: v[7], ph: v[8],
        view: PlayerCmdView { vp: v[9..25].to_vec(), ivp: v[25..41].to_vec(), pos: three(41), forward: three(44), target: three(47), yaw: v[50], pitch: v[51], distance: v[52], fov: v[53], aspect: v[54], aim_lead: v[55] },
    };
    super::with(h, |x| {
        // (the host's own camera, as the hand has it: what a rule asks of the host's view, and where one writes to it)
        if let (true, Some(c)) = (x.g.seats[seat] == game::game::game_host_seat(&x.g), x.g.host_cam) {
            x.drawn.desired = v[56];
            // (where the view looks: what is gathered round it, gusts and embers, follows it and not the man)
            x.g.engine.view_focus = Some(cmd.view.target);
            let (c, v) = (&mut x.g.tab.camera[c], &cmd.view);
            (c.target, c.pos, c.forward) = (v.target, v.pos, v.forward);
            (c.yaw, c.pitch, c.distance, c.desired_distance, c.fov, c.aspect) = (v.yaw, v.pitch, v.distance, x.drawn.desired, v.fov, v.aspect);
        }
        let mut cmd = cmd;
        if x.hands.as_ref().is_some_and(|h| h.0 == seat) {
            // (he asks of the box world: its copy stands, as for any question between two ticks)
            if game::ask::WorldAsk::try_of(&x.g).is_err() { game::scene_collision::stands(&mut x.g); }
            game::scene_box_mirror::box_mirror_things(&mut x.g);
            if let Some((_, h)) = &mut x.hands { h.hand(&x.g, &mut cmd, 1.0 / 60.0); }
        }
        x.hand_over(seat, cmd);
    });
}
/// NOBODY'S HANDS ARE ON A SEAT (game_run hands_off): the hands it is handed from here are his own. `p`, `n`: his plan, a word a line: the seed, the seconds he plays, 1 to throw the breaker
/// first, then doors by name to take first. An empty plan: the seat is the host's again.
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_hands_off(h: u32, seat: usize, p: *const u8, n: usize) {
    let text = super::text(p, n);
    let w: Vec<&str> = text.split('\n').collect();
    super::with(h, |x| x.hands = if text.is_empty() { None } else {
        let num = |i: usize| w.get(i).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        let plan = game_run::hands_off::Plan { seed: num(0) as u32, cap: num(1), breaker: num(2) != 0.0, route: w.iter().skip(3).map(|s| String::from(*s)).collect() };
        Some((seat, game_run::hands_off::HandsOff::new(x.g.seats[seat], plan)))
    });
}
/// how he stands, as JSON at face_out (empty: nobody is hands off): seconds played, how it ended, orders given, when a round last left a gun, the squad's men up and of how many, defenders left
#[no_mangle]
pub extern "C" fn face_hands_said(h: u32) {
    let s = super::with(h, |x| match &x.hands { None => String::new(), Some((seat, p)) => {
        let g = &x.g;
        let squad = game::orders::squad_orders_squad(g, &game::seats::seat(g, g.seats[*seat]));
        let up = squad.iter().filter(|&&m| g.gd[m].state != game::game::GuardState::Dead).count();
        format!("{{\"t\":{},\"end\":{},\"given\":{},\"shotAt\":{},\"up\":{},\"of\":{},\"left\":{}}}", p.t(), p.end().map_or(String::from("null"), |e| format!("\"{}\"", e.as_str())), p.given(), p.shot_at(), up, squad.len(),
            g.guards.iter().filter(|&&m| g.gd[m].team == game::teams::Team::Hostile && g.gd[m].state != game::game::GuardState::Dead).count())
    } });
    super::out(s.into_bytes());
}
/// his timeline, a line an entry, at face_out
#[no_mangle]
pub extern "C" fn face_hands_trace(h: u32) { let s = super::with(h, |x| x.hands.as_ref().map_or(String::new(), |p| p.1.trace())); super::out(s.into_bytes()); }
#[no_mangle]
pub extern "C" fn face_draw(h: u32, want: u32) -> *const u32 {
    super::with(h, |x| { x.drawn.take(&x.g, want); x.drawn.table().as_ptr() })
}
#[no_mangle]
pub extern "C" fn face_frame(h: u32, a: f64, x: f64, y: f64, z: f64) -> *const u32 {
    super::with(h, |k| { k.drawn.frame(&k.g, a, [x, y, z]); k.drawn.table().as_ptr() })
}
/// # Safety
/// `p` names `n` numbers the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_draw_look(h: u32, p: *const f64, n: usize) {
    if n != game_draw::LOOK { panic!("face_draw_look: {} numbers, and a look is {}", n, game_draw::LOOK); }
    let v = std::slice::from_raw_parts(p, n);
    super::with(h, |k| k.drawn.look(v));
}
#[no_mangle]
pub extern "C" fn face_draw_cut(h: u32) { super::with(h, |x| x.drawn.cut()); }
#[no_mangle]
pub extern "C" fn face_saves_told(h: u32) -> usize {
    let t = super::with(h, |x| {
        let (f, secs) = (x.g.saves.flash.as_ref(), x.g.k.ckpt.flash_secs);
        let mut t = String::from(f.filter(|f| x.g.time >= f.t && x.g.time - f.t < secs).map_or("", |f| f.text.as_str()));
        for s in x.said.drain(..) { t.push('\n'); t.push_str(&s); }
        t
    });
    let n = t.len();
    super::out(t.into_bytes());
    n
}
#[no_mangle]
pub extern "C" fn face_saves(h: u32) {
    let b = super::with_ref(h, |x| {
        let g = &x.g;
        let mut b = Vec::new();
        let text = |b: &mut Vec<u8>, s: &str| { b.extend_from_slice(&(s.len() as u32).to_le_bytes()); b.extend_from_slice(s.as_bytes()); };
        b.extend_from_slice(&g.mission.stats.elapsed.to_le_bytes());
        b.extend_from_slice(&(game::savestate::control::hand_no(g).is_none() as u32).to_le_bytes());
        let offer = game::savestate::checkpoints_newest(g, Some(true), Some(g.mission.stats.elapsed)).filter(|i| game::savestate::checkpoints_is_this_run(g, i));
        text(&mut b, offer.as_ref().map_or("", |i| i.key.as_str()));
        let rows = game::savestate::control::timeline(g);
        b.extend_from_slice(&(rows.len() as u32).to_le_bytes());
        for r in rows {
            text(&mut b, &r.key);
            text(&mut b, &r.label);
            b.extend_from_slice(&((r.slot == game::savestate::SlotInfoSlot::Manual) as u32 | (r.regret.is_some() as u32) << 1).to_le_bytes());
            b.extend_from_slice(&r.mission_t.to_le_bytes());
            b.extend_from_slice(&r.saved_at.to_le_bytes());
        }
        b
    });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_knobs(h: u32) {
    let b = super::with(h, |x| {
        let mut b = Vec::new();
        for (name, yes, v) in game_run::knobs::all(&x.g) {
            b.extend_from_slice(&(name.len() as u32 | (yes as u32) << 31).to_le_bytes());
            b.extend_from_slice(name.as_bytes());
            b.extend_from_slice(&v.to_le_bytes());
        }
        b
    });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_draw_name(h: u32, i: usize) {
    let b = super::with(h, |k| k.drawn.events.names.get(i).cloned().unwrap_or_default());
    super::out(b.into_bytes());
}
#[no_mangle]
pub extern "C" fn face_draw_layout() -> u32 {
    let t = game_draw::layout();
    let mut s = 0x811c_9dc5u32;
    for b in t.bytes() { s = (s ^ b as u32).wrapping_mul(0x0100_0193); }
    super::out(t.into_bytes());
    s
}
#[no_mangle]
pub extern "C" fn face_poke(h: u32, what: u32, v: f64) -> u32 {
    super::with(h, |x| {
        let g = &mut x.g;
        match what {
            0 => g.god_mode = v != 0.0,
            1 => g.ai_enabled = v != 0.0,
            2 => g.infinite_ammo = v != 0.0,
            3 => g.player_invisible = v != 0.0,
            4 => { let pl = g.player; g.pl[pl].launch_held = None; }
            _ => return 0,
        }
        1
    })
}
// ---- a game kept
#[no_mangle]
pub extern "C" fn face_keep(h: u32, how: u32) -> u32 {
    let k = super::with(h, |x| {
        let block = block_of(x.g.sim);
        // (either way a kept game has no recap: a load is a run in which that death has not happened)
        let mut g = x.g.clone(); g.out.clear(); game::deathrecap::recap_left_behind(&mut g);
        if how == 0 { Kept::Own(g, block) } else { Kept::Bytes(game::io::export(&g), block) }
    });
    keep(k)
}
/// # Safety
/// the two pointers name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_keep_given(state: *const u8, n: usize, block: *const u8, bn: usize) -> u32 {
    super::hook();
    keep(Kept::Bytes(super::bytes(state, n).to_vec(), super::bytes(block, bn).to_vec()))
}
#[no_mangle]
pub extern "C" fn face_kept_state(k: u32) {
    let b = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_deref()) { Some(Kept::Own(g, _)) => game::io::export(g), Some(Kept::Bytes(s, _)) => s.clone(), None => Vec::new() });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_kept_block(k: u32) {
    let b = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_deref()) { Some(Kept::Own(_, b)) | Some(Kept::Bytes(_, b)) => b.clone(), None => Vec::new() });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_kept_drop(k: u32) {
    drop_kept(k);
}
#[no_mangle]
pub extern "C" fn face_kept_n() -> usize {
    KEPT.with(|v| v.borrow().iter().flatten().count())
}
/// kept `k` again, in `slot`
fn again(k: u32, slot: usize) -> Option<Box<game::game::Game>> {
    KEPT.with(|v| v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_deref()).map(|k| stood(k, slot)))
}
pub(super) fn stood(k: &Kept, slot: usize) -> Box<game::game::Game> {
    match k {
        Kept::Own(g, block) => {
            sim::state::put(slot, block);
            let mut g = g.clone();
            game::io::set_slot(&mut g, slot);
            // (the box world's copy in rust/sim stands outside the block and is the abandoned moment's: written whole again, below)
            g.col.mirror.put_backs = -1.0;
            g
        }
        Kept::Bytes(state, block) => game_run::start(state, block, slot).0,
    }
}
/// the game held is kept `k` again; false = there is no such game
fn back(x: &mut super::Held, k: u32) -> bool {
    let Some(mut g) = again(k, x.g.sim) else { return false };
    // (the serial counter never goes back in a game: what was dealt after the save is still held on the host's side, and a number means one thing)
    g.serial_next = g.serial_next.max(x.g.serial_next);
    hold(x, g);
    true
}
/// this game is the one held now
pub(super) fn hold(x: &mut super::Held, mut g: Box<game::game::Game>) {
    // (the box world's copy with it: it stands again, of this moment, before anybody asks)
    game::scene_collision::unstand(&mut g);
    game::scene_collision::stands(&mut g);
    x.g = g;
    x.cmd = None;
    x.cmds = None;
    x.after.clear();
    x.drawn.cut();
}
/// the save controller's shelf of a game, with what each stands from
pub(super) fn shelf_of(h: u32) -> Vec<(String, Shelved, Rc<Kept>)> {
    let of: Vec<(String, Shelved)> = SHELF.with(|s| s.borrow().iter().filter(|((g, _), _)| *g == h).map(|((_, key), e)| (key.clone(), e.clone())).collect());
    of.into_iter().filter_map(|(key, e)| KEPT.with(|v| v.borrow().get((e.k as usize).wrapping_sub(1)).cloned().flatten()).map(|k| (key, e, k))).collect()
}
/// … and the shelf is that again
pub(super) fn shelf_put(h: u32, of: &[(String, Shelved, Rc<Kept>)]) {
    shelf_off(h, |_| true);
    for (key, e, k) in of { shelve(h, key.clone(), Shelved { k: keep_rc(k.clone()), ..e.clone() }); }
}
#[no_mangle]
pub extern "C" fn face_back(h: u32, k: u32) -> u32 {
    if !super::with(h, |x| back(x, k)) { return 0; }
    crate::memo::forget(h);
    1
}

/// AFTER A STEP, for a host that keeps games: what the step asked of the save controller, and the controller's own turn (game::savestate::control). A game is
/// kept as its own value; one put back is whole again here, and nothing is drawn through it.
pub fn saves(h: u32, x: &mut super::Held, dt: f64) {
    use game::out_events::OutEvent as E;
    use game::savestate::control::{self, Do};
    let asked: Vec<E> = x.g.out.iter().filter(|e| matches!(e, E::CheckpointsKeep(_) | E::CheckpointsRequest(_))).cloned().collect();
    for e in asked { match e { E::CheckpointsKeep(e) => control::keep(&mut x.g, &e.label), E::CheckpointsRequest(e) => control::request(&mut x.g, &e.why, &e.label), _ => {} } }
    if core::mem::replace(&mut x.g.restart_cut, false) { x.drawn.cut(); }
    match control::tick(&mut x.g, dt) {
        Some(Do::Keep { key, label }) => {
            game_run::checkpoint(&mut x.g, &label);
            let mut g = x.g.clone();
            g.out.clear();
            let k = keep(Kept::Own(g, block_of(x.g.sim)));
            shelve(h, key, Shelved { k, at: x.g.saves.shelf.m.values().map(|e| e.info.saved_at).fold(0.0, f64::max), label, regret: false });
        }
        Some(Do::Back { key }) => {
            let was = control::abandoned(&x.g);
            let kept = SHELF.with(|s| s.borrow().get(&(h, key.clone())).map(|e| e.k));
            let told = if kept.is_some_and(|k| back(x, k)) { let told = control::put_back(&mut x.g, &key, was); crate::memo::forget(h); told } else { control::gone(&mut x.g, &key) };
            x.said.push(told);
        }
        Some(Do::Say(words)) => x.said.push(words),
        None => {}
    }
    // (a run's saves go with the run; its first step is kept, for a game that is lost before anything else is)
    shelf_off(h, |key| key != START && !x.g.saves.shelf.m.has(String::from(key)));
    let run = x.g.mission.serial;
    if SHELF.with(|s| s.borrow().get(&(h, String::from(START))).map(|e| e.at)) != Some(run as f64) {
        let mut g = x.g.clone();
        g.out.clear();
        let k = keep(Kept::Own(g, block_of(x.g.sim)));
        shelve(h, String::from(START), Shelved { k, at: run as f64, label: String::new(), regret: false });
    }
    SHELF.with(|s| for ((of, key), e) in s.borrow_mut().iter_mut() { if *of == h && !e.regret { if let Some(m) = x.g.saves.shelf.m.get(key.clone()) { e.regret = m.info.regret.is_some(); } } });
}
fn shelve(h: u32, key: String, e: Shelved) { if let Some(old) = SHELF.with(|s| s.borrow_mut().insert((h, key), e)) { drop_kept(old.k); } }
/// the key of a run's first step
const START: &str = "start";
/// what is kept of game `h`, the best to go back to first: the newest save nothing came down after, the newest at all, and the run's first step last
fn kept_of(h: u32) -> Vec<(String, Shelved)> {
    let mut of: Vec<(String, Shelved)> = SHELF.with(|s| s.borrow().iter().filter(|((g, _), _)| *g == h).map(|((_, key), e)| (key.clone(), e.clone())).collect());
    of.sort_by(|a, b| (a.0 == START).cmp(&(b.0 == START)).then(a.1.regret.cmp(&b.1.regret)).then(b.1.at.total_cmp(&a.1.at)));
    of
}
/// what a game lost to a panic would stand again from: 0 nothing, 1 its run's first step, 2 a save (its label at face_out)
#[no_mangle]
pub extern "C" fn face_recover_what(h: u32) -> u32 {
    match kept_of(h).first() { None => 0, Some((key, _)) if key == START => 1, Some((_, e)) => { super::out(e.label.clone().into_bytes()); 2 } }
}
/// A GAME LOST TO A PANIC STANDS AGAIN, from the best save of it (`newest` = 0: from its run's first step), under a new number; 0 = nothing of it is kept. What was
/// kept of the lost game is the new one's.
#[no_mangle]
pub extern "C" fn face_recover(h: u32, newest: u32) -> u32 {
    super::hook();
    let of = kept_of(h);
    let Some((key, e)) = of.iter().find(|(key, _)| newest != 0 || key == START).cloned() else { return 0 };
    let Some(slot) = KEPT.with(|v| match v.borrow().get((e.k as usize).wrapping_sub(1)).and_then(|x| x.as_deref()) { Some(Kept::Own(g, _)) => Some(g.sim), _ => None }) else { return 0 };
    // (a game that still stands under the lost number would share the slot: it is let go, and what is kept of it is not)
    super::GAMES.with(|v| if let Some(x) = v.borrow_mut().get_mut((h as usize).wrapping_sub(1)) { *x = None; });
    let n = face_stand(e.k, slot);
    if n == 0 { return 0; }
    SHELF.with(|s| { let mut s = s.borrow_mut(); for (key, _) in &of { if let Some(e) = s.remove(&(h, key.clone())) { s.insert((n, key.clone()), e); } } });
    super::with(n, |x| {
        // (the saves made after the one that stands are still kept: the shelf names them all)
        for (key, e) in of.iter().filter(|(key, _)| key != START) {
            let m = KEPT.with(|v| match v.borrow().get((e.k as usize).wrapping_sub(1)).and_then(|x| x.as_deref()) { Some(Kept::Own(g, _)) => g.saves.shelf.m.get(key.clone()).cloned(), _ => None });
            if let Some(mut m) = m { if e.regret && m.info.regret.is_none() { m.info.regret = Some(String::from("the floor came down straight after")); } x.g.saves.shelf.m.set(key.clone(), m); }
        }
        if key != START { let was = game::savestate::control::abandoned(&x.g); let told = game::savestate::control::put_back(&mut x.g, &key, was); x.said.push(told); }
        x.drawn.cut();
    });
    super::out(e.label.into_bytes());
    n
}
/// the games kept for game `h` that are let go
pub fn shelf_off(h: u32, go: impl Fn(&str) -> bool) {
    let gone: Vec<String> = SHELF.with(|s| s.borrow().keys().filter(|(g, key)| *g == h && go(key)).map(|(_, key)| key.clone()).collect());
    for key in gone { if let Some(e) = SHELF.with(|s| s.borrow_mut().remove(&(h, key))) { drop_kept(e.k); } }
}
fn drop_kept(k: u32) { KEPT.with(|v| if let Some(x) = v.borrow_mut().get_mut((k as usize).wrapping_sub(1)) { *x = None; }) }
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_commands(h: u32, p: *const u8, n: usize) -> u32 {
    let b = super::bytes(p, n);
    // (a restart done between two ticks: the picture of the game before it is not blended into this one's)
    match super::with(h, |x| { let done = game_run::command::apply(&mut x.g, b); if x.g.restart_cut { x.drawn.cut(); } done }) { Ok(()) => 1, Err(why) => { super::out(why.into_bytes()); 0 } }
}

thread_local! {
    /// the highest serial counter any export has shown the host
    static SERIAL_SEEN: core::cell::Cell<u32> = const { core::cell::Cell::new(0) };
}
/// an export is about to show the host this game: its serial counter is noted (what the host holds was dealt below it)
pub fn serial_seen(next: u32) { SERIAL_SEEN.with(|s| if next > s.get() { s.set(next) }); }

#[no_mangle]
pub extern "C" fn face_stand(k: u32, slot: usize) -> u32 {
    super::hook();
    if slot >= sim::state::GAMES { return 0; }
    let Some(mut g) = again(k, slot) else { return 0 };
    g.serial_next = g.serial_next.max(SERIAL_SEEN.with(|s| s.get()));
    game::scene_collision::unstand(&mut g);   // (whatever rows its copy says it wrote were another slot's, or another moment's)
    let h = super::put(super::Held::new(g));
    crate::memo::forget(h);
    h
}
#[no_mangle]
pub extern "C" fn face_debug_panic(h: u32, how: u32) {
    super::hook();
    if how != 0 { panic!("a planted panic, outside any game (face_debug_panic)"); }
    super::with(h, |x| {
        // (half a write, as a rule that panics leaves one: the clock moved and nothing else)
        x.g.time += 1.0 / 60.0;
        let none: Option<u32> = None;
        let at = x.g.time;
        if none.is_none() { panic!("a planted panic, inside the game at {at:.3} s (face_debug_panic)"); }
    });
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_saves_in(h: u32, p: *const u8, n: usize) {
    use game::savestate::{MemSave, SlotInfo, SlotInfoKind, SlotInfoSlot};
    let b = super::bytes(p, n);
    let mut at = 0usize;
    let mut u32_ = |at: &mut usize| { let v = u32::from_le_bytes([b[*at], b[*at + 1], b[*at + 2], b[*at + 3]]) as usize; *at += 4; v };
    let f64_ = |at: &mut usize| { let mut w = [0u8; 8]; w.copy_from_slice(&b[*at..*at + 8]); *at += 8; f64::from_le_bytes(w) };
    let mut text = |at: &mut usize| { let len = u32_(at); let s = String::from_utf8_lossy(&b[*at..*at + len]).into_owned(); *at += len; s };
    let run = text(&mut at);
    let rows = { let v = u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize; at += 4; v };
    let mut list: Vec<SlotInfo> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let key = text(&mut at);
        let manual = b[at] != 0; at += 1;
        let nn = f64_(&mut at);
        let label = text(&mut at);
        let saved_at = f64_(&mut at);
        let mission_t = f64_(&mut at);
        let stage = text(&mut at);
        let run_id = text(&mut at);
        let regret = { let has = b[at] != 0; at += 1; if has { Some(text(&mut at)) } else { None } };
        list.push(SlotInfo { key, slot: if manual { SlotInfoSlot::Manual } else { SlotInfoSlot::Auto }, n: nn, label, kind: if manual { SlotInfoKind::Manual } else { SlotInfoKind::Auto },
            saved_at, mission_t, stage, run_id, bytes: 0.0, regret, mem: Some(true) });
    }
    super::with(h, |x| {
        let s = &mut x.g.saves;
        s.run_id = run;
        s.shelf.m.clear();
        for info in list { s.shelf.m.set(info.key.clone(), MemSave { info, pic: None }); }
    });
}
#[no_mangle]
pub extern "C" fn face_saves_asked(h: u32) -> u32 {
    let (bits, key) = super::with(h, |x| {
        let g = &mut x.g;
        let due = g.saves.due.take();
        let cut = core::mem::replace(&mut g.restart_cut, false);
        ((due.is_some() as u32) | ((cut as u32) << 1), due.unwrap_or_default())
    });
    super::out(key.into_bytes());
    bits
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_loaded(h: u32, p: *const u8, n: usize, loads: f64, saves: f64) {
    let label = super::text(p, n);
    super::with(h, |x| {
        let g = &mut x.g;
        let pl = g.player;
        // (handring.ts: a ring that merely stood at the save is nothing of the saved moment)
        if g.pl[pl].sel.ring.as_ref().is_some_and(|r| r.free == Some(true)) {
            let s = &mut g.pl[pl].sel;
            s.open = false; s.ring = None; s.down_t = -1.0; s.moved = false;
            g.pl[pl].near_sel = 0;
        }
        g.pl[pl].setup = None;   // (slotsetup.ts: a gun's setup that was up at the save is not up after the load)
        game::orders::squad_orders_loaded(g);
        game::sense::smoke_clear(g);   // (a save keeps no smoke in the air: the picture's own is cleared with it, and nobody is to be hidden by smoke that is not drawn)
        g.chronicle.counts.loads = loads;
        g.chronicle.counts.saves = saves;
        let _ = game::chronicle::chron_checkpoint(g, game::chronicle::ChronCheckpointWhat::Load, Some(&label));
        game::rails::rails_put_back(g);
    });
}
