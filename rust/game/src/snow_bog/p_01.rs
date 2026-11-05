//! src/game/snowBog.ts lines 1 to 74
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The run-on a thing keeps past its landing where the ground bogs by `bog`, of a hard floor's. (snowBog.ts bogRun)
pub fn bog_run(g: &mut crate::game::Game, bog: f64) -> f64 {
    run_on(bog, g.k.snowbog.run)
}

/// bogRun's arithmetic, with SNOWBOG.run handed in.
fn run_on(bog: f64, run: f64) -> f64 {
    if bog > 0.0 { m::pow(m::max(0.0, 1.0 - bog), run) } else { 1.0 }
}

/// What snow each box of the level is, worked out once a level. (snowBog.ts ground)
pub fn ground(g: &mut crate::game::Game) -> crate::snow_bog::Ground {
    if let Some(found) = &g.m.snow_bog.ground_of { return found.clone(); }
    let kind = snow_kind();
    let count = g.col.boxes.len();
    let mut made = Ground { kinds: vec![0u8; count], lays: Vec::new(), banks: Vec::new() };
    for bi in 0..count {
        let b = BoxId::from_ix(bi);
        let does = g.col.boxes[b].does.map(|w| String::from(w.as_str()));
        let p = crate::render_snow_field::snow_paint_of(g, &crate::render_snow_field::SnowPaintOfB { does });
        if crate::render_snow_field::snow_thin_lay(g, b) {
            let packed = p == Some(crate::scene_materials::SnowKind::SnowPacked);
            let old = p == Some(crate::scene_materials::SnowKind::SnowOld);
            if packed || old || crate::render_snow_field::snow_lay_bare(g, b) {
                made.lays.push(bi as f64);
                made.kinds[bi] = (if packed { kind.packed } else if old { kind.old } else { kind.bare }) as u8;
            }
        } else if p == Some(crate::scene_materials::SnowKind::Snow) {
            let found = &g.col.boxes[b];
            if (js::to_i32(found.flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) == 0 && found.c[1] + found.h[1] > 0.03 {
                made.banks.push(bi as f64);
                made.kinds[bi] = kind.bank as u8;
            }
        }
    }
    g.m.snow_bog.ground_of = Some(made.clone());
    made
}

/// Is (x, z) over the box's footprint, turned by its yaw. (snowBog.ts over)
pub fn over(g: &mut crate::game::Game, b: BoxId, x: f64, z: f64) -> bool {
    let found = &g.col.boxes[b];
    let (cs, sn, px, pz) = (m::cos(found.yaw), m::sin(found.yaw), x - found.c[0], z - found.c[2]);
    m::abs(cs * px - sn * pz) <= found.h[0] && m::abs(sn * px + cs * pz) <= found.h[2]
}

/// May the ground bog anything in this game now. (snowBog.ts snowBogMay)
pub fn snow_bog_may(g: &mut crate::game::Game) -> bool {
    if !g.k.snowbog.on { return false; }
    let name = crate::scene_level_load::level_name_of(g);
    crate::weather_kinds::snow_level(g, name.as_deref())
}

/// The level's snow as a byte a box. (snowBog.ts snowBogKinds)
pub fn snow_bog_kinds(g: &mut crate::game::Game) -> Vec<u8> {
    ground(g).kinds
}

/// How boggy the ground is where a thing touched down, 0 to 1. (snowBog.ts snowBogAt)
pub fn snow_bog_at(g: &mut crate::game::Game, x: f64, y: f64, z: f64) -> f64 {
    if !snow_bog_may(g) { return 0.0; }
    let found = ground(g);
    let n = g.col.boxes.len();
    if !(y < g.k.snowbog.top) {
        for &at in &found.banks {
            let bi = at as usize;
            if bi >= n { break; }
            let b = BoxId::from_ix(bi);
            let top = g.col.boxes[b].c[1] + g.col.boxes[b].h[1];
            if m::abs(top - y) <= g.k.snowbog.bank_m && over(g, b, x, z) { return 1.0; }
        }
        return 0.0;
    }
    if crate::weather::weather_under_roof(g, x, z) { return 0.0; }
    let names = snow_kind();
    let mut k = 1.0;
    for &at in &found.lays {
        let bi = at as usize;
        if bi >= n { break; }
        if !over(g, BoxId::from_ix(bi), x, z) { continue; }
        let kind = found.kinds[bi] as f64;
        let v = if kind == names.packed { g.k.snowbog.packed } else if kind == names.old { g.k.snowbog.old } else { 0.0 };
        if v < k { k = v; }
    }
    m::max(0.0, m::min(1.0, k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_on_matches_the_typescript_bits() {
        // bogRun under EXACT, SNOWBOG.run = 2
        let cases: [(f64, u64); 6] = [
            (0.85, 0x3f970a3d70a3d70c),
            (0.34, 0x3fdbe0ded288ce6e),
            (0.1, 0x3fe9eb851eb851ec),
            (1.5, 0x0),
            (0.0, 0x3ff0000000000000),
            (-1.0, 0x3ff0000000000000),
        ];
        for (bog, bits) in cases { assert_eq!(run_on(bog, 2.0).to_bits(), bits, "bog {bog}"); }
    }

    #[test]
    fn over_turns_the_point_by_the_yaw() {
        let mut g = Game::blank();
        let mut made = crate::scene_boxes::Box::zeroed();
        made.c = [1.0, 0.0, 2.0];
        made.h = [2.0, 1.0, 0.5];
        made.yaw = m::atan2(1.0, 0.0);
        let b = g.col.boxes.push(made);
        // a quarter turn: the long side runs along z
        assert!(over(&mut g, b, 1.4, 3.5));
        assert!(!over(&mut g, b, 2.0, 2.0));
    }
}
