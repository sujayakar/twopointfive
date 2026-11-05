#![allow(unused_variables)]
//! Every function and method of `squadsmoke`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/squadsmoke.ts:64 squadSmokeLeft)
pub fn squad_smoke_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/squadsmoke.ts:65 squadSmokeBrought)
pub fn squad_smoke_brought(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/squadsmoke.ts:69 squadSmokeStage)
pub fn squad_smoke_stage(g: &mut crate::game::Game, cans: f64) { todo!() }

/// (src/game/squadsmoke.ts:82 squadSmokeRest)
pub fn squad_smoke_rest(from: V3, sol: &crate::throwables::ThrowSolution, roll_k: Option<f64>) -> V3 { todo!() }

/// (src/game/squadsmoke.ts:87 runClear)
pub fn run_clear(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/squadsmoke.ts:97 squadHandFree)
pub fn squad_hand_free(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/squadsmoke.ts:101 squadArcTo)
pub fn squad_arc_to(g: &mut crate::game::Game, gd: GuardId, at: V3, roll_k: Option<f64>, stood: Option<V3>) -> Option<crate::squadsmoke::SquadArcTo> { todo!() }

/// (src/game/squadsmoke.ts:120 squadSmokeThrower)
pub fn squad_smoke_thrower(g: &mut crate::game::Game, men: &[GuardId], at: V3) -> Option<crate::squadsmoke::SquadSmokeThrower> { todo!() }

/// (src/game/squadsmoke.ts:130 row)
pub fn row(g: &mut crate::game::Game) -> OrdersOrderDefId { todo!() }

/// (src/game/squadsmoke.ts:148 notes)
pub fn notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/squadsmoke.ts:152 squadSmokeKey)
pub fn squad_smoke_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/squadsmoke.ts:157 squadSmokeSummary)
pub fn squad_smoke_summary(g: &mut crate::game::Game) -> String { todo!() }
