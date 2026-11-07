#![allow(unused_variables)]
//! Every function and method of `showwhy`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/showwhy.ts:115 blank)
pub fn blank(t: f64) -> crate::showwhy::Mem { todo!() }

/// (src/game/showwhy.ts:116 fresh)
pub fn fresh() -> crate::showwhy::ShowWhyStats { todo!() }

/// (src/game/showwhy.ts:117 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::showwhy::State { todo!() }

/// (src/game/showwhy.ts:118 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/showwhy.ts:121 showWhyStats)
pub fn show_why_stats(g: &mut crate::game::Game) -> crate::showwhy::ShowWhyStats { todo!() }

/// (src/game/showwhy.ts:122 showWhyLog)
pub fn show_why_log(g: &mut crate::game::Game) -> Vec<crate::showwhy::WhyNote> { todo!() }

/// (src/game/showwhy.ts:124 resetShowWhy)
pub fn reset_show_why(g: &mut crate::game::Game) { todo!() }

/// (src/game/showwhy.ts:126 note)
pub fn note(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, why: crate::showwhy::Why, signal: Option<crate::character::SignalKind>, line: Option<&str>) { todo!() }

/// (src/game/showwhy.ts:130 speaking)
pub fn speaking(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/showwhy.ts:131 say)
pub fn say(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, why: crate::showwhy::Why, text: &str) -> bool { todo!() }

/// (src/game/showwhy.ts:135 sign)
pub fn sign(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, why: crate::showwhy::Why, kind: crate::character::SignalKind, side: Option<f64>) { todo!() }

/// (src/game/showwhy.ts:142 masked)
pub fn masked(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/showwhy.ts:150 mateNearer)
pub fn mate_nearer(g: &mut crate::game::Game, gd: GuardId, to: V3) -> bool { todo!() }

/// (src/game/showwhy.ts:160 lineSide)
pub fn line_side(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/showwhy.ts:166 lineSaidBy)
pub fn line_said_by(g: &mut crate::game::Game, but: GuardId) -> bool { todo!() }

/// (src/game/showwhy.ts:173 waits)
pub fn waits(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/showwhy.ts:183 standsFree)
pub fn stands_free(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/showwhy.ts:189 manAhead)
pub fn man_ahead(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/showwhy.ts:198 stacks)
pub fn stacks(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/showwhy.ts:223 showWhyTick)
pub fn show_why_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/showwhy.ts:294 owe)
pub fn owe(m: &mut crate::showwhy::Mem, text: &str, why: crate::showwhy::Why, by: f64) { todo!() }

/// (src/game/showwhy.ts:297 showWhyOf)
pub fn show_why_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::showwhy::Why> { todo!() }

/// (src/game/showwhy.ts:308 showWhySummary)
pub fn show_why_summary(g: &mut crate::game::Game) -> String { todo!() }
