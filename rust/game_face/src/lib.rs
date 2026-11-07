//! THE FACE OF THE RUST GAME: one wasm module a host (bun, the page) loads, hands a start to, steps a tick at a time and reads back (tools/blitz/FACE.md).
//! Nothing here decides anything: the game is the crate `game`, what a host does round a step is the crate `game_run`'s library (the same lines the
//! native runner calls), and rust/sim is linked in, so ITS exports (`sim_abi`, `state_addr`, `pose_addr` … rust/sim/src/lib.rs) are this module's too, over
//! the same memory: there is one block of rust/sim's a game, the one the Rust game steps.
//!
//! WHY rust/sim's EXPORTS ARE HERE WITH NO LINE THAT RE-EXPORTS THEM: a crate's `#[no_mangle]` functions are exports of the final cdylib whenever that
//! crate is in the link at all, and a crate is in the link when some line of some linked crate names it (this file names `sim::state`, and so does the
//! crate `game`). A dependency that stands in Cargo.toml and is named by no line is not loaded, and its exports are silently absent.
//!
//! EVERY EXPORT, C ABI (a pointer and a length are `usize`, a handle `u32`, a flag `u32`):
//!
//! memory
//!   face_alloc(n) -> p · face_free(p, n)      bytes for the host to write into (a start, a tick's host half, a line)
//!   face_panic_ptr() · _len() · _clear()      what the last panic said, with its file and line (a `todo!()`, an index out of range): the module is built
//!                                             to abort on a panic, so the host sees a trap; it reads the words here. The game of that call is gone.
//!   face_out_ptr() · face_out_len()           the bytes or text the last call answered with (`face_export`, a log line): read before the next call
//!
//! a game (a handle; as many at once as rust/sim has slots, `state_games()`)
//!   face_new(state, n, block, bn) -> h        a game from a start: the bridge's bytes (`io::import`) and rust/sim's block as the start was taken, copied
//!                                             into the first free slot. 0 where every slot is taken.
//!   face_lesson(h, p, n)                      a tutorial mission's tests (tools/blitz lesson.json), which the bridge does not carry: once, after face_new
//!   face_blank() -> h                         a game as its field initialisers leave it (no level): proves the module stands
//!   face_drop(h)                              the game and its slot are free again
//!   face_slot(h) -> slot                      which of rust/sim's slots the game stands in (`state_addr(slot)` is its block)
//!   face_import_into(h, p, n)                 the bridge's bytes over a game that stands: what the walk does not carry stays as that game has it
//!
//! the host's half, before a step
//!   face_host(h, p, n, first)                 one tick's host half (tools/blitz h<k>.bin: the seats' record, the host's cameras and Input rows).
//!                                             `first` = 1 for the tick the start was taken at, whose record the start itself carries.
//!   face_give(h, k, p, n)                     a line of gives.txt (an order the test driver gave between ticks); the log's line is at face_out
//!   face_meter(h, v)                          the player's light meter as the host metered it (a tutorial mission's harness feeds it before every tick)
//!   face_hostwrite(h, k, p, n) -> 1 | 2 | 0   a line of hostwrites.txt: 1 done now (the driver's question that writes the game, a lesson's pick of a verb), 2 kept for the end of
//!                                             the next step (the autosave's row), 0 not known
//!
//! the step
//!   face_step(h, dt, flags)                   ONE TICK, and what tools/blitz/BRIDGE.md says a host calls round it, in its order:
//!                                             flag 1 `UPLOAD`: the box world listed again first (where the host uploaded since the last tick; the
//!                                             listing is also what frees the rows of the frames gone by) ·
//!                                             `game_step` with the record face_host took (or the game's own `cmd_buf` where none was taken) ·
//!                                             the autosave's rows face_hostwrite kept · flag 2 `LIGHTS`: the lights' update (a flash's envelope and
//!                                             its retirement, which the next tick reads) · flag 4 `VIEW`: the host's view of a level on rails · flag 8 `SAVES`: the
//!                                             save controller's turn (page.rs `saves`).
//!                                             A host that plays calls this one thing a tick with LIGHTS and VIEW up. A host that compares the block
//!                                             with the TypeScript's leaves them down, compares, and calls face_lights and face_view: the
//!                                             TypeScript's hashes are taken before its host's half.
//!   face_upload(h) · face_lights(h, dt) · face_view(h) · face_checkpoint(h, label, n)
//!                                             the same by name, for a host that calls them itself (face_checkpoint when it has saved)
//!
//! reading back
//!   face_export(h)                            the whole state (`io::export`) at face_out
//!   face_verdict(h) -> 0 | 1 | 2              how the mission ended, off the record the game's tick latches: 0 not yet, 1 lost, 2 won (then the record as lines at face_out)
//!   face_tick_view(h)                         what a HUD is told of the game as its last tick left it (a TickView at face_out); a read, nothing kept
//!   face_line(h, what, a…, b…) -> 0 | 1 · face_ray(h, o…, d…, max) -> 0 | 1 · face_hear(h, s…, d…)
//!                                             the box world asked of the game shared: the camera's, the sound's, the cursor's questions; nothing is kept
//!   face_die_n(h) -> f64 · face_time(h) -> f64 · face_rows(h) -> usize
//!                                             the die's count of draws (in the game's block), the game's clock, the rows of the box table
//!   face_parts() -> 13 · face_part_name(i)    the parts of rust/sim's block in their order; a part's name at face_out
//!   face_part(name, n) -> i                   a part by name (`poses`), or -1
//!   face_part_addr(slot, i) · face_part_len(i)   where part `i` of the block in `slot` is in this memory and how long: the host hashes or reads it in
//!                                             place (rust/sim's own `state_addr` / `state_len` / `state_part` say the same of the whole block)
//!   face_part_hashes(slot) -> p               13 words: each part's hash as tools/blitz/mission.ts takes it (its words up to the last not zero)
use std::cell::{Cell, RefCell};
mod memo;
mod page;
mod harness;

type Game = game::game::Game;
/// a game and what the host has handed over for its next step
struct Held {
    g: Box<Game>,
    /// the record face_host took for the next step
    cmd: Option<game::cmd::PlayerCmd>,
    /// every seat's record, as face_host_half took them for the next step (a host that plays: the bridge's own small hand-over)
    cmds: Option<Vec<game::cmd::PlayerCmd>>,
    /// the lines of hostwrites.txt that are written as a tick's last thing
    after: Vec<(usize, String)>,
    /// what the last tick left to be drawn (a host that draws)
    drawn: game_draw::Drawn,
}
impl Held { fn new(g: Box<Game>) -> Held { Held { g, cmd: None, cmds: None, after: Vec::new(), drawn: Default::default() } } }

thread_local! {
    static GAMES: RefCell<Vec<Option<Held>>> = const { RefCell::new(Vec::new()) };
    static PANIC: RefCell<String> = const { RefCell::new(String::new()) };
    static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    /// the ways round a sound's path found this tick, and for which game
    static WAYS: RefCell<(u32, game::ask::Ways)> = RefCell::new((0, Default::default()));
    static HASHES: Cell<[u32; 13]> = const { Cell::new([0; 13]) };
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
/// a call that may write what it does not say (a write by path, a state read in, a rule called from outside a tick, which may stage a level): the box world's copy is put
/// down first, and the next question or tick proves it from nothing
fn with_unsaid<R>(h: u32, f: impl FnOnce(&mut Held) -> R) -> R { with(h, |x| { game::scene_collision::unstand(&mut x.g); f(x) }) }
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
/// THE STATE WORDS (game::state_words): a hash of the whole game a part, with what a host hands in left out, for two machines of one networked game to compare. `face_state_n` parts, `face_state_name(i)` at face_out; `face_state_words(h)` puts the words at face_out, u32 little-endian. Its buffers stay here, so a call grows nothing once they have their size.
#[no_mangle]
pub extern "C" fn face_state_n() -> usize { game::state_words::STATE_NAMES.len() }
#[no_mangle]
pub extern "C" fn face_state_name(i: usize) { out(game::state_words::STATE_NAMES[i].as_bytes().to_vec()); }
thread_local! { static STATE_BUF: RefCell<(game::io::W, Vec<u32>)> = RefCell::new((game::io::W::new(), Vec::new())); }
#[no_mangle]
pub extern "C" fn face_state_words(h: u32) {
    let b = STATE_BUF.with(|c| { let mut c = c.borrow_mut(); let (o, w) = &mut *c; with(h, |x| game::state_words::state_words(&mut x.g, o, w)); let mut b = Vec::with_capacity(w.len() * 4); for v in w.iter() { b.extend_from_slice(&v.to_le_bytes()); } b });
    out(b);
}
/// THE MISSION'S VERDICT, for a host with no puppet (a server): 0 while the mission is being played, 1 lost, 2 won; when not 0 the latched record is
/// at face_out as lines (game::verdict::verdict_lines: the end's word, the why, the seconds, the way won, the counts, the men). The game's own tick
/// latches it: the first end of a run stays. While it is 0 nothing is allocated and face_out is left as it was.
#[no_mangle]
pub extern "C" fn face_verdict(h: u32) -> i32 {
    match with_ref(h, |x| game::verdict::verdict_lines(&x.g)) {
        None => 0,
        Some((won, s)) => { out(s.into_bytes()); if won { 2 } else { 1 } }
    }
}
/// WHAT A HUD IS TOLD of the game as its last tick left it (`view::tick_view`), about the seat this machine shows: a `TickView`'s bytes at face_out, read
/// by the host with the table's own reader. A read: nothing of the game is written, and no seat is changed over.
#[no_mangle]
pub extern "C" fn face_tick_view(h: u32) {
    use game::io::Io;
    out(with_ref(h, |x| { let g = &x.g; let mut o = game::io::W::new(); game::view::tick_view(g, &game::seats::seat(g, g.local_seat.unwrap_or(g.seat))).w(&mut o); o.b }));
}
// ---- the box world, asked of the game shared (game::ask::WorldAsk): the host's camera, its sound, its cursor. The copy stands from a tick's end; where a writer put it
// down the first question stands it again, and no question proves it for itself
fn asked<R>(h: u32, f: impl FnOnce(game::ask::WorldAsk) -> R) -> R {
    with(h, |x| {
        if game::ask::WorldAsk::try_of(&x.g).is_err() { game::scene_collision::stands(&mut x.g); }
        let g: &Game = &x.g;
        f(game::ask::WorldAsk::of(g))
    })
}
/// is the line a→b stopped: `what` 0 = to an eye (glass is passed), 1 = by anything solid
#[no_mangle]
pub extern "C" fn face_line(h: u32, what: u32, ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> u32 {
    asked(h, |w| w.line([ax, ay, az], [bx, by, bz], if what == 0 { game::ask::Line::Sight } else { game::ask::Line::Solid })) as u32
}
/// the nearest thing along a ray, within `max` (d a unit vector): 0 nothing · 1, and at face_out five doubles: how far, the face's normal, which (a static's place, 1e6 +
/// a leaf's)
#[no_mangle]
pub extern "C" fn face_ray(h: u32, ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64, max: f64) -> u32 {
    let Some(r) = asked(h, |w| w.ray([ox, oy, oz], [dx, dy, dz], max)) else { return 0 };
    out([r.t, r.n[0], r.n[1], r.n[2], r.index].iter().flat_map(|v| v.to_le_bytes()).collect());
    1
}
/// how a sound at `s` reaches a listener at `d`: at face_out ten doubles: how far it comes, through where (x y z), from where it seems to come (x y z), how muffled (0 1),
/// how much of it carries, and 2 direct + 1 routed. A way round is searched for once a tick between the same two places of the floor; the game remembers none of it.
#[no_mangle]
pub extern "C" fn face_hear(h: u32, sx: f64, sy: f64, sz: f64, dx: f64, dy: f64, dz: f64) {
    let p = WAYS.with(|k| { let mut k = k.borrow_mut(); if k.0 != h { *k = (h, Default::default()); } asked(h, |w| w.hear_kept(&mut k.1, [sx, sy, sz], [dx, dy, dz])) });
    let how = if p.direct { 2.0 } else { 0.0 } + if p.routed { 1.0 } else { 0.0 };
    out([p.path_len, p.via[0], p.via[1], p.via[2], p.apparent[0], p.apparent[1], p.apparent[2], p.muffle, p.carry, how].iter().flat_map(|v| v.to_le_bytes()).collect());
}
/// THE GENERATION OF THE BRIDGE this module was built with (io::LAYOUT, and `layout` in schema.json): a host refuses a module of another generation.
#[no_mangle]
pub extern "C" fn face_layout() -> u32 { game::io::LAYOUT }
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
/// # Safety
/// the two pointers name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_new(state: *const u8, n: usize, block: *const u8, bn: usize) -> u32 {
    hook();
    let Some(slot) = free_slot() else { return 0 };
    let (g, said) = game_run::start(bytes(state, n), bytes(block, bn), slot);
    out(said.join("\n").into_bytes());
    put(Held::new(g))
}
#[no_mangle]
pub extern "C" fn face_blank() -> u32 {
    hook();
    let Some(slot) = free_slot() else { return 0 };
    let mut g = Box::new(Game::blank());
    g.sim = slot;
    // (a block with nothing in it is zeros under the layout's word: whatever game stood in the slot before is gone)
    unsafe { core::ptr::write_bytes(sim::state::addr(slot) as *mut u8, 0, core::mem::size_of::<sim::state::State>()); }
    sim::state::state(slot).head.layout = sim::state::LAYOUT;
    put(Held::new(g))
}
#[no_mangle]
pub extern "C" fn face_drop(h: u32) {
    page::shelf_off(h, |_| true);
    GAMES.with(|v| if let Some(x) = v.borrow_mut().get_mut((h as usize).wrapping_sub(1)) { *x = None; });
}
#[no_mangle]
pub extern "C" fn face_slot(h: u32) -> usize {
    with(h, |x| x.g.sim)
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_import_into(h: u32, p: *const u8, n: usize) {
    let b = bytes(p, n);
    with_unsaid(h, |x| game::io::import_into(&mut x.g, b));
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_lesson(h: u32, p: *const u8, n: usize) {
    let b = bytes(p, n);
    let said = with(h, |x| game_run::lesson(&mut x.g, b));
    out(said.unwrap_or_default().into_bytes());
}

// ---- the host's half
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_host(h: u32, p: *const u8, n: usize, first: u32) {
    let b = bytes(p, n);
    with(h, |x| x.cmd = Some(game_run::host_half(&mut x.g, b, first != 0)));
}
/// THE HOST'S HAND-OVER OF A HOST THAT PLAYS (export.ts `hostHalf`): each seat's record, then every camera and Input row. The rows that are no seat's
/// are taken now; the records are kept for the next `face_step`, which then steps every seat (`game_step_seats`).
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_host_half(h: u32, p: *const u8, n: usize) {
    let b = bytes(p, n);
    with(h, |x| x.cmds = Some(game::io::take_host_half(&mut x.g, b)));
}
#[no_mangle]
pub extern "C" fn face_debug(h: u32) {
    let t = with(h, |x| format!("{:#?}", x.g));
    out(t.into_bytes());
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
/// A tap (tools/blitz/taps.json): where set, the function reports its call as an out event
#[no_mangle]
pub extern "C" fn face_tap(h: u32, id: u32, on: u32) {
    if id >= 64 { return; }   // (no such tap: a shift past the word's width is a panic in a build that checks it)
    with(h, |x| { if on != 0 { x.g.taps |= 1u64 << id; } else { x.g.taps &= !(1u64 << id); } });
}
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
extern "C" { fn face_roll() -> f64; }
#[cfg(target_arch = "wasm32")]
fn roll_of_the_host() -> f64 { unsafe { face_roll() } }
#[cfg(not(target_arch = "wasm32"))]
fn roll_of_the_host() -> f64 { 0.0 }
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_call(h: u32, f: u32, view: i32, p: *const u8, n: usize) {
    game::call_gen::HOST_ROLL = Some(roll_of_the_host);
    let b = bytes(p, n);
    // (a caller that hands in a seat's view is asking about that seat: the call runs as that seat, whoever stands on the game)
    let a = with_unsaid(h, |x| if view < 0 { game::call_gen::call(&mut x.g, f, b) } else { game::seats::with_seat(&mut x.g, <game::ids::SeatId as game::ids::Id>::from_ix(view as usize), &mut |g| game::call_gen::call(g, f, b)) });
    out(a);
}
/// which seat this machine shows, by its place in the game's list of seats: the host says so before an apply and before its half of a tick, whenever it may have changed
#[no_mangle]
pub unsafe extern "C" fn face_show_seat(h: u32, seat: u32) {
    with(h, |x| { let s = x.g.seats[seat as usize]; game::seats::show_seat(&mut x.g, s); });
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_write(h: u32, p: *const u8, n: usize) {
    let b = bytes(p, n);
    with_unsaid(h, |x| game::io::poke(&mut x.g, b));
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
/// Answers 0 and the answer's bytes; or, with no bytes and nothing asked: 1 = the game has no such order row, 2 = no such seat (or one that keeps no
/// set of its own to stand on), 3 = no such kind of question. The page can hold a row this game has not got (a row made on the page since the last start,
/// an ID from before a restart): an index past a table's end is a panic, and a panic is the game lost, so the numbers are measured here and never taken.
pub unsafe extern "C" fn face_ask(h: u32, kind: u32, row: u32, seat: u32, p: *const u8, n: usize) -> u32 {
    use game::ids::Id;
    let b = bytes(p, n);
    let (r, a) = with(h, |x| {
        let g = &mut x.g;
        if kind > 10 { return (3, Vec::new()); }
        let s = seat as usize;
        if s >= g.tab.cmd_seat.len() { return (2, Vec::new()); }
        let sid = <game::ids::SeatId as Id>::from_ix(s);
        if g.seat != sid && g.tab.cmd_seat[sid].own.is_none() { return (2, Vec::new()); }
        // (the posture's word and an order by its id string name no row)
        if kind != 6 && kind != 10 && (row as usize) >= g.tab.orders_order_def.len() { return (1, Vec::new()); }
        (0, game::io::ask(g, kind, row, seat, b))
    });
    out(a);
    r
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_give(h: u32, k: usize, p: *const u8, n: usize) {
    let l = text(p, n);
    let said = with(h, |x| game_run::give(&mut x.g, k, &l));
    out(said.into_bytes());
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_hostwrite(h: u32, k: usize, p: *const u8, n: usize) -> u32 {
    let l = text(p, n);
    let (r, said) = with_unsaid(h, |x| match game_run::write_before(&mut x.g, k, &l) {
        Some(t) => (1, t),
        // (the only write made after a tick is the autosave's; a line that is neither is the host's to report)
        None if l.split(' ').nth(1) == Some("chron_save") => { x.after.push((k, l.to_string())); (2, String::new()) }
        None => (0, String::new()),
    });
    out(said.into_bytes());
    r
}

#[no_mangle]
pub extern "C" fn face_meter(h: u32, v: f64) {
    with(h, |x| game_run::meter(&mut x.g, v));
}

// ---- the step
pub const UPLOAD: u32 = 1;
pub const LIGHTS: u32 = 2;
pub const VIEW: u32 = 4;
pub const SAVES: u32 = 8;
#[no_mangle]
pub extern "C" fn face_step(h: u32, dt: f64, flags: u32) {
    let said = with(h, |x| {
        if flags & UPLOAD != 0 { game_run::upload(&mut x.g); }
        if let Some(cmds) = x.cmds.take() { game::game::game_step_seats(&mut x.g, dt, &cmds); } else {
            let cmd = x.cmd.take().or_else(|| x.g.cmd_buf.clone()).expect("face_step: no record of the tick's input (face_host or face_host_half hands one over, or the state carries one in Game.cmd_buf)");
            game_run::step(&mut x.g, dt, &cmd);
        }
        let mut said = String::new();
        for (k, l) in core::mem::take(&mut x.after) { if let Some(t) = game_run::write_after(&mut x.g, k, &l) { said.push_str(&t); said.push('\n'); } }
        if flags & LIGHTS != 0 { game_run::lights(&mut x.g, dt); }
        if flags & VIEW != 0 { game_run::view(&mut x.g); }
        if flags & SAVES != 0 { page::saves(h, x, dt); }
        said
    });
    out(said.into_bytes());
}
#[no_mangle]
pub extern "C" fn face_upload(h: u32) {
    with(h, |x| game_run::upload(&mut x.g));
}
#[no_mangle]
pub extern "C" fn face_view(h: u32) {
    with(h, |x| game_run::view(&mut x.g));
}
#[no_mangle]
pub extern "C" fn face_lights(h: u32, dt: f64) {
    with(h, |x| game_run::lights(&mut x.g, dt));
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_checkpoint(h: u32, p: *const u8, n: usize) {
    let l = text(p, n);
    with(h, |x| { game_run::checkpoint(&mut x.g, &l); });
}

// ---- reading back
#[no_mangle]
pub extern "C" fn face_export(h: u32) {
    let b = with(h, |x| { page::serial_seen(x.g.serial_next); game::io::export(&x.g) });
    out(b);
}
/// how many out events the game holds (`g.out`: what the rules of the steps since the last clear told the host)
#[no_mangle]
pub extern "C" fn face_out_n(h: u32) -> usize {
    with(h, |x| x.g.out.len())
}
/// the host has taken (or does not want) the out events: the list is a step's output and no part of the state, so nothing else empties it
#[no_mangle]
pub extern "C" fn face_out_clear(h: u32) {
    with(h, |x| x.g.out.clear());
}
#[no_mangle]
pub extern "C" fn face_die_n(h: u32) -> f64 {
    with(h, |x| sim::state::state(x.g.sim).dice.n)
}
#[no_mangle]
pub extern "C" fn face_time(h: u32) -> f64 {
    with(h, |x| x.g.time)
}
#[no_mangle]
pub extern "C" fn face_rows(h: u32) -> usize {
    with(h, |x| x.g.tab.boxes.len() + x.g.tab.frame_boxes.len())
}
#[no_mangle]
pub extern "C" fn face_parts() -> usize {
    game_run::NAMES.len()
}
#[no_mangle]
pub extern "C" fn face_part_name(i: usize) {
    out(game_run::NAMES.get(i).copied().unwrap_or("").as_bytes().to_vec());
}
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_part(p: *const u8, n: usize) -> i32 {
    let name = text(p, n);
    game_run::NAMES.iter().position(|x| *x == name).map(|i| i as i32).unwrap_or(-1)
}
#[no_mangle]
pub extern "C" fn face_part_addr(slot: usize, i: usize) -> usize {
    if slot >= sim::state::GAMES || i >= sim::state::PARTS.len() { return 0; }
    sim::state::addr(slot) + sim::state::PARTS[i].off
}
#[no_mangle]
pub extern "C" fn face_part_len(i: usize) -> usize {
    sim::state::PARTS.get(i).map(|p| p.len).unwrap_or(0)
}
#[no_mangle]
pub extern "C" fn face_part_hashes(slot: usize) -> *const u32 {
    HASHES.with(|c| { c.set(game_run::part_hashes(slot)); c.as_ptr() as *const u32 })
}
