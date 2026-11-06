//! THE BRING-UP'S OWN PART of seats: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// INVENTED BY THE SKELETON: which row of a table kept per view this pass reads: the man of the seat on Game, or CAST_KEY inside a second cast.
/// (seats.ts SEAT_TABLES key by the view; the bridge writes a view's key as `view.player`, export.ts:83)
pub fn view_key(g: &mut crate::game::Game) -> PlayerId {
    if g.in_cast { CAST_KEY } else { g.player }
}

/// one seat's SEAT_KEYS fields and Game's, changed over
fn swap_set(g: &mut Game, o: &mut SeatSet) {
    use core::mem::swap;
    swap(&mut g.seat, &mut o.seat); swap(&mut g.player, &mut o.player); swap(&mut g.orders, &mut o.orders); swap(&mut g.door_tools, &mut o.door_tools);
    swap(&mut g.work, &mut o.work); swap(&mut g.wall_charge, &mut o.wall_charge); swap(&mut g.aim_point, &mut o.aim_point); swap(&mut g.aim_target, &mut o.aim_target);
    swap(&mut g.aim_guard, &mut o.aim_guard); swap(&mut g.aim_friend, &mut o.aim_friend); swap(&mut g.aim_snap, &mut o.aim_snap); swap(&mut g.hover, &mut o.hover);
    swap(&mut g.interactables, &mut o.interactables); swap(&mut g.nearby, &mut o.nearby); swap(&mut g.act, &mut o.act); swap(&mut g.use_door, &mut o.use_door);
    swap(&mut g.view_proj, &mut o.view_proj); swap(&mut g.threat_proximity, &mut o.threat_proximity); swap(&mut g.torch_on_t, &mut o.torch_on_t);
}

/// HOW DEEP IN A SEAT'S VIEW THE CALL STANDS. seats.ts tells a seat's view from the bare Game by identity (`rawGame(g) !== g`); here the first seat's set
/// stands on Game both bare and inside his view, so the two are told apart by whether a `with_seat` is open. STAND-IN for a word on Game (the types are
/// generated): a count of the process, right while one game is stepped at a time.
static VIEW_DEPTH: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// is this call inside a seat's view (seats.ts `rawGame(g) !== g`), and not on the bare Game
pub fn in_view() -> bool { VIEW_DEPTH.load(core::sync::atomic::Ordering::Relaxed) > 0 }

/// INVENTED BY THE SKELETON (conventions, SEATS): a pass about another seat. That seat's SEAT_KEYS fields are put on Game for the call and
/// taken back after; the seat that stood on Game keeps its own meanwhile. The seat already on Game runs as it stands (so a nesting is fine).
/// For the length of the call the game is "a seat's view" (`in_view`), the first seat's too.
pub fn with_seat<R>(g: &mut crate::game::Game, seat: SeatId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    use core::sync::atomic::Ordering::Relaxed;
    if g.seat == seat { VIEW_DEPTH.fetch_add(1, Relaxed); let r = f(g); VIEW_DEPTH.fetch_sub(1, Relaxed); return r; }
    let from = g.seat;
    let mut theirs = match g.tab.cmd_seat[seat].own.take() { Some(o) => o, None => panic!("with_seat: that seat keeps no set") };
    swap_set(g, &mut theirs);                 // Game now stands on `seat`; `theirs` holds what `from` had
    g.tab.cmd_seat[from].own = Some(theirs);
    VIEW_DEPTH.fetch_add(1, Relaxed);
    let r = f(g);
    VIEW_DEPTH.fetch_sub(1, Relaxed);
    let mut back = g.tab.cmd_seat[from].own.take().unwrap();
    swap_set(g, &mut back);
    g.tab.cmd_seat[seat].own = Some(back);
    r
}

/// THE BARE GAME FOR THE LENGTH OF A TICK: the first seat's set on Game and no view open (seats.ts seatsStand: inside a tick the Game's own fields of a
/// seat's name answer the FIRST seat's). Where another seat stood on Game (a host that shows the second player), the sets are changed over and back.
pub fn on_first_bare<R>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    let first = g.seats[0];
    if g.seat == first { return f(g); }
    let from = g.seat;
    let mut theirs = match g.tab.cmd_seat[first].own.take() { Some(o) => o, None => panic!("on_first_bare: the first seat keeps no set") };
    swap_set(g, &mut theirs);
    g.tab.cmd_seat[from].own = Some(theirs);
    let r = f(g);
    let mut back = g.tab.cmd_seat[from].own.take().unwrap();
    swap_set(g, &mut back);
    g.tab.cmd_seat[first].own = Some(back);
    r
}
