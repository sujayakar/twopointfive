#![allow(unused_variables)]
//! Every function and method of `snow_fx`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/snowFx.ts:154 freshBook)
pub fn fresh_book() -> crate::snow_fx::SnowState { todo!() }

/// (src/game/snowFx.ts:157 stateOf)
pub fn state_of(g: &mut crate::game::Game) -> crate::snow_fx::SnowState { todo!() }

/// (src/game/snowFx.ts:158 nextSeed)
pub fn next_seed(s: &mut crate::snow_fx::SnowState) -> f64 { todo!() }

/// (src/game/snowFx.ts:161 nextWispSeed)
pub fn next_wisp_seed(s: &mut crate::snow_fx::SnowState) -> f64 { todo!() }

/// (src/game/snowFx.ts:162 nextGrainSeed)
pub fn next_grain_seed(s: &mut crate::snow_fx::SnowState) -> f64 { todo!() }

/// (src/game/snowFx.ts:167 grainsOn)
pub fn grains_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snowFx.ts:171 oneAirOn)
pub fn one_air_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snowFx.ts:174 wispsOn)
pub fn wisps_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snowFx.ts:207 flakeOpen)
pub fn flake_open(roofed: &mut dyn FnMut(f64, f64) -> bool, x: f64, z: f64, past: f64) -> bool { todo!() }

/// (src/game/snowFx.ts:213 pastWorld)
pub fn past_world(x: f64, z: f64) -> bool { todo!() }

/// (src/game/snowFx.ts:219 flakeTaper)
pub fn flake_taper(r01: f64, x: f64, z: f64, past: f64, sooner: Option<[f64; 2]>) -> f64 { todo!() }

/// (src/game/snowFx.ts:224 snowPastM)
pub fn snow_past_m(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/snowFx.ts:225 snowing)
pub fn snowing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/snowFx.ts:228 num)
pub fn num(v: crate::Opaque, d: f64) -> f64 { todo!() }

/// (src/game/snowFx.ts:229 rimTaper)
pub fn rim_taper(r01: f64) -> f64 { todo!() }

/// (src/game/snowFx.ts:232 updateSnowFx)
pub fn update_snow_fx(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/snowFx.ts:335 markKicks)
pub fn mark_kicks(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, room: f64, size: f64, size_jit: f64, cx: f64, cz: f64, r: f64) -> f64 { todo!() }

/// (src/game/snowFx.ts:373 bootKicks)
pub fn boot_kicks(g: &mut crate::game::Game, kicks: &[crate::snow_tracks::SnowKick], seen: f64, amount: f64) -> f64 { todo!() }

/// (src/game/snowFx.ts:384 kicksDue)
pub fn kicks_due(t: &crate::snow_fx::KicksDueT, b: &crate::snow_fx::KicksDueB) -> Vec<crate::snow_tracks::SnowKick> { todo!() }

/// (src/game/snowFx.ts:394 wispLookByte)
pub fn wisp_look_byte() -> f64 { todo!() }

/// (src/game/snowFx.ts:399 updateWisps)
pub fn update_wisps(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, dt: f64, cx: f64, cz: f64, r: f64, area: f64, room: f64, max: Option<f64>, one_air: Option<bool>) { todo!() }

/// (src/game/snowFx.ts:420 oneAirCfg)
pub fn one_air_cfg(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, on: bool) { todo!() }

/// (src/game/snowFx.ts:430 grainCap)
pub fn grain_cap() -> f64 { todo!() }

/// (src/game/snowFx.ts:432 grainsOff)
pub fn grains_off(st: &mut crate::snow_fx::SnowState, p: Option<&mut crate::render_particles::Particles>, why: &str) { todo!() }

/// (src/game/snowFx.ts:442 updateGrains)
pub fn update_grains(g: &mut crate::game::Game, st: &mut crate::snow_fx::SnowState, pp: &mut crate::render_particles::Particles, dt: f64, cx: f64, cz: f64, room: f64, slot_life: f64) { todo!() }
