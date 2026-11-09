pub use ::game_types::snow_bog::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `b` of the level's boxes, its row. `g.level.boxes` is `g.col.boxes`, one array: the IDs are there, the rows have no table.
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// The run-on a thing keeps past its landing where the ground bogs by `bog`, of a hard floor's.
pub fn bog_run(g: &crate::game::Game, bog: f64) -> f64 {
    run_on(bog, g.k.snowbog.run)
}

/// bogRun's arithmetic, with SNOWBOG.run handed in.
fn run_on(bog: f64, run: f64) -> f64 {
    if bog > 0.0 { m::pow(m::max(0.0, 1.0 - bog), run) } else { 1.0 }
}

/// What snow each box of the level is, a byte a box, worked out once a level.
fn ground(g: &mut crate::game::Game) -> crate::snow_bog::Ground {
    if let Some(found) = &g.m.snow_bog.ground_of { return found.clone(); }
    let kind = snow_kind();
    let count = g.col.boxes.len();
    let mut made = Ground { kinds: vec![0u8; count] };
    for bi in 0..count {
        let b = g.col.boxes[bi];
        let does = seam_box(g, b).does.map(|w| String::from(w.as_str()));
        let p = crate::render_snow_field::snow_paint_of(g, &crate::render_snow_field::SnowPaintOfB { does });
        if crate::render_snow_field::snow_thin_lay(g, b) {
            let packed = p == Some(crate::scene_materials::SnowKind::SnowPacked);
            let old = p == Some(crate::scene_materials::SnowKind::SnowOld);
            if packed || old || crate::render_snow_field::snow_lay_bare(g, b) {
                made.kinds[bi] = (if packed { kind.packed } else if old { kind.old } else { kind.bare }) as u8;
            }
        } else if p == Some(crate::scene_materials::SnowKind::Snow) {
            let found = seam_box(g, b);
            if (js::to_i32(found.flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) == 0 && found.c[1] + found.h[1] > 0.03 {
                made.kinds[bi] = kind.bank as u8;
            }
        }
    }
    g.m.snow_bog.ground_of = Some(made.clone());
    made
}

/// May the ground bog anything in this game now.
pub fn snow_bog_may(g: &crate::game::Game) -> bool {
    let name = crate::scene_level_load::level_name_of(g);
    crate::weather_kinds::snow_level(g, name.as_deref())
}

/// The level's snow as a byte a box.
pub fn snow_bog_kinds(g: &mut crate::game::Game) -> Vec<u8> {
    ground(g).kinds
}

/// How boggy the ground is where a thing touched down, 0 to 1: the module's rule, over its table of the snow and the weather's roofs.
pub fn snow_bog_at(g: &mut crate::game::Game, x: f64, y: f64, z: f64) -> f64 {
    if !snow_bog_may(g) { return 0.0; }
    crate::weather::weather_roofs_now(g);
    crate::scene_collision::rule(g, &mut |g: &mut Game| {
        crate::scene_collision::rule_open(g);
        let kinds = ground(g).kinds;
        crate::scene_box_mirror::box_mirror_snow(g, &kinds);
        ::sim::snow::bog_at(&g.weather.roof_rects, g.col.mirror.slot as usize, x, y, z, &g.k)
    })
}

/// …asked of the game shared: the table of the snow as the last thing to touch the ground of this level left it (none has yet: no bog)
pub fn snow_bog_shared(g: &Game, x: f64, y: f64, z: f64) -> f64 {
    if !snow_bog_may(g) || g.col.mirror.snow_len < 0.0 { return 0.0; }
    ::sim::snow::bog_at(&g.weather.roof_rects, g.col.mirror.slot as usize, x, y, z, &g.k)
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
}
