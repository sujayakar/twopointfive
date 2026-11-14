pub use ::game_types::floor::types;
pub use types::*;
use crate::v3::V3;

/// FLOOR[mat]: the table's row for a material, walked in the written order.
fn floor_row(mat: FloorMat) -> FLOORValue {
    floor().iter().find(|(m, _)| *m == mat).unwrap().1
}

/// The material under a point on the floor (y ignored).
fn floor_at(p: V3, s: &crate::scene_level_data::SurfacesData) -> FloorMat {
    let (x, z) = (p[0], p[2]);
    let b = &s.building;
    if x < b.x0 || x > b.x1 || z < b.z0 || z > b.z1 {
        return if x < s.grass_edge || z < s.grass_edge { s.verge } else { s.outside };
    }
    s.zones.iter().find(|f| x >= f.x0 && x <= f.x1 && z >= f.z0 && z <= f.z1).map_or(s.inside, |f| f.mat)
}

/// The material under a point on this game's level.
pub fn floor_of(g: &crate::game::Game, pos: V3) -> FloorMat { floor_at(pos, &g.level.surfaces) }

/// What the hearing model has under a point, as a HUD says it (None: the floors are all one).
pub fn floor_heard(g: &crate::game::Game, pos: V3) -> Option<crate::view::FloorWord> {
    use crate::{floor::FloorMat as F, view::FloorWord as W};
    if !g.k.tune.floor_materials { return None; }
    Some(match floor_of(g, pos) { F::Carpet => W::Carpet, F::Lino => W::Lino, F::Stone => W::Stone, F::Tile => W::Tile, F::Asphalt => W::Asphalt, F::Grass => W::Grass })
}

/// One footfall, for the mixer and the model alike: plays it and returns the level the caller files.
pub fn tread(g: &mut crate::game::Game, pos: V3, base: f64, is_player: bool) -> f64 {
    let level = footfall(g, pos, base, is_player);
    crate::shards::glass_shards_step(g, pos, level)
}

/// A foot comes down on what lies under it: heard by that, and its level.
pub fn footfall(g: &mut crate::game::Game, pos: V3, base: f64, is_player: bool) -> f64 {
    let mat = floor_of(g, pos);
    let row = floor_row(mat);
    let (heard, level) = if g.k.tune.floor_materials { (base * row.loud, base * row.loud) } else { (base * (if row.hard { 1.0 } else { 0.8 }), base) };
    crate::audio_iface::game_audio_like_footstep(g, pos, heard, is_player, mat);
    level
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use super::*;
    use crate::scene_level_data::{Rect, SurfacesData, SurfacesDataZonesItem};

    fn zone(mat: FloorMat, x0: f64, z0: f64, x1: f64, z1: f64) -> SurfacesDataZonesItem {
        SurfacesDataZonesItem { mat, x0, z0, x1, z1 }
    }

    // tread's lookup: every material has its row.
    #[test]
    fn every_material_has_its_row() {
        use FloorMat::*;
        let cases: &[(FloorMat, f64, bool)] = &[
            (Carpet, 0.6, false), (Lino, 1.0, true), (Stone, 1.1, true), (Tile, 1.05, true), (Asphalt, 0.9, true), (Grass, 0.45, false),
        ];
        for &(mat, loud, hard) in cases {
            let row = floor_row(mat);
            assert_eq!((row.loud.to_bits(), row.hard), (loud.to_bits(), hard), "{}", mat.as_str());
        }
    }

    #[test]
    fn a_levels_own_surfaces() {
        use FloorMat::*;
        let s = SurfacesData {
            building: Rect { x0: 2.0, x1: 10.0, z0: 2.0, z1: 10.0 }, grass_edge: 1.0, inside: Tile, outside: Stone, verge: Lino,
            zones: vec![zone(Asphalt, 3.0, 3.0, 5.0, 5.0), zone(Grass, 4.0, 4.0, 6.0, 6.0)],
        };
        let cases: &[(f64, f64, FloorMat)] = &[
            (0.5, 5.0, Lino), (1.0, 1.0, Stone), (11.0, 5.0, Stone), (5.0, 0.5, Lino), (4.5, 4.5, Asphalt), (5.5, 5.5, Grass), (9.0, 9.0, Tile),
        ];
        for &(x, z, want) in cases { assert_eq!(floor_at([x, 0.0, z], &s), want, "{} {}", x, z); }
    }
}
