//! THE BRING-UP'S OWN PART of snow_tracks: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:154 tellMess)
pub fn tell_mess(t: &mut crate::snow_tracks::SnowTracks) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:159 dirtied)
pub fn dirtied(t: &mut crate::snow_tracks::Stamp, x0: f64, z0: f64, x1: f64, z1: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:175 clearSnowTracks)
pub fn clear_snow_tracks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:188 stampMark)
pub fn stamp_mark(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, force: f64, fn_: &mut dyn FnMut(&mut crate::game::Game, crate::snow_tracks::Stamp, crate::snow_tracks::Stamp), kick: Option<crate::snow_tracks::StampMarkKick>, quick: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:211 formSnowTracks)
pub fn form_snow_tracks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:237 press)
pub fn press(t: &mut crate::snow_tracks::Stamp, ax: f64, az: f64, bx: f64, bz: f64, r: f64, r0: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:274 pressMessy)
pub fn press_messy(t: &mut crate::snow_tracks::Stamp, tc: &mut crate::snow_tracks::Stamp, cx: f64, cz: f64, ux: f64, uz: f64, wx: f64, wz: f64, bx: f64, bz: f64, m: f64, serial: f64, run: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:360 pressCrater)
pub fn press_crater(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, cx: f64, cz: f64, r0: f64, serial: f64, l: crate::snow_tracks::CraterLook) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:364 craterInto)
pub fn crater_into(t: &mut crate::snow_tracks::Stamp, tc: &mut crate::snow_tracks::Stamp, cx: f64, cz: f64, r0: f64, serial: f64, l: crate::snow_tracks::CraterLook) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:429 pressGroove)
pub fn press_groove(t: &mut crate::snow_tracks::Stamp, ax: f64, az: f64, bx: f64, bz: f64, r0: f64, r: f64, deep: f64, m: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:454 pressClods)
pub fn press_clods(t: &mut crate::snow_tracks::Stamp, cx: f64, cz: f64, n: f64, near: f64, far: f64, dx: f64, dz: f64, serial: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:475 updateSnowTracks)
pub fn update_snow_tracks(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:490 findUnseen)
pub fn find_unseen(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, fog_up: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:502 mark)
pub fn mark(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, k: f64, a: &[f64]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:511 lay)
pub fn lay(g: &mut crate::game::Game, t: &mut crate::snow_tracks::SnowTracks, k: f64, a: &[f64], live: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/snowTracks.ts:522 pressOf)
pub fn press_of(g: &mut crate::game::Game, c_2: CharId, t: &mut crate::snow_tracks::SnowTracks, field: &crate::render_snow_field::SnowField, dt: f64) {}
