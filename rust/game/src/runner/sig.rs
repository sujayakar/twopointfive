#![allow(unused_variables)]
//! Every function and method of `runner`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/runner.ts:114 runnerLines)
pub fn runner_lines(kind: Option<crate::runner::LineKind>) -> Vec<String> { todo!() }

/// (src/game/runner.ts:116 bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::runner::LineKind) { todo!() }

/// (src/game/runner.ts:121 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/runner.ts:122 chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/runner.ts:123 headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/runner.ts:125 ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:127 spoken)
pub fn spoken(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:129 reacts)
pub fn reacts(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:134 mateOf)
pub fn mate_of(g: &mut crate::game::Game, gd: GuardId, o: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:137 foeAt)
pub fn foe_at(g: &mut crate::game::Game, c_2: CharId, who: crate::picture::Who) -> crate::runner::Foe { todo!() }

/// (src/game/runner.ts:139 foes)
pub fn foes(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::runner::Foe> { todo!() }

/// (src/game/runner.ts:145 byPicture)
pub fn by_picture(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/runner.ts:149 known)
pub fn known(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::runner::Foe> { todo!() }

/// (src/game/runner.ts:156 closeFoe)
pub fn close_foe(g: &mut crate::game::Game, gd: GuardId) -> Option<CharId> { todo!() }

/// (src/game/runner.ts:164 ripe)
pub fn ripe(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:173 lost)
pub fn lost(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:178 start)
pub fn start(g: &mut crate::game::Game, gd: GuardId, from: V3) { todo!() }

/// (src/game/runner.ts:186 runnerBreak)
pub fn runner_break(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:195 rectOf)
pub fn rect_of(g: &mut crate::game::Game) -> Option<[f64; 4]> { todo!() }

/// (src/game/runner.ts:203 outside)
pub fn outside(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/runner.ts:210 lineTo)
pub fn line_to(g: &mut crate::game::Game, e: crate::runner::Foe, p: V3) -> bool { todo!() }

/// (src/game/runner.ts:216 flood)
pub fn flood(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe], ask: Option<f64>, see: Option<&[crate::runner::Foe]>) -> crate::runner::Flood { todo!() }

/// (src/game/runner.ts:239 route)
pub fn route(g: &mut crate::game::Game, gd: GuardId, goal: V3, e: &[crate::runner::Foe]) -> bool { todo!() }

/// (src/game/runner.ts:248 pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { todo!() }

/// (src/game/runner.ts:251 pickMate)
pub fn pick_mate(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/runner.ts:259 wayBarred)
pub fn way_barred(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe]) -> bool { todo!() }

/// (src/game/runner.ts:273 drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:284 toOut)
pub fn to_out(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe]) { todo!() }

/// (src/game/runner.ts:288 toLow)
pub fn to_low(g: &mut crate::game::Game, gd: GuardId, at: V3, e: &[crate::runner::Foe]) { todo!() }

/// (src/game/runner.ts:291 planOut)
pub fn plan_out(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe], f: crate::runner::PlanOutF) -> bool { todo!() }

/// (src/game/runner.ts:304 tell)
pub fn tell(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:317 pose)
pub fn pose(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:326 runnerTick)
pub fn runner_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/runner.ts:336 step)
pub fn step(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/runner.ts:384 gone)
pub fn gone(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:390 drive)
pub fn drive(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:394 runnerScript)
pub fn runner_script(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> { todo!() }

/// (src/game/runner.ts:400 runnerBare)
pub fn runner_bare(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:405 runnerNobody)
pub fn runner_nobody(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:407 runnerMute)
pub fn runner_mute(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:409 runnerOf)
pub fn runner_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::runner::RunnerOf> { todo!() }

/// (src/game/runner.ts:411 runnerSummary)
pub fn runner_summary(g: &mut crate::game::Game) -> String { todo!() }


/// INVENTED (conventions 4, an object lent): what run_guard_script left in the script runner_script lent goes back on his row, whose step and pose read `arrived` (guards.ts:2144; squad.ts:130 writes it; runner.ts:315, 317, 346, 365, 367, 370 read it). None, or no row: the row stays.
pub fn runner_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) { todo!() }
