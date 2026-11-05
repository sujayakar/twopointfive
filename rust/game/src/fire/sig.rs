#![allow(unused_variables)]
//! Every function and method of `fire`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/fire.ts:201 fits)
pub fn fits(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/fire.ts:204 axisAt)
pub fn axis_at(g: &mut crate::game::Game, up: f64, wind: [f64; 2]) -> [f64; 4] { todo!() }

/// (src/game/fire.ts:224 fresh)
pub fn fresh() -> crate::fire::FireStats { todo!() }

/// (src/game/fire.ts:225 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::fire::FireStats { todo!() }

/// (src/game/fire.ts:228 fireStats)
pub fn fire_stats(g: &mut crate::game::Game) -> crate::fire::FireStats { todo!() }

/// (src/game/fire.ts:230 fireShown)
pub fn fire_shown(g: &mut crate::game::Game) -> Option<crate::fire::FireShown2> { todo!() }

/// (src/game/fire.ts:233 h01)
pub fn h01(x: f64) -> f64 { todo!() }

/// (src/game/fire.ts:239 isFire)
pub fn is_fire(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/fire.ts:245 fireSeed)
pub fn fire_seed(g: &mut crate::game::Game, t: FixtureId) -> f64 { todo!() }

/// (src/game/fire.ts:247 firePops)
pub fn fire_pops(g: &mut crate::game::Game, seed: f64, time: f64, back: f64) -> Vec<crate::fire::FirePopsItem> { todo!() }

/// (src/game/fire.ts:257 fireFlicker)
pub fn fire_flicker(g: &mut crate::game::Game, seed: f64, time: f64) -> f64 { todo!() }

/// (src/game/fire.ts:269 mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, t: FixtureId) -> crate::fire::MouthOf { todo!() }

/// (src/game/fire.ts:276 heatOf)
pub fn heat_of(g: &mut crate::game::Game, t: FixtureId) -> f64 { todo!() }

/// (src/game/fire.ts:277 burns)
pub fn burns(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/fire.ts:279 windAt)
pub fn wind_at(g: &mut crate::game::Game, mouth: V3) -> crate::fire::WindAt { todo!() }

/// (src/game/fire.ts:284 lookRow)
pub fn look_row(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/fire.ts:287 fireHolds)
pub fn fire_holds(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/fire.ts:292 fireStruck)
pub fn fire_struck(g: &mut crate::game::Game, t: FixtureId, p: V3, ix: f64) -> bool { todo!() }

/// (src/game/fire.ts:298 fireStir)
pub fn fire_stir(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/fire.ts:308 held)
pub fn held(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/fire.ts:316 plain)
pub fn plain(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/fire.ts:321 updateFire)
pub fn update_fire(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/fire.ts:412 fireDraws)
pub fn fire_draws(g: &mut crate::game::Game) { todo!() }

/// (src/game/fire.ts:434 fireSummary)
pub fn fire_summary(g: &mut crate::game::Game) -> String { todo!() }
