//! THE FACE OF THE GAME: one wasm module a host (the page, the server, a tool) loads, hands a start to, steps a tick at a time, draws and asks. Nothing here decides anything: the game is the
//! crate `game`, what a host does round a step is `game_start::host`, what is drawn is `game_draw`.
//!
//! THE CALLS ARE DECLARED IN `game_start::host::calls`, and what each is and takes is said there. Here is a function of each name (in the files below), and `shims!` makes the exports: `face_step(h, p, n)` reads
//! the arguments, holds the game the way its line says, and answers. Beside them, for the bytes themselves (src/draw/wire.ts):
//!
//!   face_alloc(n) -> p · face_free(p, n)      bytes for the host to write into, at a multiple of eight
//!   face_out_ptr() · face_out_len()           the bytes the last call that tells anything told: read before the next
//!   face_panic_ptr() · _len() · _clear()      what the last panic said, with its file and line. The module is built to abort on a panic, so the host sees a trap; it reads the words here. The
//!                                             game of that call is gone (`with` takes it out for the call, so no borrow is left behind), and every game kept is whole (saves.rs, moment.rs)
use game::bytes::W;
use game::crosses::Answer;
use game::view::{ActionRow, AheadView, CursorView, DebriefView, EndView, FloorView, HandsSaid, KnobRow, MapView, Met, OutcomeView, Recovered, RunView, SavesTold, SavesView, ShownRow, SoundPath, Tally, TickView};
use game_start::host::calls::{GameNo, MomentNo};
use game_start::host::field::{Bytes, Field};
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
mod stand;
mod view;

type Game = game::game::Game;
/// a game and what the host has handed over for its next step
struct Held {
    /// its number in the module
    no: u32,
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
    fn new(g: Box<Game>) -> Held { Held { no: 0, g, cmds: None, drawn: Default::default(), hands: None, said: Vec::new() } }
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
    static OUT: RefCell<W> = const { RefCell::new(W::new()) };
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
fn put(mut h: Held) -> u32 {
    GAMES.with(|v| {
        let mut v = v.borrow_mut();
        let i = v.iter().position(|x| x.is_none()).unwrap_or_else(|| { v.push(None); v.len() - 1 });
        h.no = i as u32 + 1;
        v[i] = Some(h);
        i as u32 + 1
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
    if GAMES.with(|v| v.borrow().get((h as usize).wrapping_sub(1)).is_some_and(|x| x.is_none())) { let g = f(); GAMES.with(|v| v.borrow_mut()[h as usize - 1] = Some(Held { no: h, ..Held::new(g) })); }
}
/// a call that only reads the game: what it is handed is shared, so the compiler holds it to that
fn with_ref<R>(h: u32, f: impl FnOnce(&Held) -> R) -> R { with(h, |x| f(x)) }
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
unsafe fn bytes<'a>(p: *const u8, n: usize) -> &'a [u8] { if n == 0 { &[] } else { std::slice::from_raw_parts(p, n) } }
/// the slots of rust/sim's that no live game stands in
fn free_slot() -> Option<usize> {
    GAMES.with(|v| { let v = v.borrow(); (0..sim::state::GAMES).find(|s| !v.iter().flatten().any(|h| h.g.sim == *s)) })
}

// ---- the bytes
#[no_mangle]
pub extern "C" fn face_alloc(n: usize) -> *mut u8 {
    hook();
    unsafe { std::alloc::alloc(room(n)) }
}
/// # Safety
/// `p` and `n` are what face_alloc gave.
#[no_mangle]
pub unsafe extern "C" fn face_free(p: *mut u8, n: usize) { std::alloc::dealloc(p, room(n)); }
fn room(n: usize) -> std::alloc::Layout { std::alloc::Layout::from_size_align(n.max(1), 8).expect("more bytes than there are") }
#[no_mangle]
pub extern "C" fn face_panic_clear() { PANIC.with(|p| p.borrow_mut().clear()); }
#[no_mangle]
pub extern "C" fn face_panic_ptr() -> *const u8 { PANIC.with(|p| p.borrow().as_ptr()) }
#[no_mangle]
pub extern "C" fn face_panic_len() -> usize { PANIC.with(|p| p.borrow().len()) }
#[no_mangle]
pub extern "C" fn face_out_ptr() -> *const u8 { OUT.with(|o| o.borrow().b.as_ptr()) }
#[no_mangle]
pub extern "C" fn face_out_len() -> usize { OUT.with(|o| o.borrow().b.len()) }

// ---- the calls
/// a function of every call's name
mod made { pub use crate::{ask::*, checks::*, desk::*, draw::*, end::*, hand::*, map::*, moment::*, net::*, saves::*, stand::*, view::*}; }
/// a number, as it goes back
trait Number { fn back(self) -> f64; }
macro_rules! numbers { ($($t:ident)*) => { $(impl Number for $t { fn back(self) -> f64 { self as f64 } })* } }
numbers!(u32 i32 usize f64);
impl Number for bool { fn back(self) -> f64 { self as u32 as f64 } }
/// what a call is told lies here until the next one tells anything
fn tell<T: Answer>(a: &T) -> f64 { OUT.with(|o| { let mut o = o.borrow_mut(); o.b.clear(); a.told(&mut o).back() }) }

macro_rules! shims {
    (@held free $h:ident $name:ident $($a:ident)*) => { made::$name($($a),*) };
    (@held reads $h:ident $name:ident $($a:ident)*) => { with_ref($h, |x| made::$name(x, $($a),*)) };
    (@held does $h:ident $name:ident $($a:ident)*) => { with($h, |x| made::$name(x, $($a),*)) };
    (@back $do:expr) => { { let _: () = $do; 0.0 } };
    (@back $do:expr, -> $n:ty) => { { let n: $n = $do; n.back() } };
    (@back $do:expr, => $t:ty) => { { let a: $t = $do; tell(&a) } };
    ($($(#[doc = $d:expr])* $how:ident $name:ident($($a:ident: $t:ty),*) $(-> $n:ty)? $(=> $told:ty)?;)*) => { $(
        const _: () = {
            /// # Safety
            /// `p`, `n` name bytes the host wrote into memory from face_alloc.
            #[export_name = concat!("face_", stringify!($name))]
            unsafe extern "C" fn call(h: u32, p: *const u8, n: usize) -> f64 {
                let _ = h;
                let mut r = Bytes::of(bytes(p, n));
                $(let $a = <$t as Field>::read(&mut r);)*
                let _ = &mut r;
                shims!(@back shims!(@held $how h $name $($a)*) $(, -> $n)? $(, => $told)?)
            }
        };
    )* };
}
game_start::calls!(shims);
