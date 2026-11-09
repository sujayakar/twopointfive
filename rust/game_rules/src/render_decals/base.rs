//! What the parts of `render_decals` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// A rule asks for a mark (set.rs says what becomes of it). (src/render/decals.ts:153 Decals.add)
pub fn decals_add(g: &mut crate::game::Game, kind: crate::render_decals::DecalKind, pos: V3, normal: V3, size: Option<f64>, opts: Option<crate::render_decals::DecalOpts>) {
    crate::render_decals::mark_asked(g, crate::render_decals::Ask::Add(crate::out_events::OutDecalsAdd { kind, pos, normal, size, opts }));
}

/// (src/render/decals.ts:178 Decals.clear)
pub fn decals_clear(g: &mut crate::game::Game) { crate::render_decals::marks_clear(g); }
