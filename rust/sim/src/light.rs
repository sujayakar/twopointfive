//! THE LIGHT ON A POINT — src/game/guards.ts lightEstimateAt: the direct light of every lamp in reach on the six faces of the meter, each lamp shadowed by
//! one line to it. The rule is the TypeScript's, statement for statement, from the distance to the lamp on; which lamps are asked at all (enabled, whose,
//! steady or not) stays the host's, which writes a row a lamp that passed and asks once a point. The hour's sun and sky and a stroke of lightning are added
//! to the same six faces by the host afterwards (hours.ts hourLight, weather.ts strokeLight), as they were.
//!
//! The lamps are scratch of one question. The cosines and the lines' lengths are the game's own, so this answers only where EXACT.on.

use crate::lines;
use crate::maths::cos;
use crate::world::{ASK_HOST, clamp, js_max, js_max3, js_min, smoothstep, DEG, F_GLASS};
use prim::sqrt;

/// lamps one question can carry (a level's whole list is some dozens; more is the host's own loop)
pub const MAX_LAMPS: usize = 256;
/// DOUBLES A LAMP: where it is (x y z), its range, its radius, its kind (0 point · 1 spot · 3 panel), where it points (x y z), its outer and inner half-angles
/// in degrees, its colour (r g b), its intensity
pub const LAMP: usize = 15;
const L_POS: usize = 0;
const L_RANGE: usize = 3;
const L_RADIUS: usize = 4;
const L_KIND: usize = 5;
const L_DIR: usize = 6;
const L_OUTER: usize = 9;
const L_INNER: usize = 10;
const L_COLOR: usize = 11;
const L_I: usize = 14;

static mut LAMPS: [f64; MAX_LAMPS * LAMP] = [0.0; MAX_LAMPS * LAMP];

/// where a question's lamps are written (a byte address)
pub fn lamps_addr() -> usize {
    core::ptr::addr_of!(LAMPS) as usize
}

/// the six axis normals the renderer's meter takes its best face over (guards.ts METER_NORMALS)
const N6: [[f64; 3]; 6] = [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];

/// guards.ts lightWindow: (1 − (d/range)⁴)², clamped
#[inline(always)]
fn light_window(d: f64, range: f64) -> f64 {
    let x = d / range;
    let w = clamp(1.0 - x * x * x * x, 0.0, 1.0);
    w * w
}

/// the first `n` lamps on the point `p`: the six faces go to `acc`. 0, or ASK_HOST (nothing in `acc` is then to be read).
pub fn estimate(w: usize, set: i32, n: usize, p: [f64; 3], acc: &mut [f64]) -> u32 {
    let lamps = unsafe { &*core::ptr::addr_of!(LAMPS) };
    for a in acc[..6].iter_mut() {
        *a = 0.0;
    }
    if n > MAX_LAMPS {
        return ASK_HOST;
    }
    for i in 0..n {
        let l = &lamps[i * LAMP..(i + 1) * LAMP];
        let (dx, dy, dz) = (l[L_POS] - p[0], l[L_POS + 1] - p[1], l[L_POS + 2] - p[2]);
        let dist2 = dx * dx + dy * dy + dz * dz;
        let dist = sqrt(dist2);
        let range = l[L_RANGE];
        if dist > range || dist < 1e-3 {
            continue;
        }
        let (lx, ly, lz) = (dx / dist, dy / dist, dz / dist);
        let lr = l[L_RADIUS];
        let mut att = light_window(dist, range) / js_max(dist2, lr * lr);
        let kind = l[L_KIND];
        if kind == 1.0 {
            let cd = -(lx * l[L_DIR] + ly * l[L_DIR + 1] + lz * l[L_DIR + 2]);
            let cos_outer = cos(l[L_OUTER] * DEG);
            let cos_inner = cos(js_min(l[L_INNER], l[L_OUTER] - 0.5) * DEG);
            if cd < cos_outer {
                continue;
            }
            att *= smoothstep(cos_outer, cos_inner, cd);
        } else if kind == 3.0 {
            let ce = -(lx * l[L_DIR] + ly * l[L_DIR + 1] + lz * l[L_DIR + 2]);
            if ce <= 0.0 {
                continue;
            }
            att *= ce;
        }
        if att <= 0.0 {
            continue;
        }
        let (c0, c1, c2, int) = (l[L_COLOR], l[L_COLOR + 1], l[L_COLOR + 2], l[L_I]);
        if js_max3(c0, c1, c2) * int * att < 0.002 {
            continue;
        }
        let lum = (0.2126 * c0 + 0.7152 * c1 + 0.0722 * c2) * int * att;
        let tmax = dist - lr - 0.02;
        if tmax > 0.05 {
            match lines::sight_line(w, set, F_GLASS, p[0], p[1], p[2], p[0] + lx * tmax, p[1] + ly * tmax, p[2] + lz * tmax) {
                None => return ASK_HOST,
                Some(true) => continue,
                Some(false) => {}
            }
        }
        for k in 0..6 {
            let nn = &N6[k];
            let cs = nn[0] * lx + nn[1] * ly + nn[2] * lz;
            if cs > 0.0 {
                acc[k] += lum * cs;
            }
        }
    }
    0
}
