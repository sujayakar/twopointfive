//! THE FACE OF THE GAME: one wasm module a host (the page, the server, a tool) loads, hands a start to, steps a tick at a time, draws and asks. Nothing here decides anything: the game is the
//! crate `game`, what a host does round a step is `game_start::host`, what is drawn is `game_draw`.
//!
//! C ABI: a pointer and a length are `usize`, a game's number `u32`, a flag `u32`. Each file says its own exports.
//!
//! memory
//!   face_alloc(n) -> p · face_free(p, n)      bytes for the host to write into
//!   face_out_ptr() · face_out_len()           the bytes or text the last call answered with ("at face_out"): read before the next call
//!   face_reserve(n) · face_mem() -> bytes     room asked for ahead · the memory now
//!   face_panic_ptr() · _len() · _clear()      what the last panic said, with its file and line. The module is built to abort on a panic, so the host sees a trap; it reads the words here. The
//!                                             game of that call is gone (`with` takes it out for the call, so no borrow is left behind), and every game kept is whole (saves.rs, moment.rs)
//!
//! a game (as many at once as rust/sim has slots)
//!   face_start(p, n, slot, seed, seats, flags) -> h   a game from a level file's text, the address's words, the character's files and the host's commands (game_start `handed`)
//!   face_drop(h) · face_slot(h) -> slot       the game and its slot are free again · which of rust/sim's slots it stands in
//!   face_show_seat(h, seat)                   which seat this machine shows
//!   face_step(h, dt, flags)                   ONE TICK of every seat on what was handed (hand.rs, net.rs). Flags: 8 SAVES, the save controller's turn
//!   face_out_clear(h)                         the host has taken what the tick told it
//!
//! two games held to each other
//!   face_state_n() · face_state_name(i) · face_state_words(h)   the state's words (game::state_words): a hash a part of the whole game, u32 little-endian at face_out
//!   face_die_n(h) · face_parts() · face_part_name(i) · face_part_hashes(slot) -> p   the die's count of draws · the parts of rust/sim's block, and each one's hash
use std::cell::{Cell, RefCell};
mod ask;
mod checks;
mod desk;
mod draw;
mod end;
mod hand;
mod map;
mod moment;
mod net;
mod saves;
mod view;

type Game = game::game::Game;
/// a game and what the host has handed over for its next step
struct Held {
    g: Box<Game>,
    /// every seat's record for the next step
    cmds: Option<Vec<game::cmd::PlayerCmd>>,
    /// what the last tick left to be drawn (a host that draws)
    drawn: game_draw::Drawn,
    /// the seat nobody's hands are on, and who plays it
    hands: Option<(usize, game_start::hands_off::HandsOff)>,
    /// what the save controller has said to the player, until the host asks
    said: Vec<String>,
}
impl Held {
    fn new(g: Box<Game>) -> Held { Held { g, cmds: None, drawn: Default::default(), hands: None, said: Vec::new() } }
    /// a seat's record for the next step; a seat nobody hands one is as it last was, with nothing pressed
    fn hand_over(&mut self, seat: usize, cmd: game::cmd::PlayerCmd) {
        let still = |s: game::ids::SeatId| { let c = self.g.tab.cmd_seat[s].cmd.as_ref().unwrap_or(&cmd); game::cmd::PlayerCmd { keys: Vec::new(), pressed: Vec::new(), buttons: 0.0, clicked: 0.0, wheel: 0.0, ..c.clone() } };
        let mut cmds = self.cmds.take().unwrap_or_else(|| self.g.seats.iter().map(|&s| still(s)).collect());
        cmds[seat] = cmd;
        self.cmds = Some(cmds);
    }
}

thread_local! {
    static GAMES: RefCell<Vec<Option<Held>>> = const { RefCell::new(Vec::new()) };
    static PANIC: RefCell<String> = const { RefCell::new(String::new()) };
    static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static HASHES: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
    static HOOKED: Cell<bool> = const { Cell::new(false) };
}

/// the panic's words are kept before the abort: a trap alone says nothing of which `todo!()` or which index
fn hook() {
    if HOOKED.with(|h| h.replace(true)) { return; }
    std::panic::set_hook(Box::new(|info| {
        let s = info.to_string();
        PANIC.with(|p| *p.borrow_mut() = s);
    }));
}
fn put(h: Held) -> u32 {
    GAMES.with(|v| {
        let mut v = v.borrow_mut();
        match v.iter().position(|x| x.is_none()) { Some(i) => { v[i] = Some(h); i as u32 + 1 } None => { v.push(Some(h)); v.len() as u32 } }
    })
}
fn with<R>(h: u32, f: impl FnOnce(&mut Held) -> R) -> R {
    // the game is taken out for the call, so a panic inside leaves no borrow behind (and no game: its state is whatever the panic left)
    let mut g = GAMES.with(|v| v.borrow_mut().get_mut((h as usize).wrapping_sub(1)).and_then(|x| x.take())).expect("no such game (dropped, or lost to a panic)");
    let r = f(&mut g);
    GAMES.with(|v| v.borrow_mut()[h as usize - 1] = Some(g));
    r
}
/// the game of a number that was lost to a panic is `f`'s again
fn stand_if_lost(h: u32, f: impl FnOnce() -> Box<Game>) {
    if GAMES.with(|v| v.borrow().get((h as usize).wrapping_sub(1)).is_some_and(|x| x.is_none())) { let g = f(); GAMES.with(|v| v.borrow_mut()[h as usize - 1] = Some(Held::new(g))); }
}
/// a call that only reads the game: what it is handed is shared, so the compiler holds it to that
fn with_ref<R>(h: u32, f: impl FnOnce(&Held) -> R) -> R { with(h, |x| f(x)) }
fn out(b: Vec<u8>) { OUT.with(|o| *o.borrow_mut() = b); }
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
unsafe fn bytes<'a>(p: *const u8, n: usize) -> &'a [u8] { if n == 0 { &[] } else { std::slice::from_raw_parts(p, n) } }
/// # Safety
/// as `bytes`
unsafe fn text<'a>(p: *const u8, n: usize) -> std::borrow::Cow<'a, str> { String::from_utf8_lossy(bytes(p, n)) }
/// the slots of rust/sim's that no live game stands in
fn free_slot() -> Option<usize> {
    GAMES.with(|v| { let v = v.borrow(); (0..sim::state::GAMES).find(|s| !v.iter().flatten().any(|h| h.g.sim == *s)) })
}

// ---- memory
#[no_mangle]
pub extern "C" fn face_alloc(n: usize) -> *mut u8 {
    hook();
    let mut v = Vec::<u8>::with_capacity(n.max(1));
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}
/// # Safety
/// `p` and `n` are what face_alloc gave.
#[no_mangle]
pub unsafe extern "C" fn face_free(p: *mut u8, n: usize) {
    drop(Vec::from_raw_parts(p, 0, n.max(1)));
}
/// room asked for and given back, so that no step has to grow the memory to find it
#[no_mangle]
pub extern "C" fn face_reserve(n: usize) { drop(Vec::<u8>::with_capacity(n.max(1))); }
/// the module's memory now, in bytes (a host holds its typed arrays to this: a memory that grew has detached them)
#[no_mangle]
pub extern "C" fn face_mem() -> usize {
    core::arch::wasm32::memory_size(0) * 65536
}
#[no_mangle]
pub extern "C" fn face_panic_clear() {
    PANIC.with(|p| p.borrow_mut().clear());
}
#[no_mangle]
pub extern "C" fn face_panic_ptr() -> *const u8 {
    PANIC.with(|p| p.borrow().as_ptr())
}
#[no_mangle]
pub extern "C" fn face_panic_len() -> usize {
    PANIC.with(|p| p.borrow().len())
}
#[no_mangle]
pub extern "C" fn face_out_ptr() -> *const u8 {
    OUT.with(|o| o.borrow().as_ptr())
}
#[no_mangle]
pub extern "C" fn face_out_len() -> usize {
    OUT.with(|o| o.borrow().len())
}

// ---- a game
/// A GAME STARTED HERE, by the game's own start (rust/game_start `stand`), from what the host hands in one run of bytes (`handed.rs`: the level
/// file's text, the address's words, the character's files). No state crosses.
/// `slot`: the slot of rust/sim to stand in, whose block is made anew; below 0 = a free one. `flags`: 1 = the rows that dress the picture are
/// up (a page). Answers the game, or 0 where no slot is free.
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_start(p: *const u8, n: usize, slot: i32, seed: u32, seats: u32, flags: u32) -> u32 {
    hook();
    let slot = if slot < 0 { match free_slot() { Some(s) => s, None => return 0 } } else { slot as usize };
    if slot >= sim::state::GAMES { return 0; }
    let o = game_start::stand::StartOpts { seed, seats, slot, dress: flags & 1 != 0, ..Default::default() };
    let stood = game_start::handed::stand_handed(bytes(p, n), &o, &mut |g, words, b| if let Err(why) = game_start::host::words::at_start(g, words, b) { panic!("face_start: {why}") });
    put(Held::new(stood.game))
}
#[no_mangle]
pub extern "C" fn face_drop(h: u32) {
    saves::shelf_off(h);
    moment::off(h);
    GAMES.with(|v| if let Some(x) = v.borrow_mut().get_mut((h as usize).wrapping_sub(1)) { *x = None; });
}
#[no_mangle]
pub extern "C" fn face_slot(h: u32) -> usize {
    with(h, |x| x.g.sim)
}
/// by its place in the game's list of seats: the host says so whenever it changes
#[no_mangle]
pub extern "C" fn face_show_seat(h: u32, seat: u32) {
    with(h, |x| { let s = x.g.seats[seat as usize]; game::seats::show_seat(&mut x.g, s); });
}
pub const SAVES: u32 = 8;
#[no_mangle]
pub extern "C" fn face_step(h: u32, dt: f64, flags: u32) {
    // (asked inside and refused outside the call that holds the game, so that a tick with no record loses the call and not the game)
    let Some(cmds) = with(h, |x| x.cmds.take()) else { panic!("face_step: nobody handed the tick a record (face_hand, face_record)") };
    with(h, |x| {
        game::game::game_step_seats(&mut x.g, dt, &cmds);
        saves::dealt(&x.g);
        if flags & SAVES != 0 { saves::saves(h, x, dt); } else { saves::restart_turn(h, x); }
    });
}
/// the host has taken (or does not want) the out events: the list is a step's output and no part of the state, so nothing else empties it
#[no_mangle]
pub extern "C" fn face_out_clear(h: u32) {
    with(h, |x| x.g.out.clear());
}

// ---- two games held to each other
#[no_mangle]
pub extern "C" fn face_state_n() -> usize { STATE_NAMES.with(|n| n.len()) }
#[no_mangle]
pub extern "C" fn face_state_name(i: usize) { out(STATE_NAMES.with(|n| n[i].clone().into_bytes())); }
thread_local! { static STATE_NAMES: Vec<String> = game::state_words::state_names(); static STATE_BUF: RefCell<(game::bytes::W, Vec<u32>)> = RefCell::new((game::bytes::W::new(), Vec::new())); }
#[no_mangle]
pub extern "C" fn face_state_words(h: u32) {
    let b = STATE_BUF.with(|c| { let mut c = c.borrow_mut(); let (o, w) = &mut *c; with(h, |x| game::state_words::state_words(&mut x.g, o, w)); let mut b = Vec::with_capacity(w.len() * 4); for v in w.iter() { b.extend_from_slice(&v.to_le_bytes()); } b });
    out(b);
}
#[no_mangle]
pub extern "C" fn face_die_n(h: u32) -> f64 {
    with(h, |x| sim::state::state(x.g.sim).dice.n)
}
#[no_mangle]
pub extern "C" fn face_parts() -> usize {
    game_start::block::PART_NAMES.len()
}
#[no_mangle]
pub extern "C" fn face_part_name(i: usize) {
    out(game_start::block::PART_NAMES.get(i).copied().unwrap_or("").as_bytes().to_vec());
}
#[no_mangle]
pub extern "C" fn face_part_hashes(slot: usize) -> *const u32 {
    HASHES.with(|c| { let mut c = c.borrow_mut(); *c = game_start::block::part_hashes(slot); c.as_ptr() })
}
