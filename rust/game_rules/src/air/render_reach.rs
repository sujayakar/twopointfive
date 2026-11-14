use crate::m;
use crate::v3::V3;
use alloc::{vec, vec::Vec};

/// A CEILING'S FOOTPRINT (x0 z0 x1 z1): a box at the height of a ceiling or a roof, broader than a beam.
pub fn ceiling(c: V3, h: V3) -> Option<[f64; 4]> {
    let ceiling_y = crate::scene_world::world().ceiling_y;
    let (y0, y1) = (c[1] - h[1], c[1] + h[1]);
    if !(y0 >= ceiling_y - 0.25 && y1 <= ceiling_y + 0.45) || h[0] < 0.45 || h[2] < 0.45 { return None; }
    Some([c[0] - h[0], c[2] - h[2], c[0] + h[0], c[2] + h[2]])
}
/// The ceilings among the level's statics as they stand.
pub fn ceilings(g: &crate::game::Game) -> Vec<[f64; 4]> {
    g.engine.world.statics.iter().filter_map(|&b| ceiling(g.tab.boxes[b].c, g.tab.boxes[b].h)).collect()
}

/// The metre cells a ceiling reaches into; the game's own rules read them.
pub fn build_interior_cells(g: &crate::game::Game) -> Vec<u8> {
    let world = crate::scene_world::world();
    let (w, h) = (world.grid_w, world.grid_h);
    let mut inside = vec![0u8; (w * h) as usize];
    for r in ceilings(g) {
        let x0 = m::max(0.0, m::floor(r[0] - world.min[0]));
        let x1 = m::min(w - 1.0, m::floor(r[2] - world.min[0] - 1e-6));
        let z0 = m::max(0.0, m::floor(r[1] - world.min[2]));
        let z1 = m::min(h - 1.0, m::floor(r[3] - world.min[2] - 1e-6));
        // doubles: a NaN bound runs no cell, a slab wholly off the grid neither
        let mut z = z0;
        while z <= z1 {
            let mut x = x0;
            while x <= x1 {
                inside[(z * w + x) as usize] = 1;
                x += 1.0;
            }
            z += 1.0;
        }
    }
    inside
}

/// Is this plan point in a metre cell a ceiling reaches into: what every rule that asks "indoors" asks. True where no cells were ever made.
pub fn roofed(g: &crate::game::Game, x: f64, z: f64) -> bool {
    let Some(cells) = &g.reach_interior else { return true };
    let w = crate::scene_world::world();
    let (cx, cz) = (m::floor(x - w.min[0]), m::floor(z - w.min[2]));
    cx >= 0.0 && cz >= 0.0 && cx < w.grid_w && cz < w.grid_h && cells.get((cz * w.grid_w + cx) as usize) == Some(&1)
}
