pub use ::game_types::thinline::types;
pub use types::*;
use crate::ids::*;

/// `b` is a thin line along its local z: its true half-width, the pixels its half-width must cover, what it emits at that true width. The note stands
/// beside the box and goes when the box's row is freed (`thin_forget`). Picture only: nothing in a
/// step reads it; a host that draws does (which boxes are a laser's beam or a tracer's streak).
pub fn thin_note(g: &mut crate::game::Game, b: AnyBoxId, hw0: f64, k: f64) -> AnyBoxId {
    g.m.thinline.thin.set(b, crate::thinline::Thin { hw0, k });
    b
}
/// A box's row is free again: what was noted of the box that stood in it is gone with it.
pub(super) fn thin_forget(g: &mut crate::game::Game, b: AnyBoxId) {
    if !g.m.thinline.thin.is_empty() { g.m.thinline.thin.delete(b); }
    if !g.m.thinline.air.is_empty() { g.m.thinline.air.delete(b); }
}
/// `line` (already noted) is a weapon laser's beam and `bead` the dot where it lands, or none: a host may draw the pair as light in the air.
pub fn thin_air(g: &mut crate::game::Game, line: AnyBoxId, bead: Option<AnyBoxId>) -> AnyBoxId { g.m.thinline.air.set(line, bead); line }
