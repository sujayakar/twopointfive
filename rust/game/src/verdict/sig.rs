#![allow(unused_variables)]
//! Every function and method of `verdict`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/verdict.ts:49 rowFile)
pub fn row_file(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/verdict.ts:59 loudRow)
pub fn loud_row(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/verdict.ts:119 Verdict.constructor)
pub fn verdict_new(g: &mut crate::game::Game) -> crate::verdict::Verdict { todo!() }

/// (src/game/verdict.ts:122 Verdict.live)
pub fn verdict_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/verdict.ts:126 Verdict.started)
pub fn verdict_started(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/verdict.ts:128 Verdict.result)
pub fn verdict_result(g: &mut crate::game::Game) -> Option<crate::verdict::RunVerdict> { todo!() }

/// (src/game/verdict.ts:130 Verdict.secs)
pub fn verdict_secs(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/verdict.ts:138 Verdict.tick)
pub fn verdict_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/verdict.ts:154 Verdict.fired)
pub fn verdict_fired(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/verdict.ts:155 Verdict.fresh)
pub fn verdict_fresh(g: &mut crate::game::Game, tail: Option<u32>) { todo!() }

/// (src/game/verdict.ts:171 Verdict.roster)
pub fn verdict_roster(g: &mut crate::game::Game) { todo!() }

/// (src/game/verdict.ts:180 Verdict.onlyMate)
pub fn verdict_only_mate(g: &mut crate::game::Game, h: &crate::verdict::VerdictOnlyMateH) -> Option<crate::verdict::Track> { todo!() }

/// (src/game/verdict.ts:188 Verdict.watch)
pub fn verdict_watch(g: &mut crate::game::Game) { todo!() }

/// (src/game/verdict.ts:228 Verdict.ended)
pub fn verdict_ended(g: &mut crate::game::Game) -> Option<crate::verdict::VerdictEnded> { todo!() }

/// (src/game/verdict.ts:244 Verdict.close)
pub fn verdict_close(g: &mut crate::game::Game, end: &crate::verdict::VerdictCloseEnd) { todo!() }

/// (src/game/verdict.ts:262 Verdict.approach)
pub fn verdict_approach(g: &mut crate::game::Game, kind: Option<crate::verdict::VerdictApproachKind>) -> crate::verdict::Approach { todo!() }

/// (src/game/verdict.ts:276 humans)
pub fn humans(g: &mut crate::game::Game) -> Vec<PlayerId> { todo!() }

/// (src/game/verdict.ts:277 track)
pub fn track(g: &mut crate::game::Game, id_2: f64, name: &str, tone: Option<crate::host_findme::OutlineHue>, human: bool, gd: Option<GuardId>) -> crate::verdict::Track { todo!() }
