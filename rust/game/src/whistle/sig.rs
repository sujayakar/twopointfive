#![allow(unused_variables)]
//! Every function and method of `whistle`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/whistle.ts:71 whistleLines)
pub fn whistle_lines(g: &mut crate::game::Game, kind: Option<crate::whistle::WhistleLinesKind>) -> Vec<String> { todo!() }

/// (src/game/whistle.ts:89 mem)
pub fn mem<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::whistle::WhistleMem { todo!() }

/// (src/game/whistle.ts:95 glanceToward)
pub fn glance_toward(g: &mut crate::game::Game, gd: GuardId, at: V3, secs: Option<f64>) { todo!() }

/// (src/game/whistle.ts:97 whistleMem)
pub fn whistle_mem(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::whistle::WhistleMem> { todo!() }

/// (src/game/whistle.ts:102 blower)
pub fn blower<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::whistle::Blower { todo!() }

/// (src/game/whistle.ts:107 whistleReadyIn)
pub fn whistle_ready_in(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/whistle.ts:110 whistleKey)
pub fn whistle_key(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/whistle.ts:114 tryWhistle)
pub fn try_whistle(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/whistle.ts:135 mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, e: crate::game::GameEvent) -> V3 { todo!() }

/// (src/game/whistle.ts:138 canAnswer)
pub fn can_answer(g: &mut crate::game::Game, gd: GuardId, who: f64) -> bool { todo!() }

/// (src/game/whistle.ts:144 resolve)
pub fn resolve<'g>(g: &'g mut crate::game::Game, e: crate::game::GameEvent, keen: f64) -> &'g mut crate::whistle::Resolution { todo!() }

/// (src/game/whistle.ts:168 hearWhistle)
pub fn hear_whistle(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, keen: f64, small: &mut dyn FnMut(&mut crate::game::Game, V3)) { todo!() }

/// (src/game/whistle.ts:201 pick)
pub fn pick(g: &mut crate::game::Game, pool: &[String], gd: GuardId, key: crate::whistle::PickKey) -> String { todo!() }

/// (src/game/whistle.ts:209 whistleSoft)
pub fn whistle_soft(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/whistle.ts:216 whistlePatrolTick)
pub fn whistle_patrol_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/whistle.ts:229 saveWhistle)
pub fn save_whistle(g: &mut crate::game::Game) { todo!() }

/// (src/game/whistle.ts:234 loadWhistle)
pub fn load_whistle(g: &mut crate::game::Game) { todo!() }

/// (src/game/whistle.ts:241 resetWhistle)
pub fn reset_whistle(g: &mut crate::game::Game) { todo!() }

/// (src/game/whistle.ts:265 whistleSummary)
pub fn whistle_summary(g: &mut crate::game::Game) -> String { todo!() }
