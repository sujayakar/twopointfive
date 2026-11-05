#![allow(unused_variables)]
//! Every function and method of `wounds`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wounds.ts:48 resetWoundStats)
pub fn reset_wound_stats() { todo!() }

/// (src/game/wounds.ts:51 segDist)
pub fn seg_dist(p1: V3, q1: V3, p2: V3, q2: V3) -> f64 { todo!() }

/// (src/game/wounds.ts:72 hitZone)
pub fn hit_zone(g: &mut crate::game::Game, c_2: CharId, ro: V3, rd: V3, t_hit: f64) -> crate::wounds::HitZone { todo!() }

/// (src/game/wounds.ts:84 roundDoes)
pub fn round_does(g: &mut crate::game::Game, gd: GuardId, ro: V3, rd: V3, t_hit: f64, hostage: bool, by_player: bool) -> crate::wounds::RoundDoes { todo!() }

/// (src/game/wounds.ts:101 woundDown)
pub fn wound_down(g: &mut crate::game::Game, gd: GuardId, dir: V3, zone: Option<crate::wounds::WoundDownZone>) { todo!() }

/// (src/game/wounds.ts:114 lay)
pub fn lay(g: &mut crate::game::Game, gd: GuardId, settled: bool, clock: Option<f64>) { todo!() }

/// (src/game/wounds.ts:125 finishWounded)
pub fn finish_wounded(g: &mut crate::game::Game, gd: GuardId, dir: V3, by_player: bool) { todo!() }

/// (src/game/wounds.ts:139 woundedMen)
pub fn wounded_men(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }
