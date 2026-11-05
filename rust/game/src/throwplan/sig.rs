#![allow(unused_variables)]
//! Every function and method of `throwplan`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/throwplan.ts:31 throwFlight)
pub fn throw_flight(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind, from: V3, v0: V3, step: Option<f64>) -> crate::throwplan::ThrowFlight { todo!() }

/// (src/game/throwplan.ts:72 throwWalls)
pub fn throw_walls(g: &mut crate::game::Game, at: V3) -> Vec<V3> { todo!() }

/// (src/game/throwplan.ts:82 handAt)
pub fn hand_at(g: &mut crate::game::Game, gd: GuardId, p: V3, aim: V3) -> Option<V3> { todo!() }

/// (src/game/throwplan.ts:89 throwHits)
pub fn throw_hits(g: &mut crate::game::Game, end: V3, at: V3, slack: Option<f64>) -> bool { todo!() }

/// (src/game/throwplan.ts:96 throwTo)
pub fn throw_to(g: &mut crate::game::Game, gd: GuardId, at: V3, kind: crate::squadthrow::SquadThrowKind, stood: Option<V3>, walls: Option<&[V3]>, lim: Option<&mut crate::throwplan::ThrowToLim>) -> Option<crate::throwplan::ThrowTo> { todo!() }
