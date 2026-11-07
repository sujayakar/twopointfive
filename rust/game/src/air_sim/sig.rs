#![allow(unused_variables)]
//! Every function and method of `air_sim`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/airSim.ts:56 blankStats)
pub fn blank_stats(w: f64, h: f64, why: &str) -> crate::air_sim::AirSimStats { todo!() }

/// (src/game/airSim.ts:57 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::air_sim::AirState { todo!() }

/// (src/game/airSim.ts:69 airSimStats)
pub fn air_sim_stats(g: &mut crate::game::Game) -> crate::air_sim::AirSimStats { todo!() }

/// (src/game/airSim.ts:74 airLive)
pub fn air_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/airSim.ts:77 airResend)
pub fn air_resend(g: &mut crate::game::Game) { todo!() }

/// (src/game/airSim.ts:78 airWant)
pub fn air_want(g: &mut crate::game::Game) { todo!() }

/// (src/game/airSim.ts:82 airPuff)
pub fn air_puff(g: &mut crate::game::Game, x: f64, z: f64, out: f64, sigma: f64, secs: f64, rate: Option<f64>) { todo!() }

/// (src/game/airSim.ts:92 stands)
pub fn stands(g: &mut crate::game::Game, b: BoxId, bi: f64) -> bool { todo!() }

/// (src/game/airSim.ts:101 stamp)
pub fn stamp(m: &mut Vec<u8>, w: f64, h: f64, cx: f64, cz: f64, hx: f64, hz: f64, yaw: f64) { todo!() }

/// (src/game/airSim.ts:114 redraw)
pub fn redraw(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/airSim.ts:148 sources)
pub fn sources(g: &mut crate::game::Game, ambient: &mut [f64; 2]) -> f64 { todo!() }

/// (src/game/airSim.ts:185 attachAir)
pub fn attach_air(g: &mut crate::game::Game) { todo!() }

/// (src/game/airSim.ts:201 updateAirSim)
pub fn update_air_sim(g: &mut crate::game::Game, dt: f64) { todo!() }
