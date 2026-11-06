#![allow(unused_variables)]
//! Every function and method of `debris`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/debris.ts:79 hash01)
pub fn hash01(x: f64) -> f64 { todo!() }

/// (src/game/debris.ts:86 Debris.constructor)
pub fn debris_new(g: &mut crate::game::Game) -> crate::debris::Debris { todo!() }

/// (src/game/debris.ts:89 Debris.cap)
pub fn debris_cap(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/debris.ts:90 Debris.moving)
pub fn debris_moving(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/debris.ts:94 Debris.spawn)
pub fn debris_spawn(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, w: V3, who: Option<f64>, level: Option<f64>) -> Option<DebrisChunkId> { todo!() }

/// (src/game/debris.ts:106 Debris.thrown)
pub fn debris_thrown(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, level: f64, who: f64) { todo!() }

/// (src/game/debris.ts:116 Debris.landings)
pub fn debris_landings(g: &mut crate::game::Game) { todo!() }

/// (src/game/debris.ts:141 Debris.spawnRaw)
pub fn debris_spawn_raw(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: BoxId, v: V3, w: V3, who: Option<f64>, told: Option<bool>) -> Option<DebrisChunkId> { todo!() }

/// (src/game/debris.ts:157 Debris.clear)
pub fn debris_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/debris.ts:158 Debris.boxes)
pub fn debris_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/debris.ts:163 Debris.impulse)
pub fn debris_impulse(g: &mut crate::game::Game, imp: crate::sheet_model::ImpulseLike, blocked: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, who: Option<f64>) -> f64 { todo!() }

/// (src/game/debris.ts:198 Debris.feet)
pub fn debris_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, list: Option<&[DebrisChunkId]>, book: Option<&mut crate::debris::FootBook>) -> f64 { todo!() }

/// (src/game/debris.ts:273 Debris.loose)
pub fn debris_loose(g: &mut crate::game::Game, box_: BoxId, lift: Option<f64>) -> DebrisChunkId { todo!() }

/// (src/game/debris.ts:275 Debris.footStep)
pub fn debris_foot_step(g: &mut crate::game::Game, c_2: DebrisChunkId, dt: f64) { todo!() }

/// (src/game/debris.ts:278 Debris.footGap)
pub fn debris_foot_gap(g: &mut crate::game::Game, b: BoxId, px: f64, pz: f64, d1x: f64, d1z: f64, e1: f64, e2: f64, radius: f64) -> crate::debris::DebrisFootGap { todo!() }

/// (src/game/debris.ts:295 Debris.slide)
pub fn debris_slide(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64) { todo!() }

/// (src/game/debris.ts:326 Debris.footIn)
pub fn debris_foot_in(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/debris.ts:343 Debris.update)
pub fn debris_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/debris.ts:363 Debris.step)
pub fn debris_step(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64) { todo!() }

/// (src/game/debris.ts:406 Debris.settleTick)
pub fn debris_settle_tick(g: &mut crate::game::Game, c_2: DebrisChunkId, h: f64, floor_n: Option<V3>) { todo!() }

/// (src/game/debris.ts:409 Debris.sleep)
pub fn debris_sleep(g: &mut crate::game::Game, c_2: DebrisChunkId, n: V3, on: Option<PropId>) { todo!() }

/// (src/game/debris.ts:422 Debris.land)
pub fn debris_land(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) { todo!() }

/// (src/game/debris.ts:430 Debris.summary)
pub fn debris_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/debris.ts:434 supportAlong)
pub fn support_along(g: &mut crate::game::Game, b: BoxId, n: V3) -> f64 { todo!() }

/// (src/game/debris.ts:440 evictable)
pub fn evictable(g: &mut crate::game::Game, c_2: DebrisChunkId) -> bool { todo!() }

/// (src/game/debris.ts:441 boundR)
pub fn bound_r(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/debris.ts:443 massOf)
pub fn mass_of(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/debris.ts:446 faceTilt)
pub fn face_tilt(g: &mut crate::game::Game, b: BoxId, n: V3) -> f64 { todo!() }

/// (src/game/debris.ts:451 layFlat)
pub fn lay_flat(g: &mut crate::game::Game, b: BoxId, n: V3, k: f64) { todo!() }

/// (src/game/debris.ts:467 poseKey)
pub fn pose_key(g: &mut crate::game::Game, p: PropId) -> f64 { todo!() }
