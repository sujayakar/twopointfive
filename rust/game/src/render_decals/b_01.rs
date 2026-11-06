//! THE BRING-UP'S OWN PART of render_decals: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/decals.ts:178 Decals.clear)
pub fn decals_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/decals.ts:182 Decals.touch)
pub fn decals_touch(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's decals: `Game.decals` is not carried by the bridge and marks.ts's head says a run with them on is the run with them off).
/// NOT SETTLED: triage has marks.ts "unsure" because wet.ts:169 and wallwear.ts:93 read the list; neither is on a rule's path that this bring-up has met.   (render/decals.ts:153 Decals.add)
pub fn decals_add(g: &mut crate::game::Game, kind: crate::render_decals::DecalKind, pos: V3, normal: V3, size: Option<f64>, opts: Option<crate::render_decals::DecalOpts>) -> Option<crate::render_decals::Decal> { None }

/// A new ring of marks: the class's field initialisers (empty list, enabled, dirty, serial 0) with the device kept.   (src/render/decals.ts:145 Decals.constructor)
/// BOUNDARY: the host makes the marks' storage buffer here at the swap (makeBuffer(device, DECAL_BYTES, STORAGE | COPY_DST, 'decals')) and keeps it; `buf` stays the blank's Opaque, which nothing in the sim reads.
pub fn decals_new(g: &mut crate::game::Game, device: crate::Opaque) -> crate::render_decals::Decals {
    crate::render_decals::Decals { device, ..crate::render_decals::Decals::blank() }
}
