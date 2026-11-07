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
pub fn view_key(g: &crate::game::Game) -> PlayerId {
    if g.in_cast { CAST_KEY } else { g.player }
}

/// A SEAT'S MAN IS MADE ANEW (a restart): every table kept per view moves its row from the man who left to the man who stands in his place.
/// The TypeScript keys these tables by the VIEW (seats.ts SEAT_TABLES, the eighteen named there), which a restart does not replace, so what a module
/// kept for the seat is kept across it (the rear watch's book and its log, the squad's throws). Here a view's key is its seat's man (`view_key`), and a
/// new man is a new key: without this every such module began again at a restart. The rows keep their order in each table; a row already under the
/// new man (none is made before this is called) gives way to the seat's own.
pub fn view_rekey(g: &mut crate::game::Game, old: PlayerId, new: PlayerId) {
    fn rekey<V>(m: &mut crate::js::JsMap<PlayerId, V>, old: PlayerId, new: PlayerId) {
        if old == new || !m.has(old) { return; }
        m.delete(new);
        let mut rows: Vec<(PlayerId, V)> = Vec::new();
        for k in m.keys() { if let Some(v) = m.delete(k) { rows.push((if k == old { new } else { k }, v)); } }
        m.clear();
        for (k, v) in rows { m.set(k, v); }
    }
    let m = &mut g.m;
    rekey(&mut m.blast_feel.state, old, new); rekey(&mut m.blast_look.state, old, new); rekey(&mut m.close.state, old, new);
    rekey(&mut m.guards.ground_track, old, new); rekey(&mut m.holdfire.states, old, new); rekey(&mut m.player.light_at, old, new);
    rekey(&mut m.rearwatch.books, old, new); rekey(&mut m.samlook.state, old, new); rekey(&mut m.showwhy.states, old, new);
    rekey(&mut m.silent.seats, old, new); rekey(&mut m.silent.states, old, new); rekey(&mut m.squad40.states, old, new);
    rekey(&mut m.squadsmoke.states, old, new); rekey(&mut m.squadthrow.states, old, new); rekey(&mut m.stalk.state, old, new);
    rekey(&mut m.tapstep.state, old, new); rekey(&mut m.unseen.states, old, new); rekey(&mut m.wandorder.states, old, new);
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

/// INVENTED BY THE SKELETON (conventions, SEATS): a pass about another seat. That seat's SEAT_KEYS fields are put on Game for the call and
/// taken back after; the seat that stood on Game keeps its own meanwhile. The seat already on Game runs as it stands (so a nesting is fine).
/// For the length of the call the game is "a seat's view" (`in_view`), the first seat's too.
pub fn with_seat<R>(g: &mut crate::game::Game, seat: SeatId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    if g.seat == seat { g.view_depth += 1; let r = f(g); g.view_depth -= 1; return r; }
    let from = g.seat;
    let mut theirs = match g.tab.cmd_seat[seat].own.take() { Some(o) => o, None => panic!("with_seat: that seat keeps no set") };
    swap_set(g, &mut theirs);                 // Game now stands on `seat`; `theirs` holds what `from` had
    g.tab.cmd_seat[from].own = Some(theirs);
    g.view_depth += 1;
    let r = f(g);
    g.view_depth -= 1;
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

/// HOW MANY PLAYERS' MEN STAND ON THE FLOOR: one a seat. Every budget of characters counts this, never "1" (a floor holds at most
/// guards::MAX_SKINNED characters with every seat counted: the seats' men, every teammate, every other man stood).
pub fn seat_count(g: &crate::game::Game) -> f64 { g.seats.len() as f64 }

/// A BUDGET OF CHARACTERS WRITTEN FOR ONE PLAYER, with every seat counted: one more for each further seat. The ONE rule every staging's budget goes
/// through (the eight a caller gets without asking, the four's ten, a structure's, a rescue's cast); a game of one reads the number as written.
/// spawn_unit holds any budget to guards::MAX_SKINNED; past that a staging cuts its defenders (floor_room).
pub fn budget_with_seats(g: &crate::game::Game, for_one: f64) -> f64 { for_one - 1.0 + seat_count(g) }

/// How many more men the floor can be given as it stands: the picture's sixteen less a man a seat and less every guard, teammate and unit.
/// A staging stands its team first and then as many of its defenders, in the order of their marks, as this leaves (a game of one: all of them).
pub fn floor_room(g: &crate::game::Game) -> f64 { crate::guards::MAX_SKINNED - seat_count(g) - g.guards.len() as f64 }
