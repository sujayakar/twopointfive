//! THE LINE TESTS THROUGH THE BOX WORLD — is the segment a → b blocked by a static or a leaf (src/scene/collision.ts segmentBlocked, segmentBlockedDynamic and
//! what they call: dynamicHit, segClear, rayBoxHits, lineBox; src/scene/boxes.ts rayBoxRot / rayBoxRound / rayBoxCut; src/scene/cut.ts cutClip).
//!
//! STATEMENT FOR STATEMENT: the same cells walked in the same order, the same boxes met in the same order, the same slab arithmetic on the same doubles. The
//! TypeScript says WHY each line is what it is (the embedded origin, the pad, which cells matter); this file only has to be it. tools/qa/rust-check.ts holds
//! the two together: every question a mission asks answered by both, and the missions themselves played both ways.
//!
//! THE RAY (collision.ts raycast, and boxes.ts rayBox whole: where a ray first enters anything, how far along, and the face it enters by) is here too: the
//! same leaves first, the same cells in the same half-metre steps, the nearest entry kept. Every rule of the things that flies something through a level
//! asks it (a torn leaf, a thrown can, a round).
//!
//! WHETHER A PANE IS OPEN where a line or a ray meets it is answered here (glass.rs, over the copy of every pane that is no longer whole): a line that
//! crosses a hole is not stopped by the pane, and a ray flies on through it to what stands beyond, as the TypeScript's own walks do pane by pane.
//!
//! What is not here is a rule of play that is still TypeScript: whether a round gets through the panes on its line (game/glasspen.ts). The module asks
//! nobody (it imports nothing): for a line of fire the walk LISTS the glass the line crosses that is not open there — which pane, and where the line
//! enters it — and goes on; the host judges the list when the walk is back.

use crate::glass;
use crate::maths::hypot3;
use crate::world::*;
use prim::{floor, sqrt};

/// how far past a box's own bounds the early rejects still count it as near (collision.ts SEG_PAD)
const SEG_PAD: f64 = 0.05;
const INF: f64 = f64::INFINITY;

/// collision.ts segClear: can the segment be said, without the slab test, to come nowhere near the box
#[inline(always)]
fn seg_clear(a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, b: &[f64], pad: f64) -> bool {
    let hy = neg_up(b[R_H + 1]);
    let m1 = b[R_C + 1] - a1;
    if (b[R_KIND] as u32) & K_ROT == 0 {
        let e1 = r1 * len;
        if if e1 > 0.0 { m1 - hy > e1 + pad || m1 + hy < -pad } else { m1 - hy > pad || m1 + hy < e1 - pad } {
            return true;
        }
    }
    let rr = neg_up(b[R_H]) + hy + neg_up(b[R_H + 2]) + pad;
    let m0 = b[R_C] - a0;
    let m2 = b[R_C + 2] - a2;
    let s = m0 * r0 + m1 * r1 + m2 * r2;
    if s < -rr || s > len + rr {
        return true;
    }
    m0 * m0 + m1 * m1 + m2 * m2 - s * s > rr * rr
}

/// `1 / (d || 1e-20)`
#[inline(always)]
fn inv_or(d: f64) -> f64 {
    1.0 / (if d != 0.0 && d == d { d } else { 1e-20 })
}

/// one slab of rayBox: the interval cut down by the pair of faces at ±h along an axis; false = nothing left
#[inline(always)]
fn slab(h: f64, o: f64, inv: f64, t0: &mut f64, t1: &mut f64, entered: &mut bool) -> bool {
    let mut ta = (-h - o) * inv;
    let mut tb = (h - o) * inv;
    if ta > tb {
        core::mem::swap(&mut ta, &mut tb);
    }
    if ta > *t0 {
        *t0 = ta;
        *entered = true;
    }
    if tb < *t1 {
        *t1 = tb;
    }
    !(*t0 > *t1)
}

/// collision.ts rayBoxHits: does the ray enter this yawed box within tmax
#[inline(always)]
fn ray_box_hits(a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64], tmax: f64) -> bool {
    let (cs, sn) = (b[R_COS], b[R_SIN]);
    let (px, py, pz) = (a0 - b[R_C], a1 - b[R_C + 1], a2 - b[R_C + 2]);
    let (lx, lz, ly) = (cs * px - sn * pz, sn * px + cs * pz, py);
    let (dx, dz, dy) = (cs * r0 - sn * r2, sn * r0 + cs * r2, r1);
    let (ix, iy, iz) = (inv_or(dx), inv_or(dy), inv_or(dz));
    let (mut t0, mut t1, mut entered) = (0.0, tmax, false);
    if !slab(b[R_H], lx, ix, &mut t0, &mut t1, &mut entered) {
        return false;
    }
    if !slab(b[R_H + 1], ly, iy, &mut t0, &mut t1, &mut entered) {
        return false;
    }
    if !slab(b[R_H + 2], lz, iz, &mut t0, &mut t1, &mut entered) {
        return false;
    }
    entered
}

/// a point or a direction into a fully rotated box's frame: quat.rotate(quat.conj(rot), v), its expression over scalars
#[inline(always)]
fn unrotate(b: &[f64], vx: f64, vy: f64, vz: f64) -> (f64, f64, f64) {
    let (qx, qy, qz, qw) = (-b[R_ROT], -b[R_ROT + 1], -b[R_ROT + 2], b[R_ROT + 3]);
    let (tx, ty, tz) = (2.0 * (qy * vz - qz * vy), 2.0 * (qz * vx - qx * vz), 2.0 * (qx * vy - qy * vx));
    (vx + qw * tx + (qy * tz - qz * ty), vy + qw * ty + (qz * tx - qx * tz), vz + qw * tz + (qx * ty - qy * tx))
}
/// the ray in the box's own frame, as rayBoxRound and rayBoxCut put it there (rotated, or turned by the yaw)
#[inline(always)]
fn to_local(a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64]) -> (f64, f64, f64, f64, f64, f64) {
    let (px, py, pz) = (a0 - b[R_C], a1 - b[R_C + 1], a2 - b[R_C + 2]);
    if (b[R_KIND] as u32) & K_ROT != 0 {
        let (ox, oy, oz) = unrotate(b, px, py, pz);
        let (dx, dy, dz) = unrotate(b, r0, r1, r2);
        (ox, oy, oz, dx, dy, dz)
    } else {
        let (cs, sn) = (b[R_COS], b[R_SIN]);
        (cs * px - sn * pz, py, sn * px + cs * pz, cs * r0 - sn * r2, r1, sn * r0 + cs * r2)
    }
}

/// cut.ts cutClip: the interval cut down by the record's planes. (t0, t1, k) — k the plane entered through, −1 if t0 did not move — or None = nothing left
#[inline]
fn cut_clip(planes: &[f64], n: usize, ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64, mut t0: f64, mut t1: f64) -> Option<(f64, f64, i32)> {
    let mut k = -1i32;
    let mut i = 0;
    while i < n && i < CUT_PLANES {
        let p = &planes[i * 4..i * 4 + 4];
        if p[0] == 0.0 && p[1] == 0.0 && p[2] == 0.0 {
            break;
        }
        let den = p[0] * dx + p[1] * dy + p[2] * dz;
        let num = p[3] - (p[0] * ox + p[1] * oy + p[2] * oz);
        if den < 0.0 {
            let t = num / den;
            if t > t0 {
                t0 = t;
                k = i as i32;
            }
        } else if den > 0.0 {
            let t = num / den;
            if t < t1 {
                t1 = t;
            }
        } else if num < 0.0 {
            return None;
        }
        if t0 > t1 {
            return None;
        }
        i += 1;
    }
    Some((t0, t1, k))
}
#[inline(always)]
fn planes_of<'a>(o: &'a World, b: &[f64]) -> (&'a [f64], usize) {
    let at = b[R_CUT] as usize * CUT_PLANES * 4;
    (&o.cut[at..at + CUT_PLANES * 4], b[R_CUTN] as usize)
}

/// boxes.ts rayBox for a box that is not a plain yawed one (round, cut or rotated): where the ray enters it, if it does within tmax (ray_box_n's answer
/// without the face: one walk of each shape in this file)
fn ray_box_t(o: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64], tmax: f64) -> Option<f64> {
    ray_box_n(o, a0, a1, a2, r0, r1, r2, b, tmax).map(|h| h.0)
}

/// collision.ts lineBox: the interval [t0, t1] over which the LINE lies inside the box, and how deep inside it the origin sits; None = the line misses it
fn line_box(o: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64]) -> Option<(f64, f64, f64)> {
    let kind = b[R_KIND] as u32;
    let (ox, oy, oz, dx, dy, dz);
    if kind & K_ROT != 0 {
        (ox, oy, oz) = unrotate(b, a0 - b[R_C], a1 - b[R_C + 1], a2 - b[R_C + 2]);
        (dx, dy, dz) = unrotate(b, r0, r1, r2);
    } else {
        let (cs, sn) = (b[R_COS], b[R_SIN]);
        let (px, pz) = (a0 - b[R_C], a2 - b[R_C + 2]);
        ox = cs * px - sn * pz;
        oy = a1 - b[R_C + 1];
        oz = sn * px + cs * pz;
        dx = cs * r0 - sn * r2;
        dy = r1;
        dz = sn * r0 + cs * r2;
    }
    let h = [b[R_H], b[R_H + 1], b[R_H + 2]];
    let (mut t0, mut t1) = (-INF, INF);
    let (ov, dv) = ([ox, oy, oz], [dx, dy, dz]);
    for ax in 0..3 {
        if dv[ax].abs() < 1e-12 {
            if ov[ax].abs() > h[ax] {
                return None;
            }
            continue;
        }
        let mut ta = (-h[ax] - ov[ax]) / dv[ax];
        let mut tb = (h[ax] - ov[ax]) / dv[ax];
        if ta > tb {
            core::mem::swap(&mut ta, &mut tb);
        }
        if ta > t0 {
            t0 = ta;
        }
        if tb < t1 {
            t1 = tb;
        }
        if t0 > t1 {
            return None;
        }
    }
    let mut depth = js_min(js_min(h[0] - ox.abs(), h[1] - oy.abs()), h[2] - oz.abs());
    if kind & K_ROUND != 0 {
        let (qa, qb, qc) = (dx * dx + dz * dz, ox * dx + oz * dz, ox * ox + oz * oz - h[0] * h[0]);
        if qa < 1e-18 {
            if qc > 0.0 {
                return None;
            }
        } else {
            let disc = qb * qb - qa * qc;
            if disc < 0.0 {
                return None;
            }
            let s = sqrt(disc);
            let (ta, tb) = ((-qb - s) / qa, (-qb + s) / qa);
            if ta > t0 {
                t0 = ta;
            }
            if tb < t1 {
                t1 = tb;
            }
            if t0 > t1 {
                return None;
            }
        }
        depth = js_min(h[0] - sqrt(ox * ox + oz * oz), h[1] - oy.abs());
    } else if kind & K_CUT != 0 {
        let (planes, n) = planes_of(o, b);
        let (c0, c1, _) = cut_clip(planes, n, ox, oy, oz, dx, dy, dz, t0, t1)?;
        t0 = c0;
        t1 = c1;
        let mut k = 0;
        while k < n && k < CUT_PLANES {
            let p = &planes[k * 4..k * 4 + 4];
            if p[0] == 0.0 && p[1] == 0.0 && p[2] == 0.0 {
                break;
            }
            depth = js_min(depth, p[3] - (p[0] * ox + p[1] * oy + p[2] * oz));
            k += 1;
        }
    }
    Some((t0, t1, depth))
}

/// collision.ts dynamicHit: the leaves of list `set` against a segment already reduced to origin + unit direction + length
fn dynamic_hit(o: &World, set: usize, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, need: u32) -> bool {
    let n = o.leaves[set] as usize;
    let uu = r0 * r0 + r2 * r2;
    let su = sqrt(uu);
    let lu = len * uu;
    for i in 0..n {
        let at = set * MAX_LEAVES + i;
        if o.leaf_flags[at] & need != need {
            continue;
        }
        let b = &o.leaf[at * REC..at * REC + REC];
        if (b[R_KIND] as u32) & K_SHAPED != 0 {
            if seg_clear(a0, a1, a2, r0, r1, r2, len, b, SEG_PAD) {
                continue;
            }
            if let Some(t) = ray_box_t(o, a0, a1, a2, r0, r1, r2, b, len) {
                if t >= 0.0 {
                    return true;
                }
            }
            continue;
        }
        let rr = neg_up(b[R_H]) + neg_up(b[R_H + 2]) + SEG_PAD;
        let (m0, m2) = (b[R_C] - a0, b[R_C + 2] - a2);
        let cr = m0 * r2 - m2 * r0;
        if cr * cr > rr * rr * uu {
            continue;
        }
        let (s, e) = (m0 * r0 + m2 * r2, rr * su);
        if s < -e || s > lu + e {
            continue;
        }
        if ray_box_hits(a0, a1, a2, r0, r1, r2, b, len) {
            return true;
        }
    }
    false
}

/// collision.ts segmentBlockedDynamic, past its length: is the segment crossed by a leaf of list `set` carrying every flag of `need`
pub fn leaves_blocked(w: usize, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, need: u32, set: i32) -> bool {
    if set < 0 {
        return false;
    }
    let (d0, d1, d2) = (b0 - a0, b1 - a1, b2 - a2);
    let il = 1.0 / len;
    dynamic_hit(world(w), set as usize, a0, a1, a2, d0 * il, d1 * il, d2 * il, len, need)
}

/// what the walk was asked beside the segment
pub const ASK_GLASS: u32 = 1; // the game has a rule for an open pane (glass.rs): glass crossed where it is open is not in the way
pub const ASK_PANES: u32 = 2; // a line of fire: glass in the way is listed, and every cell is walked in the old order (the list's order is the scan's)

/// A PANE OF GLASS IN THE WAY OF A LINE that entered it `t0` along (collision.ts segmentBlocked's three statements for one): 0 = the line goes on (the
/// pane is open there by the game's rule, or it is a line of fire and the pane is noted for the penetration rule) · −1 = the pane stops it · −2 = the
/// list of panes is full
#[inline]
#[allow(clippy::too_many_arguments)]
fn pane_in_way(o: &mut World, rule: bool, panes: bool, bi: usize, t0: f64, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64) -> i32 {
    if rule {
        let te = js_max(0.0, t0);
        if glass::open(o, bi, a0 + r0 * te, a1 + r1 * te, a2 + r2 * te) {
            return 0;
        }
    }
    if !panes {
        return -1;
    }
    if o.note(bi, t0) {
        0
    } else {
        -2
    }
}

/// collision.ts segmentBlocked, past its length (`len` = the host's own length of b − a, at least 1e-4 and finite): −1 = something solid is in the way ·
/// n ≥ 0 = nothing solid is, and n panes of glass are (listed in pane_bi / pane_t: 0 unless it is a line of fire) · −2 = more glass than the list holds
/// (the host asks its own walk instead)
pub fn blocked(w: usize, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, shrink: f64, through: u32, set: i32, ask: u32) -> i32 {
    let o = world(w);
    let (d0, d1, d2) = (b0 - a0, b1 - a1, b2 - a2);
    let il = 1.0 / len;
    let (r0, r1, r2) = (d0 * il, d1 * il, d2 * il);
    if set >= 0 && dynamic_hit(o, set as usize, a0, a1, a2, r0, r1, r2, len, 0) {
        return -1;
    }
    let id = o.ask();
    let steps = -floor(-(len / 0.5)) + 1.0;
    let (cs, gw, gh) = (o.cs, o.w as f64, o.h as f64);
    let wi = o.w as usize;
    let skip = through | F_NO_SHADOW;
    let pad = SEG_PAD + (if shrink < 0.0 { -shrink } else { 0.0 });
    let uu = r0 * r0 + r2 * r2;
    let su = sqrt(uu);
    let lu = len * uu;
    let e1 = r1 * len;
    let lxz = sqrt(d0 * d0 + d2 * d2);
    let (gx, gz, g0) = (gw * cs - pad - 0.25, gh * cs - pad - 0.25, pad + 0.25);
    let panes = ask & ASK_PANES != 0;
    let rule = ask & ASK_GLASS != 0;
    let near = !panes && lxz > 1e-6 && pad < 0.5 && a0 > g0 && a0 < gx && b0 > g0 && b0 < gx && a2 > g0 && a2 < gz && b2 > g0 && b2 < gz;
    let mut n = 0usize;
    o.panes = 0;
    if near {
        for k in 0..o.n_tilted as usize {
            let tl = &o.tilted[k * 5..k * 5 + 5];
            let bi = tl[0] as usize;
            o.stamp[bi] = id;
            let fl = o.flags[bi];
            if fl & skip != 0 {
                continue;
            }
            let b = &o.rec[bi * REC..bi * REC + REC];
            if seg_clear(a0, a1, a2, r0, r1, r2, len, b, pad) {
                continue;
            }
            let (x0, x1, z0, z1) = ((tl[1] - 1) as f64, (tl[2] + 1) as f64, (tl[3] - 1) as f64, (tl[4] + 1) as f64);
            let mut met = false;
            let mut i = 0.0;
            while i <= steps {
                let t = (i / steps) * len;
                let (x, z) = (floor((a0 + r0 * t) / cs), floor((a2 + r2 * t) / cs));
                if x >= x0 && x <= x1 && z >= z0 && z <= z1 {
                    met = true;
                    break;
                }
                i += 1.0;
            }
            if !met {
                continue;
            }
            let (t0, t1, depth) = match line_box(o, a0, a1, a2, r0, r1, r2, b) {
                Some(s) => s,
                None => continue,
            };
            if depth > shrink {
                continue;
            }
            if t1 > shrink && t0 < len - shrink {
                if fl & F_GLASS != 0 {
                    match pane_in_way(o, rule, panes, bi, t0, a0, a1, a2, r0, r1, r2) {
                        0 => continue,
                        said => return said,
                    }
                }
                return -1;
            }
        }
        let q = pad + 1e-3;
        let x_major = neg_up(d0) >= neg_up(d2);
        let (am, bm, an, slope) = if x_major { (a0, b0, a2, d2 / d0) } else { (a2, b2, a0, d0 / d2) };
        let (lo, hi) = if am < bm { (am, bm) } else { (bm, am) };
        let (mm, nn) = if x_major { (gw, gh) } else { (gh, gw) };
        let mut c0 = floor((lo - q) / cs);
        let mut c1 = floor((hi + q) / cs);
        if c0 < 0.0 {
            c0 = 0.0;
        }
        if c1 > mm - 1.0 {
            c1 = mm - 1.0;
        }
        let fwd = bm >= am;
        let mut j = c0;
        while j <= c1 {
            let c = if fwd { j } else { c1 - (j - c0) };
            let mut s0 = c * cs - q;
            let mut s1 = (c + 1.0) * cs + q;
            if s0 < lo {
                s0 = lo;
            }
            if s1 > hi {
                s1 = hi;
            }
            let (n0, n1) = (an + (s0 - am) * slope, an + (s1 - am) * slope);
            let mut q0 = floor(((if n0 < n1 { n0 } else { n1 }) - q) / cs);
            let mut q1 = floor(((if n0 < n1 { n1 } else { n0 }) + q) / cs);
            if q0 < 0.0 {
                q0 = 0.0;
            }
            if q1 > nn - 1.0 {
                q1 = nn - 1.0;
            }
            let ci = c as usize;
            let mut r = q0;
            while r <= q1 {
                let ri = r as usize;
                o.seg_cells[n] = (if x_major { ri * wi + ci } else { ci * wi + ri }) as u32;
                n += 1;
                r += 1.0;
            }
            j += 1.0;
        }
    } else {
        let (mut lx, mut lz) = (f64::NAN, f64::NAN);
        let mut i = 0.0;
        while i <= steps {
            let t = (i / steps) * len;
            let (x, z) = (floor((a0 + r0 * t) / cs), floor((a2 + r2 * t) / cs));
            i += 1.0;
            if x == lx && z == lz {
                continue;
            }
            lx = x;
            lz = z;
            let mut dz = -1.0;
            while dz <= 1.0 {
                let mut dx = -1.0;
                while dx <= 1.0 {
                    let (cx, cz) = (x + dx, z + dz);
                    dx += 1.0;
                    if cx < 0.0 || cz < 0.0 || cx >= gw || cz >= gh {
                        continue;
                    }
                    let ci = cz as usize * wi + cx as usize;
                    if o.cell_stamp[ci] == id {
                        continue;
                    }
                    o.cell_stamp[ci] = id;
                    o.seg_cells[n] = ci as u32;
                    n += 1;
                }
                dz += 1.0;
            }
        }
    }
    for j in 0..n {
        let cell = o.seg_cells[j] as usize;
        let (from, to) = (o.start[cell] as usize, o.start[cell + 1] as usize);
        for k in from..to {
            let bi = o.items[k] as usize;
            if o.stamp[bi] == id {
                continue;
            }
            o.stamp[bi] = id;
            let fl = o.flags[bi];
            if fl & skip != 0 {
                continue;
            }
            let b = &o.rec[bi * REC..bi * REC + REC];
            let (mut t0, mut t1) = (-INF, INF);
            let depth;
            if (b[R_KIND] as u32) & K_SHAPED != 0 {
                if seg_clear(a0, a1, a2, r0, r1, r2, len, b, pad) {
                    continue;
                }
                match line_box(o, a0, a1, a2, r0, r1, r2, b) {
                    Some(s) => {
                        t0 = s.0;
                        t1 = s.1;
                        depth = s.2;
                    }
                    None => continue,
                }
            } else {
                let (h0, h1, h2) = (b[R_H], b[R_H + 1], b[R_H + 2]);
                let hy = neg_up(h1);
                let m1 = b[R_C + 1] - a1;
                if if e1 > 0.0 { m1 - hy > e1 + pad || m1 + hy < -pad } else { m1 - hy > pad || m1 + hy < e1 - pad } {
                    continue;
                }
                let rr = neg_up(h0) + neg_up(h2) + pad;
                let (m0, m2) = (b[R_C] - a0, b[R_C + 2] - a2);
                let cr = m0 * r2 - m2 * r0;
                if cr * cr > rr * rr * uu {
                    continue;
                }
                let (sa, ea) = (m0 * r0 + m2 * r2, rr * su);
                if sa < -ea || sa > lu + ea {
                    continue;
                }
                // lineBox over scalars: its statements in its order
                let (co, sn) = (b[R_COS], b[R_SIN]);
                let (qx, qz) = (a0 - b[R_C], a2 - b[R_C + 2]);
                let (ox, oy, oz) = (co * qx - sn * qz, a1 - b[R_C + 1], sn * qx + co * qz);
                let (ex, ez) = (co * r0 - sn * r2, sn * r0 + co * r2);
                if ex.abs() < 1e-12 {
                    if ox.abs() > h0 {
                        continue;
                    }
                } else {
                    let (mut ta, mut tb) = ((-h0 - ox) / ex, (h0 - ox) / ex);
                    if ta > tb {
                        core::mem::swap(&mut ta, &mut tb);
                    }
                    if ta > t0 {
                        t0 = ta;
                    }
                    if tb < t1 {
                        t1 = tb;
                    }
                    if t0 > t1 {
                        continue;
                    }
                }
                if r1.abs() < 1e-12 {
                    if oy.abs() > h1 {
                        continue;
                    }
                } else {
                    let (mut ta, mut tb) = ((-h1 - oy) / r1, (h1 - oy) / r1);
                    if ta > tb {
                        core::mem::swap(&mut ta, &mut tb);
                    }
                    if ta > t0 {
                        t0 = ta;
                    }
                    if tb < t1 {
                        t1 = tb;
                    }
                    if t0 > t1 {
                        continue;
                    }
                }
                if ez.abs() < 1e-12 {
                    if oz.abs() > h2 {
                        continue;
                    }
                } else {
                    let (mut ta, mut tb) = ((-h2 - oz) / ez, (h2 - oz) / ez);
                    if ta > tb {
                        core::mem::swap(&mut ta, &mut tb);
                    }
                    if ta > t0 {
                        t0 = ta;
                    }
                    if tb < t1 {
                        t1 = tb;
                    }
                    if t0 > t1 {
                        continue;
                    }
                }
                depth = js_min(js_min(h0 - ox.abs(), h1 - oy.abs()), h2 - oz.abs());
            }
            if depth > shrink {
                continue;
            }
            if t1 > shrink && t0 < len - shrink {
                if fl & F_GLASS != 0 {
                    match pane_in_way(o, rule, panes, bi, t0, a0, a1, a2, r0, r1, r2) {
                        0 => continue,
                        said => return said,
                    }
                }
                return -1;
            }
        }
    }
    o.panes as i32
}

// ---- a line asked from inside the module (the senses: sight.rs, light.rs, smoke.rs, hear.rs, nav.rs), from its two ends
/// A LINE'S LENGTH as the game works it out — or what a line answers with no walk at all: nothing is between two points that are one (Some(false)), and a
/// length that is no length of this world's is the host's to judge (None).
#[inline]
fn walked(a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Result<f64, Option<bool>> {
    let len = hypot3(b0 - a0, b1 - a1, b2 - a2);
    if len < 1e-4 {
        Err(Some(false))
    } else if !(len < 1e6) {
        Err(None)
    } else {
        Ok(len)
    }
}
/// IS A MAN HIDDEN FROM AN EYE (fogofwar.ts lineToMan, guards.ts eyeContact): the chest first, and the head only behind a blocked chest
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn man_hidden(w: usize, set: i32, through: u32, e0: f64, e1: f64, e2: f64, c0: f64, c1: f64, c2: f64, h0: f64, h1: f64, h2: f64) -> Option<bool> {
    match sight_line(w, set, through, e0, e1, e2, c0, c1, c2) {
        Some(true) => sight_line(w, set, through, e0, e1, e2, h0, h1, h2),
        said => said,
    }
}
/// collision.ts segmentBlocked(a, b, 0.02, through) for a line that PASSES glass (`through` carries the glass flag, so no pane is ever the host's to judge):
/// the length is the game's own, worked out here. None = not a line this module walks (a length that is no length of this world's): the host asks its own.
#[inline]
pub fn sight_line(w: usize, set: i32, through: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Option<bool> {
    let len = match walked(a0, a1, a2, b0, b1, b2) {
        Ok(len) => len,
        Err(said) => return said,
    };
    Some(blocked(w, a0, a1, a2, b0, b1, b2, len, 0.02, through, set, 0) == -1)
}
/// collision.ts solidBlocked(a, b): the same line with nothing passed. `ask` = ASK_GLASS where the game has a rule for an open pane. None = a line too
/// long to be a line of this world: the host's to judge.
#[inline]
pub fn solid_line(w: usize, set: i32, ask: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Option<bool> {
    let len = match walked(a0, a1, a2, b0, b1, b2) {
        Ok(len) => len,
        Err(said) => return said,
    };
    match blocked(w, a0, a1, a2, b0, b1, b2, len, 0.02, 0, set, ask) {
        -1 => Some(true),
        0 => Some(false),
        _ => None,
    }
}
/// collision.ts segmentBlockedDynamic(a, b, need) from its two ends
#[inline]
pub fn leaf_line(w: usize, set: i32, need: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Option<bool> {
    let len = match walked(a0, a1, a2, b0, b1, b2) {
        Ok(len) => len,
        Err(said) => return said,
    };
    Some(leaves_blocked(w, a0, a1, a2, b0, b1, b2, len, need, set))
}

// ---- THE RAY: the nearest thing along it, how far, and the face it is entered by (collision.ts raycast, boxes.ts rayBox)

/// one slab of rayBox with the face it enters by: the interval cut down by the faces at ±h along axis `k`; the entry's axis and sign kept. false = nothing left
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn slab_n(h: f64, o: f64, inv: f64, t0: &mut f64, t1: &mut f64, axis: &mut i32, sgn: &mut f64, k: i32) -> bool {
    let mut ta = (-h - o) * inv;
    let mut tb = (h - o) * inv;
    let mut s = -1.0;
    if ta > tb {
        core::mem::swap(&mut ta, &mut tb);
        s = 1.0;
    }
    if ta > *t0 {
        *t0 = ta;
        *axis = k;
        *sgn = s;
    }
    if tb < *t1 {
        *t1 = tb;
    }
    !(*t0 > *t1)
}
/// a direction out of a fully rotated box's frame: quat.rotate(rot, v), its expression over scalars
#[inline(always)]
fn rotate(b: &[f64], vx: f64, vy: f64, vz: f64) -> (f64, f64, f64) {
    let (qx, qy, qz, qw) = (b[R_ROT], b[R_ROT + 1], b[R_ROT + 2], b[R_ROT + 3]);
    let (tx, ty, tz) = (2.0 * (qy * vz - qz * vy), 2.0 * (qz * vx - qx * vz), 2.0 * (qx * vy - qy * vx));
    (vx + qw * tx + (qy * tz - qz * ty), vy + qw * ty + (qz * tx - qx * tz), vz + qw * tz + (qx * ty - qy * tx))
}
/// a face normal of the box's own frame, in the world's: by its rotation, or turned by its yaw (the cos and sin the copy keeps beside it)
#[inline(always)]
fn out_of(b: &[f64], l0: f64, l1: f64, l2: f64) -> (f64, f64, f64) {
    if (b[R_KIND] as u32) & K_ROT != 0 {
        rotate(b, l0, l1, l2)
    } else {
        let (cs, sn) = (b[R_COS], b[R_SIN]);
        (cs * l0 + sn * l2, l1, -sn * l0 + cs * l2)
    }
}
/// the unit vector of axis `k`, times `sgn` (the other two the literal zeros the TypeScript writes)
#[inline(always)]
fn face_of(axis: i32, sgn: f64) -> (f64, f64, f64) {
    (if axis == 0 { sgn } else { 0.0 }, if axis == 1 { sgn } else { 0.0 }, if axis == 2 { sgn } else { 0.0 })
}

/// boxes.ts rayBox, whole (a round box, a cut one, a fully rotated one, a plain yawed one, asked in that order): where the ray enters the box within tmax
/// and the normal of the face it enters by. None = it does not, or it starts inside (the rule rayBox documents).
#[allow(clippy::too_many_arguments)]
fn ray_box_n(o: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64], tmax: f64) -> Option<(f64, f64, f64, f64)> {
    let kind = b[R_KIND] as u32;
    if kind & K_ROUND != 0 {
        // rayBoxRound: face 0 = none entered (the origin is inside), 1 = an end, 2 = the side
        let (ox, oy, oz, dx, dy, dz) = to_local(a0, a1, a2, r0, r1, r2, b);
        let (rr, hy) = (b[R_H], b[R_H + 1]);
        let (mut t0, mut t1, mut face, mut sgn) = (0.0, tmax, -1i32, 0.0);
        if !slab_n(hy, oy, inv_or(dy), &mut t0, &mut t1, &mut face, &mut sgn, 1) {
            return None;
        }
        let (qa, hb, qc) = (dx * dx + dz * dz, ox * dx + oz * dz, ox * ox + oz * oz - rr * rr);
        if qa < 1e-18 {
            if qc > 0.0 {
                return None;
            }
        } else {
            let disc = hb * hb - qa * qc;
            if disc < 0.0 {
                return None;
            }
            let s = sqrt(disc);
            let (ta, tb) = ((-hb - s) / qa, (-hb + s) / qa);
            if ta > t0 {
                t0 = ta;
                face = 2;
            }
            if tb < t1 {
                t1 = tb;
            }
            if t0 > t1 {
                return None;
            }
        }
        if face < 0 {
            return None;
        }
        let (mut l0, mut l1, mut l2) = (0.0, sgn, 0.0);
        if face == 2 {
            let (hx, hz) = (ox + dx * t0, oz + dz * t0);
            let len = sqrt(hx * hx + hz * hz);
            let il = 1.0 / (if len != 0.0 && len == len { len } else { 1.0 });
            l0 = hx * il;
            l1 = 0.0;
            l2 = hz * il;
        }
        let (n0, n1, n2) = out_of(b, l0, l1, l2);
        return Some((t0, n0, n1, n2));
    }
    if kind & K_CUT != 0 {
        // rayBoxCut
        let (ox, oy, oz, dx, dy, dz) = to_local(a0, a1, a2, r0, r1, r2, b);
        let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1i32, 0.0);
        if !slab_n(b[R_H], ox, inv_or(dx), &mut t0, &mut t1, &mut axis, &mut sgn, 0) {
            return None;
        }
        if !slab_n(b[R_H + 1], oy, inv_or(dy), &mut t0, &mut t1, &mut axis, &mut sgn, 1) {
            return None;
        }
        if !slab_n(b[R_H + 2], oz, inv_or(dz), &mut t0, &mut t1, &mut axis, &mut sgn, 2) {
            return None;
        }
        let (planes, n) = planes_of(o, b);
        let (c0, _c1, k) = cut_clip(planes, n, ox, oy, oz, dx, dy, dz, t0, t1)?;
        if k < 0 && axis < 0 {
            return None;
        }
        let (l0, l1, l2) = if k >= 0 {
            let p = &planes[k as usize * 4..k as usize * 4 + 3];
            (p[0], p[1], p[2])
        } else {
            face_of(axis, sgn)
        };
        let (n0, n1, n2) = out_of(b, l0, l1, l2);
        return Some((c0, n0, n1, n2));
    }
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1i32, 0.0);
    if kind & K_ROT != 0 {
        // rayBoxRot
        let (ox, oy, oz) = unrotate(b, a0 - b[R_C], a1 - b[R_C + 1], a2 - b[R_C + 2]);
        let (dx, dy, dz) = unrotate(b, r0, r1, r2);
        if !slab_n(b[R_H], ox, inv_or(dx), &mut t0, &mut t1, &mut axis, &mut sgn, 0) {
            return None;
        }
        if !slab_n(b[R_H + 1], oy, inv_or(dy), &mut t0, &mut t1, &mut axis, &mut sgn, 1) {
            return None;
        }
        if !slab_n(b[R_H + 2], oz, inv_or(dz), &mut t0, &mut t1, &mut axis, &mut sgn, 2) {
            return None;
        }
        if axis < 0 {
            return None;
        }
        let (l0, l1, l2) = face_of(axis, sgn);
        let (n0, n1, n2) = rotate(b, l0, l1, l2);
        return Some((t0, n0, n1, n2));
    }
    // the plain yawed box
    let (cs, sn) = (b[R_COS], b[R_SIN]);
    let (px, py, pz) = (a0 - b[R_C], a1 - b[R_C + 1], a2 - b[R_C + 2]);
    let (lx, lz, ly) = (cs * px - sn * pz, sn * px + cs * pz, py);
    let (dx, dz, dy) = (cs * r0 - sn * r2, sn * r0 + cs * r2, r1);
    let (ix, iy, iz) = (inv_or(dx), inv_or(dy), inv_or(dz));
    if !slab_n(b[R_H], lx, ix, &mut t0, &mut t1, &mut axis, &mut sgn, 0) {
        return None;
    }
    if !slab_n(b[R_H + 1], ly, iy, &mut t0, &mut t1, &mut axis, &mut sgn, 1) {
        return None;
    }
    if !slab_n(b[R_H + 2], lz, iz, &mut t0, &mut t1, &mut axis, &mut sgn, 2) {
        return None;
    }
    if axis < 0 {
        return None;
    }
    let (l0, l1, l2) = face_of(axis, sgn);
    Some((t0, cs * l0 + sn * l2, l1, -sn * l0 + cs * l2))
}

/// the ray's answer when nothing is met, and when the host must answer it itself (a ray that is not a number)
pub const RAY_NONE: i32 = -1;
pub const RAY_HOST: i32 = -2;
/// what a leaf's index is offset by in the answer (collision.ts raycast: index ≥ 1e6 = a dynamic occluder)
pub const RAY_LEAF: i32 = 1_000_000;

/// collision.ts raycast: the nearest thing the ray from a along the unit r meets within tmax — the leaves of list `set` first (−1: none), then the statics of
/// the cells round the ray, walked in half-metre steps. → RAY_NONE · RAY_HOST · the
/// static's index, or RAY_LEAF + the leaf's; and into `out` how far along, then the face's normal. `ask` = the game has a rule for an open pane: a pane
/// met where it is open is passed, and the ray flies on to what stands beyond.
#[allow(clippy::too_many_arguments)]
pub fn ray(w: usize, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, set: i32, ask: bool, out: &mut [f64]) -> i32 {
    ray_but(w, a0, a1, a2, r0, r1, r2, tmax, set, -1, ask, out)
}
/// …with ONE LEAF OF THE LIST LEFT OUT, `but` (−1: none): the ray of a thing that is itself one of the leaves (a slab off its hinges is no wall to
/// itself: doorblast.ts withoutLeaf). The leaves keep their own indices in the answer.
#[allow(clippy::too_many_arguments)]
pub fn ray_but(w: usize, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, set: i32, but: i32, ask: bool, out: &mut [f64]) -> i32 {
    let o = world(w);
    let (mut best, mut bi, mut bn) = (tmax, -1i32, (0.0, 1.0, 0.0));
    if set >= 0 {
        let s = set as usize;
        for i in 0..o.leaves[s] as usize {
            if i as i32 == but {
                continue;
            }
            let at = (s * MAX_LEAVES + i) * REC;
            if let Some((t, n0, n1, n2)) = ray_box_n(o, a0, a1, a2, r0, r1, r2, &o.leaf[at..at + REC], best) {
                if t < best {
                    best = t;
                    bi = RAY_LEAF + i as i32;
                    bn = (n0, n1, n2);
                }
            }
        }
    }
    let id = o.ask();
    let steps = js_ceil(tmax / 0.5) + 1.0;
    let (cs, gw, gh) = (o.cs, o.w as f64, o.h as f64);
    let wi = o.w as usize;
    let mut i = 0.0;
    while i <= steps {
        let t = (i / steps) * tmax;
        if t > best + 1.5 {
            break;
        }
        let (x, z) = (floor((a0 + r0 * t) / cs), floor((a2 + r2 * t) / cs));
        if x != x || z != z {
            return RAY_HOST; // (a ray that is not a number: the host's walk fails on it loudly, and goes on doing so)
        }
        let mut dz = -1.0;
        while dz <= 1.0 {
            let mut dx = -1.0;
            while dx <= 1.0 {
                let (cx, cz) = (x + dx, z + dz);
                dx += 1.0;
                if !(cx >= 0.0 && cz >= 0.0 && cx < gw && cz < gh) {
                    continue;
                }
                let c = cz as usize * wi + cx as usize;
                let (k0, k1) = (o.start[c] as usize, o.start[c + 1] as usize);
                for k in k0..k1 {
                    let b = o.items[k] as usize;
                    if o.stamp[b] == id {
                        continue;
                    }
                    o.stamp[b] = id;
                    let fl = o.flags[b];
                    if fl & F_NO_SHADOW != 0 {
                        continue;
                    }
                    if let Some((t, n0, n1, n2)) = ray_box_n(o, a0, a1, a2, r0, r1, r2, &o.rec[b * REC..b * REC + REC], best) {
                        if t < best {
                            if ask && fl & F_GLASS != 0 && glass::open(o, b, a0 + r0 * t, a1 + r1 * t, a2 + r2 * t) {
                                continue;
                            }
                            best = t;
                            bi = b as i32;
                            bn = (n0, n1, n2);
                        }
                    }
                }
            }
            dz += 1.0;
        }
        i += 1.0;
    }
    if bi < 0 {
        return RAY_NONE;
    }
    out[0] = best;
    out[1] = bn.0;
    out[2] = bn.1;
    out[3] = bn.2;
    bi
}

/// A RAY ASKED BY A RULE OF THE MODULE'S (a slab off its hinges, a thrown thing): collision.ts raycast as a rule over there gets it, with what only the
/// host can answer said as RAY_HOST — a ray of no length or of no number (collision.ts walks those itself). `set`, `but` and `ask` as ray_but takes
/// them. → RAY_NONE · RAY_HOST · the static's index, or RAY_LEAF + the leaf's; `out` as ray writes it.
#[allow(clippy::too_many_arguments)]
pub fn ray_rule(w: usize, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, set: i32, but: i32, ask: bool, out: &mut [f64]) -> i32 {
    if !(tmax > 0.0 && tmax < 1e6) {
        return RAY_HOST;
    }
    ray_but(w, a0, a1, a2, r0, r1, r2, tmax, set, but, ask, out)
}

/// where ray_things says which part it met, after how far along and the normal
pub const RAY_PART: usize = 4;
/// game/props.ts PropSystem.raycast: the nearest PART of any thing of furniture the ray from a along the unit r meets within tmax — the things in the
/// furniture's own order (the world's `thing_live`), each passed by if the upright cylinder that holds it is clear of the ray in plan or lies behind it
/// or beyond the best so far, its parts (list THING_SET of the leaves) in their order. → −1, or which thing of the level's list; and into `out` how far
/// along, the face's normal, and which part of the list.
#[allow(clippy::too_many_arguments)]
pub fn ray_things(w: usize, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, out: &mut [f64]) -> i32 {
    let o = world(w);
    let (mut best, mut bp, mut bn, mut bb) = (tmax, -1i32, (0.0, 1.0, 0.0), 0usize);
    let fl = crate::maths::hypot2(r0, r2);
    for r in 0..o.things as usize {
        let (id, start, count) = (o.thing_live[r * TL] as usize, o.thing_live[r * TL + 1] as usize, o.thing_live[r * TL + 2] as usize);
        if id >= MAX_THINGS || start + count > MAX_LEAVES {
            continue;
        }
        let radius = o.thing[id * TH + TH_RADIUS];
        let (ox, oz) = (o.thing[id * TH + TH_X] - a0, o.thing[id * TH + TH_Z] - a2);
        let along = if fl > 1e-6 { (ox * r0 + oz * r2) / fl } else { 0.0 };
        if fl > 1e-6 && (along < -radius || along > best * fl + radius) {
            continue;
        }
        let perp2 = ox * ox + oz * oz - along * along;
        if perp2 > (radius + 0.05) * (radius + 0.05) {
            continue;
        }
        for i in start..start + count {
            let at = (THING_SET * MAX_LEAVES + i) * REC;
            if let Some((t, n0, n1, n2)) = ray_box_n(o, a0, a1, a2, r0, r1, r2, &o.leaf[at..at + REC], best) {
                if t < best {
                    best = t;
                    bp = r as i32;
                    bn = (n0, n1, n2);
                    bb = i;
                }
            }
        }
    }
    if bp < 0 {
        return -1;
    }
    out[0] = best;
    out[1] = bn.0;
    out[2] = bn.1;
    out[3] = bn.2;
    out[RAY_PART] = bb as f64;
    bp
}
