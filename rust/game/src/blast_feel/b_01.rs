//! THE BRING-UP'S OWN PART of blast_feel: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:84 loadFeel)
pub fn load_feel(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:105 later)
pub fn later(g: &mut crate::game::Game, pl: PlayerId, secs: f64, run: &mut dyn FnMut(&mut crate::game::Game)) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:129 blastFeelTick)
pub fn blast_feel_tick(g: &mut crate::game::Game, cam: CameraId) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:130 blastFeelTickRaw)
pub fn blast_feel_tick_raw(g: &mut crate::game::Game, cam: CameraId) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:162 report)
pub fn report(g: &mut crate::game::Game, pl: PlayerId, at: V3) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:175 emit)
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:181 fireball)
pub fn fireball(g: &mut crate::game::Game, pl: PlayerId, w: &crate::wallcharge::WallSpot, open: bool, n: V3) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:196 ringStage)
pub fn ring_stage(g: &mut crate::game::Game, pl: PlayerId, k: f64, faces: &[crate::blast_feel::Face]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/blastFeel.ts:213 ceiling)
pub fn ceiling(g: &mut crate::game::Game, pl: PlayerId, faces: &[crate::blast_feel::Face]) {}
