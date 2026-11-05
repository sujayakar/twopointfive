//! THE BRING-UP'S OWN PART of blast_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:162 blastLookTick)
pub fn blast_look_tick(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:163 blastLookTickRaw)
pub fn blast_look_tick_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:190 chargeBoxes)
pub fn charge_boxes(g: &mut crate::game::Game, s: crate::blast_look::Seen, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:239 afterglow)
pub fn afterglow(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:280 doorBlast)
pub fn door_blast(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, d: crate::blast_look::DoorBlastD) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:287 burstOf)
pub fn burst_of(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, at: V3, r: crate::blast_look::STUNSPARKSFountain, k: crate::blast_look::BurstOfK) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:293 emit)
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:302 wallBlast)
pub fn wall_blast(g: &mut crate::game::Game, pl: PlayerId, p: &crate::render_particles::Particles, w: &crate::wallcharge::WallSpot) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastLook.ts:320 stunSparks)
pub fn stun_sparks(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, at: V3) {}
