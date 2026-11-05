#![allow(unused_variables)]
//! Every function and method of `sense`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/sense.ts:109 SmokeGrid.constructor)
pub fn smoke_grid_new(g: &mut crate::game::Game, sim: crate::Opaque) -> crate::sense::SmokeGrid { todo!() }

/// (src/game/sense.ts:119 SmokeGrid.row0)
pub fn smoke_grid_row0(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sense.ts:121 SmokeGrid.at)
pub fn smoke_grid_at(g: &mut crate::game::Game, i: f64) -> f64 { todo!() }

/// (src/game/sense.ts:124 SmokeGrid.row)
pub fn smoke_grid_row(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/sense.ts:127 SmokeGrid.smoky)
pub fn smoke_grid_smoky(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/sense.ts:128 SmokeGrid.emit)
pub fn smoke_grid_emit(g: &mut crate::game::Game, x: f64, y: f64, z: f64, radius: f64, rate: f64, ttl: f64) { todo!() }

/// (src/game/sense.ts:136 SmokeGrid.step)
pub fn smoke_grid_step(g: &mut crate::game::Game, dt: f64, lamps: bool, wind: Option<&mut dyn FnMut(&mut crate::game::Game) -> Option<crate::scene_collision::GridWind>>) { todo!() }

/// (src/game/sense.ts:145 SmokeGrid.clear)
pub fn smoke_grid_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:148 SmokeGrid.renew)
pub fn smoke_grid_renew(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:151 SmokeGrid.along)
pub fn smoke_grid_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sense.ts:155 SmokeGrid.glowAt)
pub fn smoke_grid_glow_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/sense.ts:200 CpuSmoke.emit)
pub fn cpu_smoke_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) { todo!() }

/// (src/game/sense.ts:215 CpuSmoke.budget)
pub fn cpu_smoke_budget(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sense.ts:216 CpuSmoke.step)
pub fn cpu_smoke_step(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/sense.ts:226 CpuSmoke.clear)
pub fn cpu_smoke_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:229 CpuSmoke.along)
pub fn cpu_smoke_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sense.ts:237 CpuSmoke.blobsAlong)
pub fn cpu_smoke_blobs_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sense.ts:242 CpuSmoke.blobsAsk)
pub fn cpu_smoke_blobs_ask(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }


/// INVENTED, in place of SimState.settle for this file's one tenant (sense.ts:177-180; settle is not in the game): as a tick opens the grid comes, empty, or goes, by SENSE's switches. Wants = cpu && grid && the module runs; has = g.cpu_smoke.grid.is_some(). Called first in game::game_sense_step.
pub fn settle_grid(g: &mut crate::game::Game) { todo!() }
