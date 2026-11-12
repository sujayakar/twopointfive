use crate::ids::*;
use crate::m;
use alloc::{vec, vec::Vec};

/// The cells under a ceiling or roof slab, from the level's ceiling-height statics; the game's own rules read them.
pub fn build_interior_cells(g: &crate::game::Game, statics: &[BoxId]) -> Vec<u8> {
    let world = crate::scene_world::world();
    let (w, h) = (world.grid_w, world.grid_h);
    let mut inside = vec![0u8; (w * h) as usize];
    for &id in statics {
        let (bc, bh) = (g.tab.boxes[id].c, g.tab.boxes[id].h);
        let (y0, y1) = (bc[1] - bh[1], bc[1] + bh[1]);
        if !(y0 >= world.ceiling_y - 0.25 && y1 <= world.ceiling_y + 0.45) || bh[0] < 0.45 || bh[2] < 0.45 { continue; }
        let x0 = m::max(0.0, m::floor(bc[0] - bh[0] - world.min[0]));
        let x1 = m::min(w - 1.0, m::floor(bc[0] + bh[0] - world.min[0] - 1e-6));
        let z0 = m::max(0.0, m::floor(bc[2] - bh[2] - world.min[2]));
        let z1 = m::min(h - 1.0, m::floor(bc[2] + bh[2] - world.min[2] - 1e-6));
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
