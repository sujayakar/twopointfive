pub use ::game_types::scene_stuff::types;
pub use types::*;
use crate::game::Game;

/// The paint this array is, if anybody said so.
pub fn paint_of(g: &crate::game::Game, albedo: Option<&[f64]>) -> Option<crate::scene_stuff::Paint> {
    match albedo {
        Some(row) => paints_get(g, row),
        None => None,
    }
}

/// The words a box made with `albedo` carries: the maker's ('plain' = nothing, said over a paint that says something), else the paint's.
pub fn word_for(g: &crate::game::Game, albedo: Option<&[f64]>, said: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>) -> crate::scene_stuff::WordFor {
    let p = paint_of(g, albedo);
    let p_stuff = p.as_ref().and_then(|p| p.stuff);
    let p_does = p.as_ref().and_then(|p| p.does);
    let w = said.or(p_stuff);
    let d = does.or(p_does);
    WordFor {
        stuff: if w != Some(BoxStuff::Plain) { w } else { None },
        does: d,
    }
}

fn paints_get(g: &Game, row: &[f64]) -> Option<Paint> {
    if row.len() < 3 { return None; }
    g.m.scene_stuff.paints.get([row[0].to_bits(), row[1].to_bits(), row[2].to_bits()]).cloned()
}
