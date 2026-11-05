#![allow(unused_variables)]
//! Every function and method of `powerout`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/powerout.ts:114 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/powerout.ts:115 frac)
pub fn frac(x: f64) -> f64 { todo!() }

/// (src/game/powerout.ts:117 idShare)
pub fn id_share(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/powerout.ts:118 alive)
pub fn alive(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:119 fit)
pub fn fit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:120 roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:121 loudHeard)
pub fn loud_heard(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/powerout.ts:122 defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/powerout.ts:124 ruled)
pub fn ruled(g: &mut crate::game::Game) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/powerout.ts:128 event)
pub fn event(g: &mut crate::game::Game, what: crate::powerout::EventWhat, gd: Option<GuardId>, why: &str) { todo!() }

/// (src/game/powerout.ts:134 sawThemDie)
pub fn saw_them_die(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:145 floorWatched)
pub fn floor_watched(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/powerout.ts:159 carried)
pub fn carried() -> bool { todo!() }

/// (src/game/powerout.ts:161 powerOutKept)
pub fn power_out_kept(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<String> { todo!() }

/// (src/game/powerout.ts:178 leverSpot)
pub fn lever_spot(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/powerout.ts:183 wayTo)
pub fn way_to(g: &mut crate::game::Game, from: V3, to: V3) -> Option<crate::powerout::WayTo> { todo!() }

/// (src/game/powerout.ts:190 coverSpot)
pub fn cover_spot(g: &mut crate::game::Game, lever: V3, way: &[V3], from: V3) -> crate::powerout::CoverSpot { todo!() }

/// (src/game/powerout.ts:203 calmOf)
pub fn calm_of(g: &mut crate::game::Game, d: RoomfightDefId) -> crate::powerout::Calm { todo!() }

/// (src/game/powerout.ts:208 walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) { todo!() }

/// (src/game/powerout.ts:210 settle)
pub fn settle(g: &mut crate::game::Game, m: &crate::powerout::Man, d: RoomfightDefId) { todo!() }

/// (src/game/powerout.ts:214 calmBack)
pub fn calm_back(g: &mut crate::game::Game, d: RoomfightDefId, c_2: &crate::powerout::Calm, wary: bool) { todo!() }

/// (src/game/powerout.ts:224 calmLoad)
pub fn calm_load(g: &mut crate::game::Game, c_2: &crate::powerout::CalmSnap) -> crate::powerout::Calm { todo!() }

/// (src/game/powerout.ts:227 powerOut)
pub fn power_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/powerout.ts:244 noticed)
pub fn noticed(g: &mut crate::game::Game, gd: GuardId, how: &str) { todo!() }

/// (src/game/powerout.ts:249 nobody)
pub fn nobody(g: &mut crate::game::Game) { todo!() }

/// (src/game/powerout.ts:254 powerOutMan)
pub fn power_out_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:256 powerOutLit)
pub fn power_out_lit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:261 powerOutWord)
pub fn power_out_word(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:263 powerOutOwns)
pub fn power_out_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/powerout.ts:265 powerOutTimeline)
pub fn power_out_timeline(g: &mut crate::game::Game) -> Vec<crate::powerout::PowerOutEvent> { todo!() }

/// (src/game/powerout.ts:266 powerOutReset)
pub fn power_out_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/powerout.ts:269 decide)
pub fn decide(g: &mut crate::game::Game, defs: &[RoomfightDefId]) { todo!() }

/// (src/game/powerout.ts:303 powerOutTick)
pub fn power_out_tick(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/powerout.ts:372 stepOff)
pub fn step_off(g: &mut crate::game::Game, fight: bool) { todo!() }

/// (src/game/powerout.ts:388 missed)
pub fn missed(g: &mut crate::game::Game, defs: &[RoomfightDefId], fight: bool) { todo!() }

/// (src/game/powerout.ts:397 standDown)
pub fn stand_down(g: &mut crate::game::Game) { todo!() }
