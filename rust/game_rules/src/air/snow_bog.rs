use crate::boxworld::{SNOW_BANK, SNOW_BARE, SNOW_OLD, SNOW_PACKED};
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// The run-on a thing keeps past its landing where the ground bogs by `bog`, of a hard floor's.
pub fn bog_run(g: &crate::game::Game, bog: f64) -> f64 {
    run_on(bog, g.k.snowbog.run)
}

/// bogRun's arithmetic, with SNOWBOG.run handed in.
fn run_on(bog: f64, run: f64) -> f64 {
    if bog > 0.0 { m::pow(m::max(0.0, 1.0 - bog), run) } else { 1.0 }
}

/// what snow a box of the level is (SNOW_*; 0 none)
fn kind_of(g: &Game, b: BoxId) -> u8 {
    use crate::scene_materials::SnowKind;
    let found = &g.tab.boxes[b];
    let paint = crate::scene_materials::does_of(g, found.does).snow;
    if crate::render_snow_field::snow_thin_lay(g, b) {
        if paint == Some(SnowKind::SnowPacked) { SNOW_PACKED } else if paint == Some(SnowKind::SnowOld) { SNOW_OLD } else if crate::render_snow_field::snow_lay_bare(g, b) { SNOW_BARE } else { 0 }
    } else if paint == Some(SnowKind::Snow) && js::to_i32(found.flags) & js::to_i32(crate::scene_boxes::box_flag().glass) == 0 && found.c[1] + found.h[1] > 0.03 { SNOW_BANK } else { 0 }
}
/// THE SNOW OF A LEVEL that has any, as the level stands
pub fn lie(g: &mut Game) {
    if !snow_bog_may(g) { return; }
    let kinds: Vec<u8> = g.col.boxes.iter().map(|&b| kind_of(g, b)).collect();
    g.tab.boxes.snow(kinds);
}

/// May the ground bog anything in this game now.
pub fn snow_bog_may(g: &crate::game::Game) -> bool {
    let name = crate::scene_level_load::level_name_of(g);
    crate::weather_kinds::snow_level(g, name.as_deref())
}

/// How boggy the ground is where a thing touched down, 0 to 1, over the level's snow and the weather's roofs.
pub fn snow_bog_at(g: &Game, x: f64, y: f64, z: f64) -> f64 {
    if !snow_bog_may(g) { return 0.0; }
    ::sim::snow::bog_at(&g.weather.roof_rects, g.tab.boxes.world(), x, y, z, &g.k)
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
