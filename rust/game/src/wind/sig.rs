#![allow(unused_variables)]
//! Every function and method of `wind`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wind.ts:40 shape)
pub fn shape(g: &mut crate::game::Game, i: f64) -> f64 { todo!() }

/// (src/game/wind.ts:65 saveWind)
pub fn save_wind(g: &mut crate::game::Game) { todo!() }

/// (src/game/wind.ts:66 resetWindKnobs)
pub fn reset_wind_knobs(g: &mut crate::game::Game) { todo!() }

/// (src/game/wind.ts:68 windOf)
pub fn wind_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wind.ts:72 airOut)
pub fn air_out(out: &mut V3) -> V3 { todo!() }

/// (src/game/wind.ts:74 jetVel)
pub fn jet_vel(pos: V3, dir: V3, speed: f64, reach: f64, tan_spread: f64, r0: f64, d_max: f64, floor_y: f64, p: V3, out: Option<&mut V3>) -> V3 { todo!() }

/// (src/game/wind.ts:79 inRect)
pub fn in_rect(x: f64, z: f64, r: &[f64]) -> bool { todo!() }

/// (src/game/wind.ts:81 stormVel)
pub fn storm_vel(wind: V3, interior: &[f64], annex: &[f64], p: V3, out: Option<&mut V3>) -> V3 { todo!() }

/// (src/game/wind.ts:87 stormVelRoofs)
pub fn storm_vel_roofs(wind: V3, roofs: &[Vec<f64>], p: V3, out: Option<&mut V3>) -> V3 { todo!() }

/// (src/game/wind.ts:95 vnoise)
pub fn vnoise(seed: f64, x: f64) -> f64 { todo!() }

/// (src/game/wind.ts:104 ventDue)
pub fn vent_due(v: &crate::scene_level::VentDef, t: f64) -> bool { todo!() }

/// (src/game/wind.ts:125 Wind.forParticles)
pub fn wind_for_particles(g: &mut crate::game::Game) -> Vec<WindWindSourceId> { todo!() }

/// (src/game/wind.ts:129 Wind.constructor)
pub fn wind_new(g: &mut crate::game::Game) -> crate::wind::Wind { todo!() }

/// (src/game/wind.ts:148 Wind.doorSource)
pub fn wind_door_source(g: &mut crate::game::Game, d: DoorId, inward: V3) -> WindWindSourceId { todo!() }

/// (src/game/wind.ts:161 Wind.reroof)
pub fn wind_reroof(g: &mut crate::game::Game, roofs: Option<&[[f64; 4]]>, own: Option<bool>) { todo!() }

/// (src/game/wind.ts:187 Wind.syncField)
pub fn wind_sync_field(g: &mut crate::game::Game) { todo!() }

/// (src/game/wind.ts:191 Wind.fieldRoofs)
pub fn wind_field_roofs(g: &mut crate::game::Game) -> Option<Vec<Vec<f64>>> { todo!() }

/// (src/game/wind.ts:194 Wind.stormShare)
pub fn wind_storm_share(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/game/wind.ts:196 Wind.shareUnder)
pub fn wind_share_under(g: &mut crate::game::Game, x: f64, z: f64, roofs: Option<&[Vec<f64>]>) -> f64 { todo!() }

/// (src/game/wind.ts:206 Wind.stormDrift)
pub fn wind_storm_drift(g: &mut crate::game::Game) -> Option<crate::scene_collision::GridWind> { todo!() }

/// (src/game/wind.ts:219 Wind.blows)
pub fn wind_blows(g: &mut crate::game::Game, session: bool) -> bool { todo!() }

/// (src/game/wind.ts:225 Wind.stormWind)
pub fn wind_storm_wind(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/wind.ts:233 Wind.update)
pub fn wind_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/wind.ts:286 Wind.at)
pub fn wind_at(g: &mut crate::game::Game, p: V3, out: Option<&mut V3>) -> V3 { todo!() }

/// (src/game/wind.ts:297 Wind.byName)
pub fn wind_by_name(g: &mut crate::game::Game, name: &str) -> Option<WindWindSourceId> { todo!() }

/// (src/game/wind.ts:300 Wind.fresh)
pub fn wind_fresh(g: &mut crate::game::Game) { todo!() }

/// (src/game/wind.ts:302 Wind.describe)
pub fn wind_describe(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/wind.ts:303 Wind.summary)
pub fn wind_summary(g: &mut crate::game::Game) -> String { todo!() }
