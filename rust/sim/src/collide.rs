//! A CIRCLE AGAINST THE BOX WORLD — a body's footprint pushed out of the statics, and which statics it touches (src/scene/collision.ts collideCircle,
//! circleTouches, gather), over the module's copy of the boxes (world.rs).
//!
//! STATEMENT FOR STATEMENT, as lines.rs is: the same cells in the same order, a box met once a pass, the same pushes added in the same order — a body pushed
//! out of two boxes ends where the TypeScript leaves it, to the bit. tools/qa/rust-check.ts holds the two together.
//!
//! What is not here: whether a broken pane can be walked through (game/glassbreak.ts) is a rule of the TypeScript's; the copy carries its answer for each
//! pane as it stood when the copy was last compared (K_WALK), and the rule says so when it changes one.

use crate::world::*;
use prim::{floor, sqrt};

/// collision.ts gather's test of one static: is it out of the question (wholly above or below the span, hidden at runtime, a pane walked through)
#[inline(always)]
fn passed(b: &[f64], fl: u32, y0: f64, y1: f64) -> bool {
    b[R_C + 1] + b[R_H + 1] < y0 || b[R_C + 1] - b[R_H + 1] > y1 || fl & F_NO_SHADOW != 0 || (b[R_KIND] as u32) & K_WALK != 0
}

/// collision.ts collideCircle (its span's foot already lifted by the host: footBand): the circle of radius `r` at (px, pz) pushed out of the statics that
/// overlap [y0, y1]. Where it ends is left in `pt`; 1 = it touched something.
pub fn circle(w: usize, px: f64, pz: f64, r: f64, y0: f64, y1: f64, iterations: u32) -> u32 {
    let o = world(w);
    let (mut p0, mut p2) = (px, pz);
    let mut touched = false;
    let (cs, wm, hm) = (o.cs, o.w as f64 - 1.0, o.h as f64 - 1.0);
    let wi = o.w as usize;
    let mut it = 0;
    while it < iterations {
        let mut any = false;
        let (x0, x1) = (js_max(0.0, floor((p0 - r) / cs)), js_min(wm, floor((p0 + r) / cs)));
        let (z0, z1) = (js_max(0.0, floor((p2 - r) / cs)), js_min(hm, floor((p2 + r) / cs)));
        let id = o.ask();
        let mut z = z0;
        while z <= z1 {
            let mut x = x0;
            while x <= x1 {
                let c = z as usize * wi + x as usize;
                let (k0, k1) = (o.start[c] as usize, o.start[c + 1] as usize);
                for k in k0..k1 {
                    let bi = o.items[k] as usize;
                    if o.stamp[bi] == id {
                        continue;
                    }
                    o.stamp[bi] = id;
                    let b = &o.rec[bi * REC..bi * REC + REC];
                    if passed(b, o.flags[bi], y0, y1) {
                        continue;
                    }
                    let (dx, dz) = (p0 - b[R_C], p2 - b[R_C + 2]);
                    let (h0, h2) = (b[R_H], b[R_H + 2]);
                    if (b[R_KIND] as u32) & K_ROUND != 0 {
                        let rr = r + h0;
                        let d2r = dx * dx + dz * dz;
                        if d2r >= rr * rr {
                            continue;
                        }
                        if d2r > 1e-10 {
                            let d = sqrt(d2r);
                            let pen = rr - d;
                            p0 += dx / d * pen;
                            p2 += dz / d * pen;
                        } else {
                            p0 += rr;
                        }
                        any = true;
                        touched = true;
                        continue;
                    }
                    let (cy, sy) = (b[R_COS], b[R_SIN]);
                    let (lx, lz) = (cy * dx - sy * dz, sy * dx + cy * dz);
                    let (qx, qz) = (js_max(-h0, js_min(h0, lx)), js_max(-h2, js_min(h2, lz)));
                    let (ddx, ddz) = (lx - qx, lz - qz);
                    let d2 = ddx * ddx + ddz * ddz;
                    if d2 >= r * r {
                        continue;
                    }
                    let (nx, nz, pen);
                    if d2 > 1e-10 {
                        let d = sqrt(d2);
                        nx = ddx / d;
                        nz = ddz / d;
                        pen = r - d;
                    } else {
                        let (ex, ez) = (h0 - lx.abs(), h2 - lz.abs());
                        if ex < ez {
                            nx = sign_or_one(lx);
                            nz = 0.0;
                            pen = ex + r;
                        } else {
                            nx = 0.0;
                            nz = sign_or_one(lz);
                            pen = ez + r;
                        }
                    }
                    let (wx, wz) = (cy * nx + sy * nz, -sy * nx + cy * nz);
                    p0 += wx * pen;
                    p2 += wz * pen;
                    any = true;
                    touched = true;
                }
                x += 1.0;
            }
            z += 1.0;
        }
        if !any {
            break;
        }
        it += 1;
    }
    o.pt = [p0, p2];
    touched as u32
}

/// collision.ts gather: the statics filed under the rectangle [x0, x1] × [z0, z1] that overlap [y0, y1], once each, into `list` — as many as `cap` (the
/// host's own list's length). Returns how many.
pub fn gather(w: usize, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, cap: usize) -> usize {
    let o = world(w);
    let cap = if cap > MAX_LIST { MAX_LIST } else { cap };
    let id = o.ask();
    let mut n = 0usize;
    let cs = o.cs;
    let wi = o.w as usize;
    let (cx0, cx1) = (js_max(0.0, floor(x0 / cs)), js_min(o.w as f64 - 1.0, floor(x1 / cs)));
    let (cz0, cz1) = (js_max(0.0, floor(z0 / cs)), js_min(o.h as f64 - 1.0, floor(z1 / cs)));
    let mut z = cz0;
    while z <= cz1 {
        let mut x = cx0;
        while x <= cx1 {
            let c = z as usize * wi + x as usize;
            let (k0, k1) = (o.start[c] as usize, o.start[c + 1] as usize);
            for k in k0..k1 {
                let bi = o.items[k] as usize;
                if o.stamp[bi] == id {
                    continue;
                }
                o.stamp[bi] = id;
                if passed(&o.rec[bi * REC..bi * REC + REC], o.flags[bi], y0, y1) {
                    continue;
                }
                if n < cap {
                    o.list[n] = bi as u32;
                    n += 1;
                }
            }
            x += 1.0;
        }
        z += 1.0;
    }
    n
}

/// collision.ts circleTouches (its span's foot already lifted by the host): the statics a circle of radius `r` at (x, z) touches, at the front of `list`.
/// −1 = the host's list could not hold what was gathered; else (how many touch) × 65536 + (how many were gathered: the host's list holds those past the
/// touching ones as the gather left them, and so must its copy).
pub fn touches(w: usize, x: f64, z: f64, r: f64, y0: f64, y1: f64, cap: usize) -> i32 {
    if cap > MAX_LIST {
        return -2;
    }
    let n = gather(w, x - r, z - r, x + r, z + r, y0, y1, cap);
    if n >= cap {
        return -1;
    }
    let o = world(w);
    let mut k = 0usize;
    for j in 0..n {
        let bi = o.list[j] as usize;
        let b = &o.rec[bi * REC..bi * REC + REC];
        let (cy, sy) = (b[R_COS], b[R_SIN]);
        let (dx, dz) = (x - b[R_C], z - b[R_C + 2]);
        let (h0, h2) = (b[R_H], b[R_H + 2]);
        if (b[R_KIND] as u32) & K_ROUND != 0 {
            let rr = r + h0;
            if dx * dx + dz * dz < rr * rr {
                o.list[k] = bi as u32;
                k += 1;
            }
            continue;
        }
        let (lx, lz) = (cy * dx - sy * dz, sy * dx + cy * dz);
        let (ex, ez) = (lx - js_max(-h0, js_min(h0, lx)), lz - js_max(-h2, js_min(h2, lz)));
        if ex * ex + ez * ez < r * r {
            o.list[k] = bi as u32;
            k += 1;
        }
    }
    (k * 65536 + n) as i32
}
