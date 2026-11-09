//! src/game/floor.ts lines 1 to 80
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// FLOOR[mat]: the table's row for a material, walked in the written order. (floor.ts:22)
fn floor_row(mat: FloorMat) -> FLOORValue {
    floor().iter().find(|(m, _)| *m == mat).unwrap().1
}

/// The material under a point on the floor (y ignored): the level's `surfaces` block, or without one the office's map. (floor.ts floorAt)
pub fn floor_at(p: V3, s: Option<&crate::scene_level_data::SurfacesData>) -> crate::floor::FloorMat {
    let (x, z) = (p[0], p[2]);
    if let Some(s) = s {
        let b = &s.building;
        if x < b.x0 || x > b.x1 || z < b.z0 || z > b.z1 {
            return if x < s.grass_edge || z < s.grass_edge { s.verge } else { s.outside };
        }
        for f in s.zones.iter() {
            if x >= f.x0 && x <= f.x1 && z >= f.z0 && z <= f.z1 { return f.mat; }
        }
        return s.inside;
    }
    let b = building();
    if x < b.x0 || x > b.x1 || z < b.z0 || z > b.z1 {
        return if x < GRASS_EDGE || z < GRASS_EDGE { FloorMat::Grass } else { FloorMat::Asphalt };
    }
    for f in finishes().iter() {
        if x >= f.x0 && x <= f.x1 && z >= f.z0 && z <= f.z1 { return f.mat; }
    }
    FloorMat::Carpet
}

/// The material under a point on this game's level; a quiet mission's level that declares no surfaces is plain lino. (floor.ts floorOf)
pub fn floor_of(g: &crate::game::Game, pos: V3) -> crate::floor::FloorMat {
    let l = &g.level;
    if l.surfaces.is_none() && l.scenarios.as_ref().map_or(false, |s| s.quiet.is_some()) {
        FloorMat::Lino
    } else {
        floor_at(pos, l.surfaces.as_ref())
    }
}

/// What the hearing model has under a point, as a HUD says it (None: the floors are all one). (game.ts floorUnderPlayer)
pub fn floor_heard(g: &crate::game::Game, pos: V3) -> Option<crate::view::FloorWord> {
    use crate::{floor::FloorMat as F, view::FloorWord as W};
    if !g.tune.floor_materials { return None; }
    Some(match floor_of(g, pos) { F::Carpet => W::Carpet, F::Lino => W::Lino, F::Stone => W::Stone, F::Tile => W::Tile, F::Asphalt => W::Asphalt, F::Grass => W::Grass })
}

/// One footfall, for the mixer and the model alike: plays it and returns the level the caller files. (floor.ts tread)
pub fn tread(g: &mut crate::game::Game, pos: V3, base: f64, is_player: bool) -> f64 {
    let level = footfall(g, pos, base, is_player);
    crate::shards::glass_shards_step(g, pos, level)
}

/// A foot comes down on what lies under it: heard by that, and its level.
pub fn footfall(g: &mut crate::game::Game, pos: V3, base: f64, is_player: bool) -> f64 {
    let mat = floor_of(g, pos);
    let row = floor_row(mat);
    let (heard, level) = if g.tune.floor_materials { (base * row.loud, base * row.loud) } else { (base * (if row.hard { 1.0 } else { 0.8 }), base) };
    crate::audio_iface::game_audio_like_footstep(g, pos, heard, is_player, mat);
    level
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_level_data::{Rect, SurfacesData, SurfacesDataZonesItem};

    fn zone(mat: FloorMat, x0: f64, z0: f64, x1: f64, z1: f64) -> SurfacesDataZonesItem {
        SurfacesDataZonesItem { mat, x0, z0, x1, z1 }
    }

    // Expected values printed by the TypeScript's own floorAt under bun.
    #[test]
    fn the_offices_map() {
        use FloorMat::*;
        let cases: &[(f64, f64, FloorMat)] = &[
            (4.0, 10.0, Lino), (36.0, 12.2, Lino), (12.0, 16.0, Stone), (12.5, 16.0, Carpet), (14.0, 4.0, Tile), (22.0, 10.0, Lino),
            (28.0, 12.2, Lino), (36.0, 24.0, Lino), (20.0, 15.0, Carpet), (3.9, 3.5, Asphalt), (3.1, 30.0, Grass), (30.0, 3.1, Grass),
            (3.2, 3.2, Asphalt), (36.5, 24.0, Asphalt), (20.0, 24.5, Asphalt), (4.0, 4.0, Carpet), (f64::NAN, 15.0, Carpet),
        ];
        for &(x, z, want) in cases { assert_eq!(floor_at([x, 7.0, z], None), want, "{} {}", x, z); }
    }

    // tread's lookup: every material has its row, with floor.ts's numbers.
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
        for &(x, z, want) in cases { assert_eq!(floor_at([x, 0.0, z], Some(&s)), want, "{} {}", x, z); }
    }
}
