#![allow(unused_variables)]
//! Every function and method of `sense`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/sense.ts:93 erf)
pub fn erf(x: f64) -> f64 { todo!() }

/// (src/game/sense.ts:115 SmokeGrid.constructor)
pub fn smoke_grid_new(g: &mut crate::game::Game, sim: crate::Opaque) -> crate::sense::SmokeGrid { todo!() }

/// (src/game/sense.ts:118 SmokeGrid.make)
pub fn smoke_grid_make(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:123 SmokeGrid.free)
pub fn smoke_grid_free(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:125 SmokeGrid.row0)
pub fn smoke_grid_row0(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sense.ts:127 SmokeGrid.at)
pub fn smoke_grid_at(g: &mut crate::game::Game, i: f64) -> f64 { todo!() }

/// (src/game/sense.ts:130 SmokeGrid.row)
pub fn smoke_grid_row(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/sense.ts:133 SmokeGrid.smoky)
pub fn smoke_grid_smoky(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/sense.ts:134 SmokeGrid.emit)
pub fn smoke_grid_emit(g: &mut crate::game::Game, x: f64, y: f64, z: f64, radius: f64, rate: f64, ttl: f64) { todo!() }

/// (src/game/sense.ts:141 SmokeGrid.step)
pub fn smoke_grid_step(g: &mut crate::game::Game, dt: f64, lamps: Option<&mut dyn FnMut(&mut crate::game::Game, crate::scene_collision::StaticCollision) -> bool>, wind: Option<&mut dyn FnMut(&mut crate::game::Game) -> Option<crate::scene_collision::GridWind>>) { todo!() }

/// (src/game/sense.ts:150 SmokeGrid.clear)
pub fn smoke_grid_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:153 SmokeGrid.renew)
pub fn smoke_grid_renew(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:156 SmokeGrid.along)
pub fn smoke_grid_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sense.ts:160 SmokeGrid.glowAt)
pub fn smoke_grid_glow_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/sense.ts:205 CpuSmoke.emit)
pub fn cpu_smoke_emit(g: &mut crate::game::Game, e: crate::smoke_types::SmokeEmitter) { todo!() }

/// (src/game/sense.ts:220 CpuSmoke.budget)
pub fn cpu_smoke_budget(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sense.ts:221 CpuSmoke.step)
pub fn cpu_smoke_step(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/sense.ts:231 CpuSmoke.clear)
pub fn cpu_smoke_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/sense.ts:234 CpuSmoke.trans)
pub fn cpu_smoke_trans(g: &mut crate::game::Game, a: V3, b: V3, blocked: &mut dyn FnMut(&mut crate::game::Game, V3, V3) -> bool) -> f64 { todo!() }

/// (src/game/sense.ts:253 CpuSmoke.along)
pub fn cpu_smoke_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sense.ts:259 CpuSmoke.alongIn)
pub fn cpu_smoke_along_in(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sense.ts:276 ownSmoke)
pub fn own_smoke(g: &mut crate::game::Game) -> crate::FnField { todo!() }
