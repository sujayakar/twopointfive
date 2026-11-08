//! THE BRING-UP'S OWN PART of thinline: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `b` is a thin line along its local z: its true half-width, the pixels its half-width must cover, what it emits at that true width. The note stands
/// beside the box (the TypeScript's side table, keyed by the box) and goes when the box's row is freed (`thin_forget`). Picture only: nothing in a
/// step reads it; a host that draws does (which boxes are a laser's beam or a tracer's streak).   (thinline.ts:17 thinNote)
pub fn thin_note(g: &mut crate::game::Game, b: AnyBoxId, hw0: f64, k: f64, r: f64, g_2: f64, bl: f64) -> AnyBoxId {
    g.m.thinline.thin.set(b, crate::thinline::Thin { hw0, k, e: [r, g_2, bl] });
    b
}
/// A box's row is free again: what was noted of the box that stood in it is gone with it (the TypeScript's tables hold their keys weakly).
pub fn thin_forget(g: &mut crate::game::Game, b: AnyBoxId) {
    if !g.m.thinline.thin.is_empty() { g.m.thinline.thin.delete(b); }
    if !g.m.thinline.air.is_empty() { g.m.thinline.air.delete(b); }
}
/// BOUNDARY NO-OP (thinline.ts:26): no line was noted, so none is refitted: the list stands
pub fn thin_fit(g: &crate::game::Game, list: &[AnyBoxId], px: f64) -> Option<Vec<AnyBoxId>> { None }
/// `line` (already noted) is a weapon laser's beam and `bead` the dot where it lands, or none: a host may draw the pair as light in the air.   (thinline.ts:44 thinAir)
pub fn thin_air(g: &mut crate::game::Game, line: AnyBoxId, bead: Option<AnyBoxId>) -> AnyBoxId { g.m.thinline.air.set(line, bead); line }
