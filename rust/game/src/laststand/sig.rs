#![allow(unused_variables)]
//! Every function and method of `laststand`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/laststand.ts:96 saveLastStand)
pub fn save_last_stand(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:146 newStats)
pub fn new_stats() -> crate::laststand::StateStats { todo!() }

/// (src/game/laststand.ts:173 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/laststand.ts:174 alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/laststand.ts:175 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/laststand.ts:176 cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/laststand.ts:177 inRoom)
pub fn in_room(g: &mut crate::game::Game, r: Option<RoomfightRoomId>, p: V3) -> bool { todo!() }

/// (src/game/laststand.ts:178 note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/laststand.ts:180 fitGd)
pub fn fit_gd(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/laststand.ts:181 fit)
pub fn fit(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/laststand.ts:183 calm)
pub fn calm(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/laststand.ts:186 movable)
pub fn movable(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/laststand.ts:189 where)
pub fn where_(g: &mut crate::game::Game, p: V3) -> crate::laststand::Where { todo!() }

/// (src/game/laststand.ts:202 spoken)
pub fn spoken(g: &mut crate::game::Game, p: V3) -> crate::laststand::Spoken { todo!() }

/// (src/game/laststand.ts:210 voice)
pub fn voice(g: &mut crate::game::Game, gd: GuardId, kind: crate::laststand::LineKind, text: &str, cls: f64) { todo!() }

/// (src/game/laststand.ts:219 fill)
pub fn fill(tpl: &str, vars: &JsMap<String, String>) -> String { todo!() }

/// (src/game/laststand.ts:220 net)
pub fn net(g: &mut crate::game::Game, gd: GuardId, kind: crate::laststand::LineKind, vars: Option<&JsMap<String, String>>, then: Option<&crate::laststand::LaterThen>, redo: Option<&crate::laststand::LaterRedo>) { todo!() }

/// (src/game/laststand.ts:228 netAt)
pub fn net_at(g: &mut crate::game::Game, d: RoomfightDefId, kind: crate::laststand::LineKind, f: &crate::laststand::LaterRedo) { todo!() }

/// (src/game/laststand.ts:232 tickLater)
pub fn tick_later(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:242 name)
pub fn name(g: &mut crate::game::Game, d: RoomfightDefId, id_2: Option<&str>) { todo!() }

/// (src/game/laststand.ts:244 nameAt)
pub fn name_at(g: &mut crate::game::Game, d: RoomfightDefId, w: Option<&crate::laststand::Spoken>) { todo!() }

/// (src/game/laststand.ts:251 tickMoved)
pub fn tick_moved(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:272 teamKnown)
pub fn team_known(g: &mut crate::game::Game, near: V3) -> crate::laststand::TeamKnown { todo!() }

/// (src/game/laststand.ts:283 spotIn)
pub fn spot_in(g: &mut crate::game::Game, r: RoomfightRoomId, p0: V3, taken: &[V3]) -> V3 { todo!() }

/// (src/game/laststand.ts:296 wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> { todo!() }

/// (src/game/laststand.ts:307 canCome)
pub fn can_come(g: &mut crate::game::Game, x: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> { todo!() }

/// (src/game/laststand.ts:313 standRoom)
pub fn stand_room(g: &mut crate::game::Game, d: RoomfightDefId, known: &[V3]) -> Option<crate::laststand::StandRoom> { todo!() }

/// (src/game/laststand.ts:320 homeRoom)
pub fn home_room(g: &mut crate::game::Game, d: RoomfightDefId, known: &[V3]) -> Option<crate::laststand::HomeRoom> { todo!() }

/// (src/game/laststand.ts:327 theOf)
pub fn the_of(g: &mut crate::game::Game, id_2: &str) -> String { todo!() }

/// (src/game/laststand.ts:331 choose)
pub fn choose(g: &mut crate::game::Game, men: &[RoomfightDefId], known: &[V3], min: Option<f64>) -> Option<crate::laststand::Plan> { todo!() }

/// (src/game/laststand.ts:352 startCheck)
pub fn start_check(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:367 tickCheck)
pub fn tick_check(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:391 lone)
pub fn lone(st: &mut crate::laststand::State) -> bool { todo!() }

/// (src/game/laststand.ts:393 tellOnly)
pub fn tell_only(g: &mut crate::game::Game, c_2: RoomfightDefId) { todo!() }

/// (src/game/laststand.ts:402 newStand)
pub fn new_stand(g: &mut crate::game::Game, room: Option<RoomfightRoomId>, id_2: Option<&str>, the: &str, anchor: V3) -> crate::laststand::Stand { todo!() }

/// (src/game/laststand.ts:404 anchorOf)
pub fn anchor_of(g: &mut crate::game::Game, s: &crate::laststand::Stand) -> V3 { todo!() }

/// (src/game/laststand.ts:405 member)
pub fn member(g: &mut crate::game::Game, s: &crate::laststand::Stand, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/laststand.ts:407 holdsOwn)
pub fn holds_own(g: &mut crate::game::Game, s: &mut crate::laststand::Stand, d: RoomfightDefId, say: Option<crate::laststand::LineKind>) { todo!() }

/// (src/game/laststand.ts:412 decide)
pub fn decide(g: &mut crate::game::Game, c_2: RoomfightDefId, heard: &[RoomfightDefId]) { todo!() }

/// (src/game/laststand.ts:467 tickMover)
pub fn tick_mover(g: &mut crate::game::Game, s: &mut crate::laststand::Stand, m: &mut crate::laststand::Mover) -> bool { todo!() }

/// (src/game/laststand.ts:500 comeBy)
pub fn come_by(g: &mut crate::game::Game, d: RoomfightDefId, r: RoomfightRoomId) -> V3 { todo!() }

/// (src/game/laststand.ts:512 tickStand)
pub fn tick_stand(g: &mut crate::game::Game, s: &mut crate::laststand::Stand) { todo!() }

/// (src/game/laststand.ts:559 fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::laststand::State { todo!() }

/// (src/game/laststand.ts:566 dissolve)
pub fn dissolve(g: &mut crate::game::Game, s: &crate::laststand::Stand) { todo!() }

/// (src/game/laststand.ts:568 ofStand)
pub fn of_stand(g: &mut crate::game::Game, s: &crate::laststand::Stand, gd: GuardId) -> bool { todo!() }

/// (src/game/laststand.ts:569 standSay)
pub fn stand_say(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:571 lastStandTick)
pub fn last_stand_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:572 stepStand)
pub fn step_stand(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:593 lastStandStop)
pub fn last_stand_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/laststand.ts:596 lastStandHint)
pub fn last_stand_hint(g: &mut crate::game::Game) -> Option<crate::laststand::Hint> { todo!() }

/// (src/game/laststand.ts:606 lastStandStats)
pub fn last_stand_stats(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/laststand.ts:608 lastStandReport)
pub fn last_stand_report(g: &mut crate::game::Game) -> crate::laststand::LastStandReport { todo!() }

/// (src/game/laststand.ts:613 lastStandSummary)
pub fn last_stand_summary(g: &mut crate::game::Game) -> String { todo!() }
