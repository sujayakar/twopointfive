#![allow(unused_variables)]
//! Every function and method of `doorforce`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doorforce.ts:107 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doorforce.ts:108 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::doorforce::State { todo!() }

/// (src/game/doorforce.ts:114 manOf)
pub fn man_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::doorforce::Man { todo!() }

/// (src/game/doorforce.ts:115 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/doorforce.ts:116 log)
pub fn log(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/doorforce.ts:118 defender)
pub fn defender(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doorforce.ts:120 shut)
pub fn shut(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doorforce.ts:123 behind)
pub fn behind(g: &mut crate::game::Game, gd: GuardId, d: DoorId, from: V3, at: V3) -> bool { todo!() }

/// (src/game/doorforce.ts:131 doorBetween)
pub fn door_between(g: &mut crate::game::Game, gd: GuardId, from: V3, at: V3) -> Option<DoorId> { todo!() }

/// (src/game/doorforce.ts:138 doorHiding)
pub fn door_hiding(g: &mut crate::game::Game, gd: GuardId, at: V3) -> Option<DoorId> { todo!() }

/// (src/game/doorforce.ts:144 learn)
pub fn learn(g: &mut crate::game::Game, gd: GuardId, d: DoorId, at: V3, add: f64, seen: bool) { todo!() }

/// (src/game/doorforce.ts:154 doorForceHear)
pub fn door_force_hear(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, prop: crate::doorforce::DoorForceHearProp) { todo!() }

/// (src/game/doorforce.ts:188 doorForceOwnBoot)
pub fn door_force_own_boot(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent) -> bool { todo!() }

/// (src/game/doorforce.ts:195 doorForceTick)
pub fn door_force_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/doorforce.ts:223 drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/doorforce.ts:228 roundFor)
pub fn round_for(g: &mut crate::game::Game, gd: GuardId, b: &mut crate::doorforce::Belief) -> Option<V3> { todo!() }

/// (src/game/doorforce.ts:248 doorForce)
pub fn door_force(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::doorforce::DoorForce> { todo!() }

/// (src/game/doorforce.ts:312 kickStep)
pub fn kick_step(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::doorforce::KickStep> { todo!() }

/// (src/game/doorforce.ts:354 doorForceBelief)
pub fn door_force_belief(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> Option<crate::doorforce::DoorForceBelief> { todo!() }

/// (src/game/doorforce.ts:358 doorForceOn)
pub fn door_force_on(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doorforce::DoorForceOn> { todo!() }

/// (src/game/doorforce.ts:364 doorForceSaw)
pub fn door_force_saw(g: &mut crate::game::Game, gd: GuardId, at: V3, age: Option<f64>) { todo!() }

/// (src/game/doorforce.ts:366 doorForceReset)
pub fn door_force_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/doorforce.ts:368 doorForceSummary)
pub fn door_force_summary(g: &mut crate::game::Game) -> String { todo!() }
