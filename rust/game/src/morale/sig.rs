#![allow(unused_variables)]
//! Every function and method of `morale`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/morale.ts:156 moraleLines)
pub fn morale_lines(g: &mut crate::game::Game, kind: Option<crate::morale::LineKind>) -> Vec<String> { todo!() }

/// (src/game/morale.ts:159 bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::morale::LineKind, radio: bool, force: Option<bool>) -> bool { todo!() }

/// (src/game/morale.ts:167 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/morale.ts:168 chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/morale.ts:169 headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/morale.ts:171 ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:173 rescued)
pub fn rescued(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:175 reacts)
pub fn reacts(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:179 mind)
pub fn mind<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::morale::Mind { todo!() }

/// (src/game/morale.ts:197 inTheDark)
pub fn in_the_dark(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:203 hasMate)
pub fn has_mate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:213 bandOf)
pub fn band_of(g: &mut crate::game::Game, gd: GuardId) -> Option<Vec<GuardId>> { todo!() }

/// (src/game/morale.ts:220 mixNow)
pub fn mix_now(g: &mut crate::game::Game) -> crate::morale::MixNow { todo!() }

/// (src/game/morale.ts:228 trainingOf)
pub fn training_of(g: &mut crate::game::Game, gd: GuardId) -> crate::morale::TrainingOf { todo!() }

/// (src/game/morale.ts:241 leaderOf)
pub fn leader_of(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/morale.ts:248 folds)
pub fn folds(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:250 nerved)
pub fn nerved(g: &mut crate::game::Game, x: f64) -> f64 { todo!() }

/// (src/game/morale.ts:253 stand)
pub fn stand(g: &mut crate::game::Game, gd: GuardId, first: Option<bool>) { todo!() }

/// (src/game/morale.ts:261 roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/morale.ts:262 standing)
pub fn standing(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:263 dread)
pub fn dread(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:266 mayBreak)
pub fn may_break(s: crate::morale::Mind) -> bool { todo!() }

/// (src/game/morale.ts:269 rungNow)
pub fn rung_now(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:275 moraleShock)
pub fn morale_shock(g: &mut crate::game::Game, gd: GuardId, amount: f64) { todo!() }

/// (src/game/morale.ts:281 moraleManDown)
pub fn morale_man_down(g: &mut crate::game::Game, victim: GuardId) { todo!() }

/// (src/game/morale.ts:295 sawLeaderFall)
pub fn saw_leader_fall(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:299 moraleBlast)
pub fn morale_blast(g: &mut crate::game::Game, at: V3, who: Option<f64>) { todo!() }

/// (src/game/morale.ts:306 moraleReset)
pub fn morale_reset(g: &mut crate::game::Game, men: Option<bool>) { todo!() }

/// (src/game/morale.ts:309 feed)
pub fn feed(g: &mut crate::game::Game, b: crate::morale::Blast) { todo!() }

/// (src/game/morale.ts:333 begin)
pub fn begin(g: &mut crate::game::Game, gd: GuardId, act: crate::morale::MoraleAct, hole: Option<V3>) { todo!() }

/// (src/game/morale.ts:334 end)
pub fn end(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/morale.ts:339 seenBy)
pub fn seen_by(g: &mut crate::game::Game, gd: GuardId, hole: Option<V3>) -> Vec<CharId> { todo!() }

/// (src/game/morale.ts:348 thrown)
pub fn thrown(g: &mut crate::game::Game, gd: GuardId, hole: V3) -> bool { todo!() }

/// (src/game/morale.ts:355 clearRun)
pub fn clear_run(g: &mut crate::game::Game, gd: GuardId, dx: f64, dz: f64, max: f64) -> f64 { todo!() }

/// (src/game/morale.ts:368 awayFrom)
pub fn away_from(g: &mut crate::game::Game, c_2: CharId, hole: V3) -> [f64; 2] { todo!() }

/// (src/game/morale.ts:375 backGoal)
pub fn back_goal(g: &mut crate::game::Game, gd: GuardId, hole: V3) -> Option<V3> { todo!() }

/// (src/game/morale.ts:386 sideGoal)
pub fn side_goal(g: &mut crate::game::Game, gd: GuardId, hole: V3, to: Option<GuardId>) -> Option<V3> { todo!() }

/// (src/game/morale.ts:397 round)
pub fn round(g: &mut crate::game::Game, gd: GuardId, dt: f64, every: f64, of: crate::morale::RoundOf) -> bool { todo!() }

/// (src/game/morale.ts:410 runDry)
pub fn run_dry(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/morale.ts:419 bait)
pub fn bait(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/morale.ts:428 pose)
pub fn pose(g: &mut crate::game::Game, gd: GuardId, rooted: bool) { todo!() }

/// (src/game/morale.ts:437 moraleTick)
pub fn morale_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/morale.ts:458 step)
pub fn step(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/morale.ts:522 drive)
pub fn drive(g: &mut crate::game::Game, gd: GuardId, dt: f64, rooted: bool) { todo!() }

/// (src/game/morale.ts:533 moraleScript)
pub fn morale_script(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> { todo!() }

/// (src/game/morale.ts:541 dreadOf)
pub fn dread_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:544 shakenOf)
pub fn shaken_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:546 rungOf)
pub fn rung_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/morale.ts:548 moraleOf)
pub fn morale_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::morale::MoraleOf> { todo!() }

/// (src/game/morale.ts:550 moraleSummary)
pub fn morale_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/morale.ts:558 moraleRoster)
pub fn morale_roster(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/morale.ts:569 kept)
pub fn kept() -> crate::morale::Kept { todo!() }

/// (src/game/morale.ts:573 moraleKeep)
pub fn morale_keep(g: &mut crate::game::Game, row: crate::morale::MoraleKeepRow) { todo!() }

/// (src/game/morale.ts:576 moraleFromQuery)
pub fn morale_from_query(g: &mut crate::game::Game, q: crate::Opaque) { todo!() }

/// (src/game/morale.ts:583 moraleRelease)
pub fn morale_release(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/morale.ts:587 n3)
pub fn n3(p: Option<V3>) -> Vec<f64> { todo!() }


/// INVENTED (conventions 4, an object lent): what run_guard_script left in the script morale_script lent goes back on his row, whose step reads `arrived` (guards.ts:2145; squad.ts:130 writes it; morale.ts:477, 496 read it). None, or no row: the row stays.
pub fn morale_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) { todo!() }
