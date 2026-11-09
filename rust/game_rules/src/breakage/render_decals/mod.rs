#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
//! THE BRING-UP'S OWN PART of render_decals: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)

pub use ::game_types::render_decals::types;
pub use types::*;
pub mod set;
pub use set::*;
use crate::game::Game;
use crate::v3::{self, Quat, V3};

/// A rule asks for a mark (set.rs says what becomes of it). (src/render/decals.ts:153 Decals.add)
pub fn decals_add(g: &mut crate::game::Game, kind: crate::render_decals::DecalKind, pos: V3, normal: V3, size: Option<f64>, opts: Option<crate::render_decals::DecalOpts>) {
    crate::render_decals::mark_asked(g, crate::render_decals::Ask::Add(crate::out_events::OutDecalsAdd { kind, pos, normal, size, opts }));
}

/// (src/render/decals.ts:178 Decals.clear)
pub fn decals_clear(g: &mut crate::game::Game) { crate::render_decals::marks_clear(g); }

/// A new ring of marks: the class's field initialisers (empty list, enabled, dirty, serial 0) with the device kept.   (src/render/decals.ts:145 Decals.constructor)
/// BOUNDARY: the host makes the marks' storage buffer here at the swap (makeBuffer(device, DECAL_BYTES, STORAGE | COPY_DST, 'decals')) and keeps it; `buf` stays the blank's Opaque, which nothing in the sim reads.
pub fn decals_new(g: &crate::game::Game, device: crate::Opaque) -> crate::render_decals::Decals {
    crate::render_decals::Decals { ..crate::render_decals::Decals::blank() }
}
