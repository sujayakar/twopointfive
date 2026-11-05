#![allow(unused_variables)]
//! Every function and method of `rain_fx`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rainFx.ts:115 dripJob)
pub fn drip_job(boxes: f64, serial: f64, left: f64, hold: f64) -> crate::rain_fx::DripJob { todo!() }

/// (src/game/rainFx.ts:138 freshBook)
pub fn fresh_book() -> crate::rain_fx::RainState { todo!() }

/// (src/game/rainFx.ts:139 stateOf)
pub fn state_of(g: &mut crate::game::Game) -> crate::rain_fx::RainState { todo!() }

/// (src/game/rainFx.ts:141 rnd)
pub fn rnd(s: &mut crate::rain_fx::RainState) -> f64 { todo!() }

/// (src/game/rainFx.ts:142 nextSeed)
pub fn next_seed(s: &mut crate::rain_fx::RainState) -> f64 { todo!() }

/// (src/game/rainFx.ts:146 inRect)
pub fn in_rect(r: [f64; 4], x: f64, z: f64) -> bool { todo!() }

/// (src/game/rainFx.ts:148 outdoors)
pub fn outdoors(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/rainFx.ts:150 rimTaper)
pub fn rim_taper(r01: f64) -> f64 { todo!() }

/// (src/game/rainFx.ts:153 updateRainFx)
pub fn update_rain_fx(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/rainFx.ts:265 dripLife)
pub fn drip_life(fall: f64) -> f64 { todo!() }

/// (src/game/rainFx.ts:266 num)
pub fn num(v: crate::Opaque, d: f64) -> f64 { todo!() }

/// (src/game/rainFx.ts:271 buildDripEdges)
pub fn build_drip_edges(g: &mut crate::game::Game, st: &mut crate::rain_fx::RainState) { todo!() }

/// (src/game/rainFx.ts:275 setDripEdges)
pub fn set_drip_edges(st: &mut crate::rain_fx::RainState, out: &[crate::rain_fx::DripPoint], boxes: f64) { todo!() }

/// (src/game/rainFx.ts:282 paceDripEdges)
pub fn pace_drip_edges(g: &mut crate::game::Game, st: &mut crate::rain_fx::RainState, now: bool) { todo!() }

/// (src/game/rainFx.ts:292 dripWalk)
pub fn drip_walk(g: &mut crate::game::Game, job: &mut crate::rain_fx::DripJob) -> bool { todo!() }

/// (src/game/rainFx.ts:341 rainSummary)
pub fn rain_summary(g: &mut crate::game::Game) -> String { todo!() }
