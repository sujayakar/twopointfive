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

/// INVENTED BY THE SKELETON (conventions, SEATS): a pass about another seat. The seat whose man is `seat` has its SEAT_KEYS fields put on Game for the call and
/// taken back after; the seat that stood on Game keeps its own meanwhile. The seat already on Game runs as it stands (so a nesting is fine).
/// NEVER RUN in the bring-up past its first line: the missions compared have one seat.
pub fn with_seat<R>(g: &mut crate::game::Game, seat: PlayerId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    if g.player == seat { return f(g); }
    let (from, mut to) = (g.seat, None);
    for i in 0..g.seats.len() { let s = g.seats[i]; if g.tab.cmd_seat[s].own.as_ref().map_or(false, |o| o.player == seat) { to = Some(s); break; } }
    let to = match to { Some(s) => s, None => panic!("with_seat: no seat keeps that man's set") };
    let mut theirs = g.tab.cmd_seat[to].own.take().unwrap();
    swap_set(g, &mut theirs);                 // Game now stands on `to`; `theirs` holds what `from` had
    g.tab.cmd_seat[from].own = Some(theirs);
    let r = f(g);
    let mut back = g.tab.cmd_seat[from].own.take().unwrap();
    swap_set(g, &mut back);
    g.tab.cmd_seat[to].own = Some(back);
    r
}
