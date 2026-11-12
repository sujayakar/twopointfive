pub use ::game_types::render_decals::types;
pub use types::*;
pub mod set;
pub use set::*;
use crate::v3::V3;

/// A rule asks for a mark (set.rs says what becomes of it).
pub fn decals_add(g: &mut crate::game::Game, kind: crate::render_decals::DecalKind, pos: V3, normal: V3, size: Option<f64>, opts: Option<crate::render_decals::DecalOpts>) {
    crate::render_decals::mark_asked(g, crate::render_decals::Ask::Add(crate::out_events::OutDecalsAdd { kind, pos, normal, size, opts }));
}

pub fn decals_clear(g: &mut crate::game::Game) { crate::render_decals::marks_clear(g); }
