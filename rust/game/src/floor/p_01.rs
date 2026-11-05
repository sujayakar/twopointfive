//! src/game/floor.ts lines 1 to 80
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// FINISHES[i].mat as the word it is: the skeleton's row holds a String where the TypeScript has FloorMat. (floor.ts:38)
fn seam_finish_mat(word: &str) -> FloorMat {
    match word {
        "carpet" => FloorMat::Carpet,
        "lino" => FloorMat::Lino,
        "stone" => FloorMat::Stone,
        "tile" => FloorMat::Tile,
        "asphalt" => FloorMat::Asphalt,
        "grass" => FloorMat::Grass,
        _ => unreachable!("FINISHES holds only FloorMat's words"),
    }
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
        if x >= f.x0 && x <= f.x1 && z >= f.z0 && z <= f.z1 { return seam_finish_mat(&f.mat); }
    }
    FloorMat::Carpet
}

/// The material a footfall at `pos` is on for everyone who listens: the map's, or lino everywhere with the materials off. (floor.ts stepMat)
pub fn step_mat(g: &mut crate::game::Game, pos: V3) -> crate::floor::FloorMat {
    if g.tune.floor_materials { floor_of(g, pos) } else { FloorMat::Lino }
}

/// The material under a point on this game's level; a quiet mission's level that declares no surfaces is plain lino. (floor.ts floorOf)
pub fn floor_of(g: &mut crate::game::Game, pos: V3) -> crate::floor::FloorMat {
    let l = &g.level;
    if l.surfaces.is_none() && l.scenarios.as_ref().map_or(false, |s| s.quiet.is_some()) {
        FloorMat::Lino
    } else {
        floor_at(pos, l.surfaces.as_ref())
    }
}

/// One footfall, for the mixer and the model alike: plays it and returns the level the caller files. (floor.ts tread)
pub fn tread(g: &mut crate::game::Game, pos: V3, base: f64, is_player: bool) -> f64 {
    if !g.tune.floor_materials {
        let real = floor_of(g, pos);
        let hard = floor().get(String::from(real.as_str())).unwrap().hard;
        crate::audio_iface::game_audio_like_footstep(g, pos, base * (if hard { 1.0 } else { 0.8 }), is_player, Some(real));
        return crate::shards::glass_shards_step(g, pos, base);
    }
    let mat = floor_of(g, pos);
    let level = base * floor().get(String::from(mat.as_str())).unwrap().loud;
    crate::audio_iface::game_audio_like_footstep(g, pos, level, is_player, Some(mat));
    crate::shards::glass_shards_step(g, pos, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_level_data::{Rect, SurfacesData, SurfacesDataZonesItem};

    fn zone(mat: FloorMat, x0: f64, z0: f64, x1: f64, z1: f64) -> SurfacesDataZonesItem {
        SurfacesDataZonesItem { id: String::new(), mat, x0, z0, x1, z1, notes: None }
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

    // tread's two lookups by word: every material has its row, with floor.ts's numbers.
    #[test]
    fn every_material_has_its_row() {
        use FloorMat::*;
        let cases: &[(FloorMat, f64, bool)] = &[
            (Carpet, 0.6, false), (Lino, 1.0, true), (Stone, 1.1, true), (Tile, 1.05, true), (Asphalt, 0.9, true), (Grass, 0.45, false),
        ];
        let t = floor();
        for &(mat, loud, hard) in cases {
            let row = t.get(String::from(mat.as_str())).unwrap();
            assert_eq!((row.loud.to_bits(), row.hard), (loud.to_bits(), hard), "{}", mat.as_str());
            assert_eq!(seam_finish_mat(mat.as_str()), mat);
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
