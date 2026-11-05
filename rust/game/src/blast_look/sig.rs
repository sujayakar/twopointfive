#![allow(unused_variables)]
//! Every function and method of `blast_look`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/blastLook.ts:151 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::blast_look::State { todo!() }

/// (src/game/blastLook.ts:153 hashU)
pub fn hash_u(n: f64) -> f64 { todo!() }

/// (src/game/blastLook.ts:155 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/blastLook.ts:156 thick)
pub fn thick(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { todo!() }

/// (src/game/blastLook.ts:157 near)
pub fn near(a: V3, b: V3, m: f64) -> bool { todo!() }

/// (src/game/blastLook.ts:159 burstAt)
pub fn burst_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 { todo!() }

/// (src/game/blastLook.ts:162 blastLookTick)
pub fn blast_look_tick(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/blastLook.ts:163 blastLookTickRaw)
pub fn blast_look_tick_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/blastLook.ts:184 runOf)
pub fn run_of(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { todo!() }

/// (src/game/blastLook.ts:190 chargeBoxes)
pub fn charge_boxes(g: &mut crate::game::Game, s: crate::blast_look::Seen, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/blastLook.ts:216 holed)
pub fn holed(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/blastLook.ts:219 flashOf)
pub fn flash_of<'g>(g: &'g mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<&'g mut LightId> { todo!() }

/// (src/game/blastLook.ts:225 LUM)
pub fn lum(r: f64, g_2: f64, b: f64) -> f64 { todo!() }

/// (src/game/blastLook.ts:229 nudge)
pub fn nudge(g: &mut crate::game::Game, x: f64, up: bool) -> f64 { todo!() }

/// (src/game/blastLook.ts:230 warmOf)
pub fn warm_of(g: &mut crate::game::Game, c_2: V3) -> V3 { todo!() }

/// (src/game/blastLook.ts:239 afterglow)
pub fn afterglow(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) { todo!() }

/// (src/game/blastLook.ts:246 blastSparks)
pub fn blast_sparks(g: &mut crate::game::Game, p: &mut crate::render_particles::Particles, at: V3) -> bool { todo!() }

/// (src/game/blastLook.ts:258 flashUnder)
pub fn flash_under(g: &mut crate::game::Game, at: V3, born: bool) -> bool { todo!() }

/// (src/game/blastLook.ts:261 canAt)
pub fn can_at(g: &mut crate::game::Game, at: V3) -> bool { todo!() }

/// (src/game/blastLook.ts:268 doorChargeAt)
pub fn door_charge_at(g: &mut crate::game::Game, at: V3, dt: f64) -> Option<crate::blast_look::DoorChargeAt> { todo!() }

/// (src/game/blastLook.ts:280 doorBlast)
pub fn door_blast(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, d: crate::blast_look::DoorBlastD) { todo!() }

/// (src/game/blastLook.ts:287 burstOf)
pub fn burst_of(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, at: V3, r: crate::blast_look::STUNSPARKSFountain, k: crate::blast_look::BurstOfK) { todo!() }

/// (src/game/blastLook.ts:293 emit)
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) { todo!() }

/// (src/game/blastLook.ts:299 scale)
pub fn scale(g: &mut crate::game::Game) -> crate::blast_look::BurstOfK { todo!() }

/// (src/game/blastLook.ts:302 wallBlast)
pub fn wall_blast(g: &mut crate::game::Game, pl: PlayerId, p: &crate::render_particles::Particles, w: &crate::wallcharge::WallSpot) { todo!() }

/// (src/game/blastLook.ts:320 stunSparks)
pub fn stun_sparks(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, at: V3) { todo!() }
