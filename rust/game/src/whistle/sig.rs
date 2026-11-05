#![allow(unused_variables)]
//! Every function and method of `whistle`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/whistle.ts:70 whistleLines)
pub fn whistle_lines(g: &mut crate::game::Game, kind: Option<crate::whistle::WhistleLinesKind>) -> Vec<String> { todo!() }

/// (src/game/whistle.ts:88 mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::whistle::WhistleMem { todo!() }

/// (src/game/whistle.ts:94 glanceToward)
pub fn glance_toward(g: &mut crate::game::Game, gd: GuardId, at: V3, secs: Option<f64>) { todo!() }

/// (src/game/whistle.ts:96 whistleMem)
pub fn whistle_mem(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::whistle::WhistleMem2> { todo!() }

/// (src/game/whistle.ts:101 blower)
pub fn blower<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::whistle::Blower { todo!() }

/// (src/game/whistle.ts:106 whistleReadyIn)
pub fn whistle_ready_in(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/whistle.ts:109 whistleKey)
pub fn whistle_key(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/whistle.ts:113 tryWhistle)
pub fn try_whistle(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/whistle.ts:134 mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, e: crate::game::GameEvent) -> V3 { todo!() }

/// (src/game/whistle.ts:137 canAnswer)
pub fn can_answer(g: &mut crate::game::Game, gd: GuardId, who: f64) -> bool { todo!() }

/// (src/game/whistle.ts:143 resolve)
pub fn resolve<'g>(g: &'g mut crate::game::Game, e: crate::game::GameEvent, keen: f64) -> &'g mut crate::whistle::Resolution { todo!() }

/// (src/game/whistle.ts:167 hearWhistle)
pub fn hear_whistle(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, keen: f64, small: &mut dyn FnMut(&mut crate::game::Game, V3)) { todo!() }

/// (src/game/whistle.ts:200 pick)
pub fn pick(g: &mut crate::game::Game, pool: &[String], gd: GuardId, key: crate::whistle::PickKey) -> String { todo!() }

/// (src/game/whistle.ts:208 whistleSoft)
pub fn whistle_soft(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/whistle.ts:215 whistlePatrolTick)
pub fn whistle_patrol_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/whistle.ts:228 saveWhistle)
pub fn save_whistle(g: &mut crate::game::Game) { todo!() }

/// (src/game/whistle.ts:233 loadWhistle)
pub fn load_whistle(g: &mut crate::game::Game) { todo!() }

/// (src/game/whistle.ts:240 resetWhistle)
pub fn reset_whistle(g: &mut crate::game::Game) { todo!() }

/// (src/game/whistle.ts:264 whistleSummary)
pub fn whistle_summary(g: &mut crate::game::Game) -> String { todo!() }
