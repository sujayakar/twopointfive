//! THE BRING-UP'S OWN PART of render_decals: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};




/// A new ring of marks: the class's field initialisers (empty list, enabled, dirty, serial 0) with the device kept.   (src/render/decals.ts:145 Decals.constructor)
/// BOUNDARY: the host makes the marks' storage buffer here at the swap (makeBuffer(device, DECAL_BYTES, STORAGE | COPY_DST, 'decals')) and keeps it; `buf` stays the blank's Opaque, which nothing in the sim reads.
pub fn decals_new(g: &crate::game::Game, device: crate::Opaque) -> crate::render_decals::Decals {
    crate::render_decals::Decals { ..crate::render_decals::Decals::blank() }
}
