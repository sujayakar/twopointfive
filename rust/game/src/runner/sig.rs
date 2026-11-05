#![allow(unused_variables)]
//! Every function and method of `runner`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/runner.ts:111 runnerLines)
pub fn runner_lines(kind: Option<crate::runner::LineKind>) -> Vec<String> { todo!() }

/// (src/game/runner.ts:113 bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::runner::LineKind) { todo!() }

/// (src/game/runner.ts:118 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/runner.ts:119 chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/runner.ts:120 headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/runner.ts:122 ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:124 spoken)
pub fn spoken(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:126 reacts)
pub fn reacts(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:131 mateOf)
pub fn mate_of(g: &mut crate::game::Game, gd: GuardId, o: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:134 foeAt)
pub fn foe_at(g: &mut crate::game::Game, c_2: CharId, who: crate::picture::Who) -> crate::runner::Foe { todo!() }

/// (src/game/runner.ts:136 foes)
pub fn foes(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::runner::Foe> { todo!() }

/// (src/game/runner.ts:142 byPicture)
pub fn by_picture(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/runner.ts:146 known)
pub fn known(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::runner::Foe> { todo!() }

/// (src/game/runner.ts:153 closeFoe)
pub fn close_foe(g: &mut crate::game::Game, gd: GuardId) -> Option<CharId> { todo!() }

/// (src/game/runner.ts:161 ripe)
pub fn ripe(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:170 lost)
pub fn lost(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:175 start)
pub fn start(g: &mut crate::game::Game, gd: GuardId, from: V3) { todo!() }

/// (src/game/runner.ts:183 runnerBreak)
pub fn runner_break(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:192 rectOf)
pub fn rect_of(g: &mut crate::game::Game) -> Option<[f64; 4]> { todo!() }

/// (src/game/runner.ts:200 outside)
pub fn outside(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/runner.ts:207 lineTo)
pub fn line_to(g: &mut crate::game::Game, e: crate::runner::Foe, p: V3) -> bool { todo!() }

/// (src/game/runner.ts:213 flood)
pub fn flood(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe], ask: Option<f64>, see: Option<&[crate::runner::Foe]>) -> crate::runner::Flood { todo!() }

/// (src/game/runner.ts:236 route)
pub fn route(g: &mut crate::game::Game, gd: GuardId, goal: V3, e: &[crate::runner::Foe]) -> bool { todo!() }

/// (src/game/runner.ts:245 pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { todo!() }

/// (src/game/runner.ts:248 pickMate)
pub fn pick_mate(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/runner.ts:256 wayBarred)
pub fn way_barred(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe]) -> bool { todo!() }

/// (src/game/runner.ts:270 drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:281 toOut)
pub fn to_out(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe]) { todo!() }

/// (src/game/runner.ts:285 toLow)
pub fn to_low(g: &mut crate::game::Game, gd: GuardId, at: V3, e: &[crate::runner::Foe]) { todo!() }

/// (src/game/runner.ts:288 planOut)
pub fn plan_out(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe], f: crate::runner::PlanOutF) -> bool { todo!() }

/// (src/game/runner.ts:301 tell)
pub fn tell(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:314 pose)
pub fn pose(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:323 runnerTick)
pub fn runner_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/runner.ts:333 step)
pub fn step(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/runner.ts:381 gone)
pub fn gone(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:387 drive)
pub fn drive(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/runner.ts:391 runnerScript)
pub fn runner_script(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> { todo!() }

/// (src/game/runner.ts:397 runnerOwns)
pub fn runner_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:399 runnerFled)
pub fn runner_fled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:401 runnerBare)
pub fn runner_bare(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:406 runnerNobody)
pub fn runner_nobody(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:408 runnerMute)
pub fn runner_mute(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/runner.ts:410 runnerOf)
pub fn runner_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::runner::RunnerOf> { todo!() }

/// (src/game/runner.ts:412 runnerSummary)
pub fn runner_summary(g: &mut crate::game::Game) -> String { todo!() }
