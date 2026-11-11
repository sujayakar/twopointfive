//! THE LINE TESTS THROUGH THE BOX WORLD: is the segment a → b blocked by a static or a leaf (`blocked`, `leaves_blocked`, and from two ends `sight_line`, `solid_line`,
//! `leaf_line`), over the rows of the box world.
//!
//! IT KEEPS NOTHING OF A GAME. A question reads the rows and writes only the scratch of one question (world.rs `Asking`: what it has met, the cells it walked, the panes it
//! lists, and a guess at what stops it first, never an answer). No question is asked inside another.
//!
//! THE ORDER IS PART OF THE ANSWER: the cells are walked in a written order, the boxes met in that order, the slab arithmetic done on the same doubles in the same order. The
//! panes a line lists, a pane's hit and a ray's nearest entry all hang on it, so changing it changes the game.
//!
//! THE RAY (`ray`: where a ray first enters anything, how far along, and the face it enters by) is here too: the
//! leaves first, the cells in half-metre steps, the nearest entry kept. Every rule that flies something through a level
//! asks it (a torn leaf, a thrown can, a round).
//!
//! WHETHER A PANE IS OPEN where a line or a ray meets it is answered here (glass.rs, over the rows of every pane that is no longer whole): a line that
//! crosses a hole is not stopped by the pane, and a ray flies on through it to what stands beyond.
//!
//! What is not here is whether a round gets through the panes on its line (pen.rs). For a line of fire the walk LISTS the glass the line crosses that is not open
//! there (which pane, and where the line enters it) and goes on; the caller judges the list when the walk is back.

use game_types::knobs::Knobs;
use crate::glass;
use crate::maths::hypot3;
use crate::world::*;
use prim::{floor, sqrt};

/// how far past a box's own bounds the early rejects still count it as near
const SEG_PAD: f64 = 0.05;
const INF: f64 = f64::INFINITY;

/// Can the segment be said, without the slab test, to come nowhere near the box
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

/// one slab of a ray against a box: the interval cut down by the pair of faces at ±h along an axis; false = nothing left
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

/// Does the ray enter this yawed box within tmax
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
/// the ray in the box's own frame, as the round and the cut walks want it (rotated, or turned by the yaw)
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

/// The interval cut down by the record's planes. (t0, t1, k) — k the plane entered through, −1 if t0 did not move — or None = nothing left
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
/// a cut box's planes, in the table its record's place is of (the world's, or a question's own extra boxes')
#[inline(always)]
fn planes_of<'a>(cut: &'a [f64], b: &[f64]) -> (&'a [f64], usize) {
    let at = b[R_CUT] as usize * CUT_PLANES * 4;
    (&cut[at..at + CUT_PLANES * 4], b[R_CUTN] as usize)
}

/// a ray against a box that is not a plain yawed one (round, cut or rotated): where the ray enters it, if it does within tmax (ray_box_n's answer
/// without the face: one walk of each shape in this file)
fn ray_box_t(cut: &[f64], a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64], tmax: f64) -> Option<f64> {
    ray_box_n(cut, a0, a1, a2, r0, r1, r2, b, tmax).map(|h| h.0)
}

/// The interval [t0, t1] over which the LINE lies inside the box, and how deep inside it the origin sits; None = the line misses it
pub fn line_box(o: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64]) -> Option<(f64, f64, f64)> {
    let cut = &o.cut[..];
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
    if !(slab_span(ox, dx, h[0], &mut t0, &mut t1) && slab_span(oy, dy, h[1], &mut t0, &mut t1) && slab_span(oz, dz, h[2], &mut t0, &mut t1)) {
        return None;
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
        let (planes, n) = planes_of(cut, b);
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

/// One loose box against a segment already reduced to origin + unit direction + length
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn loose_hit(cut: &[f64], b: &[f64], a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, uu: f64, su: f64, lu: f64) -> bool {
    if (b[R_KIND] as u32) & K_SHAPED != 0 {
        if seg_clear(a0, a1, a2, r0, r1, r2, len, b, SEG_PAD) {
            return false;
        }
        return match ray_box_t(cut, a0, a1, a2, r0, r1, r2, b, len) {
            Some(t) => t >= 0.0,
            None => false,
        };
    }
    let rr = neg_up(b[R_H]) + neg_up(b[R_H + 2]) + SEG_PAD;
    let (m0, m2) = (b[R_C] - a0, b[R_C + 2] - a2);
    let cr = m0 * r2 - m2 * r0;
    if cr * cr > rr * rr * uu {
        return false;
    }
    let (s, e) = (m0 * r0 + m2 * r2, rr * su);
    if s < -e || s > lu + e {
        return false;
    }
    ray_box_hits(a0, a1, a2, r0, r1, r2, b, len)
}
/// …the leaves the question stands with that carry every flag of `need`, and the boxes it adds (which carry none): the first in the way, by its place, or EXTRA
#[allow(clippy::too_many_arguments)]
fn dynamic_hit(o: &World, v: &Over, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, need: u32) -> Option<u32> {
    let uu = r0 * r0 + r2 * r2;
    let su = sqrt(uu);
    let lu = len * uu;
    if v.leaves {
        for (i, (row, fl)) in o.leaf.chunks_exact(REC).zip(&o.leaf_flags).enumerate() {
            if fl & need != need || (!v.but.is_empty() && v.but.contains(&(i as u32))) {
                continue;
            }
            if loose_hit(&o.cut, row, a0, a1, a2, r0, r1, r2, len, uu, su, lu) {
                return Some(i as u32);
            }
        }
    }
    (need == 0 && v.extra.chunks_exact(REC).any(|b| loose_hit(v.extra_cut, b, a0, a1, a2, r0, r1, r2, len, uu, su, lu))).then_some(EXTRA)
}
/// a box the question added
const EXTRA: u32 = u32::MAX;

/// is the segment crossed by a leaf carrying every flag of `need`
pub fn leaves_blocked(w: &World, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, need: u32) -> bool {
    leaves_blocked_over(w, &Over::with(true), a0, a1, a2, b0, b1, b2, len, need)
}
/// …of the world as the question takes it
#[allow(clippy::too_many_arguments)]
fn leaves_blocked_over(w: &World, v: &Over, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, need: u32) -> bool {
    let (d0, d1, d2) = (b0 - a0, b1 - a1, b2 - a2);
    let il = 1.0 / len;
    dynamic_hit(w, v, a0, a1, a2, d0 * il, d1 * il, d2 * il, len, need).is_some()
}

/// what the walk was asked beside the segment
pub const ASK_GLASS: u32 = 1; // the game has a rule for an open pane (glass.rs): glass crossed where it is open is not in the way
pub const ASK_PANES: u32 = 2; // a line of fire: glass in the way is listed, in the order the walk comes on it

/// A PANE OF GLASS IN THE WAY OF A LINE that entered it `t0` along: 0 = the line goes on (the
/// pane is open there by the game's rule, or it is a line of fire and the pane is noted for the penetration rule) · −1 = the pane stops it · −2 = the
/// list of panes is full
#[inline]
#[allow(clippy::too_many_arguments)]
fn pane_in_way(o: &World, s: &mut Asking, rule: bool, panes: bool, bi: usize, t0: f64, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tune: &Knobs) -> i32 {
    if rule {
        let te = js_max(0.0, t0);
        if glass::open(o, bi, a0 + r0 * te, a1 + r1 * te, a2 + r2 * te, tune) {
            return 0;
        }
    }
    if !panes {
        return -1;
    }
    if s.note(bi, t0) {
        0
    } else {
        -2
    }
}

/// IS THE LINE BLOCKED (`len` = the host's own length of b − a, at least 1e-4 and finite): −1 = something solid is in the way ·
/// n ≥ 0 = nothing solid is, and n panes of glass are (listed in pane_bi / pane_t: 0 unless it is a line of fire) · −2 = more glass than the list holds
pub fn blocked(w: &World, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, shrink: f64, through: u32, leaves: bool, ask: u32, tune: &Knobs) -> i32 {
    blocked_over(w, &Over::with(leaves), a0, a1, a2, b0, b1, b2, len, shrink, through, ask, tune)
}
/// …of the world as the question takes it: the leaves it lets through, the statics it lets through, the boxes it adds
#[allow(clippy::too_many_arguments)]
pub fn blocked_over(w: &World, v: &Over, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, shrink: f64, through: u32, ask: u32, tune: &Knobs) -> i32 {
    let (o, s) = (w, asking());
    let (d0, d1, d2) = (b0 - a0, b1 - a1, b2 - a2);
    let il = 1.0 / len;
    let (r0, r1, r2) = (d0 * il, d1 * il, d2 * il);
    let uu = r0 * r0 + r2 * r2;
    let mut seg = Seg {
        o,
        v,
        tune,
        a: [a0, a1, a2],
        b: [b0, b1, b2],
        d: [d0, d1, d2],
        r: [r0, r1, r2],
        len,
        shrink,
        skip: through | F_NO_SHADOW,
        pad: SEG_PAD + (if shrink < 0.0 { -shrink } else { 0.0 }),
        uu,
        su: sqrt(uu),
        lu: len * uu,
        e1: r1 * len,
        id: s.ask(),
        steps: -floor(-(len / 0.5)) + 1.0,
        panes: ask & ASK_PANES != 0,
        rule: ask & ASK_GLASS != 0,
        at: last_at(&[a0, a1, a2], &[b0, b1, b2]),
    };
    let inside = seg.goes() && seg.inside(&seg.a) && seg.inside(&seg.b);
    let guess = s.last[seg.at];
    if guess != 0 && seg.stopped_by(guess, inside) {
        return -1;
    }
    if v.leaves || !v.extra.is_empty() {
        if let Some(leaf) = dynamic_hit(o, v, a0, a1, a2, r0, r1, r2, len, 0) {
            if leaf != EXTRA {
                s.last[seg.at] = LAST_LEAF | leaf;
            }
            return -1;
        }
    }
    s.panes = 0;
    let (foot, top) = if d1 < 0.0 { (b1 - seg.pad, a1 + seg.pad) } else { (a1 - seg.pad, b1 + seg.pad) };
    if !seg.goes() {
        let n = seg.cells_round(s, 0, false);
        return seg.stopped(s, n, foot, top).unwrap_or(s.panes as i32);
    }
    let said = match seg.tilted(s) {
        Some(said) => said,
        None => {
            let mut n = seg.cells_along(s);
            if !inside {
                n = seg.cells_round(s, n, true);
            }
            seg.stopped(s, n, foot, top).unwrap_or(s.panes as i32)
        }
    };
    if said < 2 {
        return said;
    }
    // (the order of one pane is no order; of more it is the walk's)
    seg.id = s.ask();
    s.panes = 0;
    let n = seg.cells_round(s, 0, false);
    seg.stopped(s, n, foot, top).unwrap_or(s.panes as i32)
}

/// what a line was last stopped by, as `Asking::last` keeps it: nothing known, 1 + a static, or this bit and a leaf
const LAST_LEAF: u32 = 1 << 31;
/// THE PLACE OF A LINE'S LAST: by the two metres either end is in. Two lines may share one; what stands there is only a guess.
#[inline(always)]
fn last_at(a: &[f64; 3], b: &[f64; 3]) -> usize {
    let mut h = 0u32;
    for v in [a[0], a[1], a[2], b[0], b[1], b[2]] {
        h = (h ^ (v * 0.5) as i32 as u32).wrapping_mul(0x9e37_79b1);
        h ^= h >> 15;
    }
    h as usize & (LASTS - 1)
}

/// A SEGMENT BEING WALKED: its ends, the way along it and its unit direction, what it passes, and the numbers a box is refused by.
struct Seg<'a> {
    o: &'a World,
    v: &'a Over<'a>,
    tune: &'a Knobs,
    a: [f64; 3],
    b: [f64; 3],
    d: [f64; 3],
    r: [f64; 3],
    len: f64,
    shrink: f64,
    skip: u32,
    pad: f64,
    /// the square of its direction's length in plan, that length, the square by its own, and how far it climbs
    uu: f64,
    su: f64,
    lu: f64,
    e1: f64,
    /// the question's own stamp, and how many half-metre steps it takes
    id: u32,
    steps: f64,
    panes: bool,
    rule: bool,
    /// where what last stopped a line like it is kept
    at: usize,
}
impl Seg<'_> {
    /// What the statics of the cells listed say of it: None = it goes on past them all.
    #[inline(always)]
    fn stopped(&self, s: &mut Asking, n: usize, foot: f64, top: f64) -> Option<i32> {
        let o = self.o;
        for j in 0..n {
            let cell = s.seg_cells[j] as usize;
            for k in o.start[cell] as usize..o.start[cell + 1] as usize {
                let bi = o.items[k] as usize;
                // (one that stands over or under the whole of the line is none of its)
                if o.tall[bi][0] > top || o.tall[bi][1] < foot || s.stamp[bi] == self.id {
                    continue;
                }
                s.stamp[bi] = self.id;
                if self.passes(bi) {
                    continue;
                }
                let Some(span) = self.span(&o.rec[bi * REC..bi * REC + REC]) else {
                    continue;
                };
                if let Some(said) = self.meets(s, bi, span) {
                    if o.flags[bi] & F_GLASS == 0 {
                        s.last[self.at] = bi as u32 + 1;
                    }
                    return Some(said);
                }
            }
        }
        None
    }
    /// whether it goes anywhere in plan, so that its cells can be listed by their columns
    #[inline(always)]
    fn goes(&self) -> bool {
        self.pad < 0.5 && sqrt(self.d[0] * self.d[0] + self.d[2] * self.d[2]) > 1e-6
    }
    /// whether a point of it lies well inside the level (what stands over the level's edge is filed in the cells along it)
    #[inline(always)]
    fn inside(&self, p: &[f64; 3]) -> bool {
        let (o, g0) = (self.o, self.pad + 0.25);
        p[0] > g0 && p[0] < o.w as f64 * o.cs - g0 && p[2] > g0 && p[2] < o.h as f64 * o.cs - g0
    }
    /// whether it passes a static whatever lies between: of a kind it was told to pass, or one the question lets it through
    #[inline(always)]
    fn passes(&self, bi: usize) -> bool {
        self.o.flags[bi] & self.skip != 0 || (!self.v.open.is_empty() && self.v.open.contains(&(bi as u32)))
    }
    /// whether a box no cell's list holds at its edge is out of its way
    #[inline(always)]
    fn clear(&self, b: &[f64]) -> bool {
        let (a, r) = (&self.a, &self.r);
        seg_clear(a[0], a[1], a[2], r[0], r[1], r[2], self.len, b, self.pad)
    }
    #[inline(always)]
    fn line_box(&self, b: &[f64]) -> Option<(f64, f64, f64)> {
        let (a, r) = (&self.a, &self.r);
        line_box(self.o, a[0], a[1], a[2], r[0], r[1], r[2], b)
    }
    /// What a box the line lies in over [t0, t1], and starts `depth` inside of, says of it: −1 = in the way, or a pane's own answer; None = it goes on.
    #[inline(always)]
    fn meets(&self, s: &mut Asking, bi: usize, (t0, t1, depth): (f64, f64, f64)) -> Option<i32> {
        if depth > self.shrink || !(t1 > self.shrink && t0 < self.len - self.shrink) {
            return None;
        }
        if self.o.flags[bi] & F_GLASS == 0 {
            return Some(-1);
        }
        let (a, r) = (&self.a, &self.r);
        match pane_in_way(self.o, s, self.rule, self.panes, bi, t0, a[0], a[1], a[2], r[0], r[1], r[2], self.tune) {
            0 => None,
            said => Some(said),
        }
    }
    /// WHAT STOPPED A LINE LIKE IT LAST, ASKED FIRST: most lines asked are asked again the tick after and are stopped by the same wall. True only if the walk would be
    /// stopped by that box too: a leaf by the walk's own test of it; a plain static if it is in the way and filed in a cell under the line at a point inside it, so that
    /// the walk comes on it. A pane, a tilted box and a line that leaves the level are the walk's.
    #[inline(always)]
    fn stopped_by(&self, guess: u32, inside: bool) -> bool {
        let (o, a, r) = (self.o, &self.a, &self.r);
        if guess & LAST_LEAF != 0 {
            let i = (guess ^ LAST_LEAF) as usize;
            return self.v.leaves
                && i < o.leaves()
                && !self.v.but.contains(&(i as u32))
                && loose_hit(&o.cut, &o.leaf[i * REC..i * REC + REC], a[0], a[1], a[2], r[0], r[1], r[2], self.len, self.uu, self.su, self.lu);
        }
        let bi = guess as usize - 1;
        if !inside || bi >= o.boxes() || o.flags[bi] & F_GLASS != 0 || self.passes(bi) {
            return false;
        }
        let b = o.row(bi);
        if (b[R_KIND] as u32) & K_ROT != 0 {
            return false;
        }
        let Some((t0, t1, depth)) = self.span(b) else {
            return false;
        };
        if depth > self.shrink || !(t1 > self.shrink && t0 < self.len - self.shrink) {
            return false;
        }
        let t = js_min(js_max(t0, 0.0), self.len);
        let c = floor((a[2] + r[2] * t) / o.cs) as usize * o.w as usize + floor((a[0] + r[0] * t) / o.cs) as usize;
        c + 1 < o.start.len() && o.items[o.start[c] as usize..o.start[c + 1] as usize].contains(&(bi as u32))
    }
    /// The tilted statics, whose cells a plan's columns do not tell: each asked where a half-metre step of the walk would have met its cells.
    fn tilted(&self, s: &mut Asking) -> Option<i32> {
        let o = self.o;
        for tl in o.tilted.chunks_exact(5) {
            let bi = tl[0] as usize;
            s.stamp[bi] = self.id;
            if self.passes(bi) {
                continue;
            }
            let b = &o.rec[bi * REC..bi * REC + REC];
            if self.clear(b) || !self.steps_in((tl[1] - 1) as f64, (tl[2] + 1) as f64, (tl[3] - 1) as f64, (tl[4] + 1) as f64) {
                continue;
            }
            let Some(span) = self.line_box(b) else {
                continue;
            };
            if let Some(said) = self.meets(s, bi, span) {
                return Some(said);
            }
        }
        None
    }
    /// whether a step of the walk falls in these cells
    fn steps_in(&self, x0: f64, x1: f64, z0: f64, z1: f64) -> bool {
        let mut i = 0.0;
        while i <= self.steps {
            let (x, z) = self.cell_at(i);
            if x >= x0 && x <= x1 && z >= z0 && z <= z1 {
                return true;
            }
            i += 1.0;
        }
        false
    }
    /// the cell step `i` of the walk falls in
    #[inline(always)]
    fn cell_at(&self, i: f64) -> (f64, f64) {
        let t = (i / self.steps) * self.len;
        (floor((self.a[0] + self.r[0] * t) / self.o.cs), floor((self.a[2] + self.r[2] * t) / self.o.cs))
    }
    /// The cells under it, listed by the columns (or the rows) it crosses, the near ones first. → how many
    #[inline(always)]
    fn cells_along(&self, s: &mut Asking) -> usize {
        let (o, a, b, d) = (self.o, &self.a, &self.b, &self.d);
        let (cs, gw, gh, wi) = (o.cs, o.w as f64, o.h as f64, o.w as usize);
        let mut n = 0usize;
        let q = self.pad + 1e-3;
        let x_major = neg_up(d[0]) >= neg_up(d[2]);
        let (am, bm, an, slope) = if x_major { (a[0], b[0], a[2], d[2] / d[0]) } else { (a[2], b[2], a[0], d[0] / d[2]) };
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
                s.seg_cells[n] = (if x_major { ri * wi + ci } else { ci * wi + ri }) as u32;
                n += 1;
                r += 1.0;
            }
            j += 1.0;
        }
        n
    }
    /// The cells round each half-metre step of it (`edge`: of each that is not well inside the level), each once, in the order the steps come on them, after the `n`
    /// that are listed. → how many
    fn cells_round(&self, s: &mut Asking, mut n: usize, edge: bool) -> usize {
        let (gw, gh, wi) = (self.o.w as f64, self.o.h as f64, self.o.w as usize);
        let (mut lx, mut lz) = (f64::NAN, f64::NAN);
        let mut i = 0.0;
        while i <= self.steps {
            let t = (i / self.steps) * self.len;
            if edge && self.inside(&[self.a[0] + self.r[0] * t, 0.0, self.a[2] + self.r[2] * t]) {
                i += 1.0;
                continue;
            }
            let (x, z) = self.cell_at(i);
            i += 1.0;
            if x == lx && z == lz {
                continue;
            }
            // (those round the sample before are listed)
            let (px, pz) = (lx, lz);
            lx = x;
            lz = z;
            let mut dz = -1.0;
            while dz <= 1.0 {
                let mut dx = -1.0;
                while dx <= 1.0 {
                    let (cx, cz) = (x + dx, z + dz);
                    dx += 1.0;
                    if cx < 0.0 || cz < 0.0 || cx >= gw || cz >= gh || ((cx - px).abs() <= 1.0 && (cz - pz).abs() <= 1.0) {
                        continue;
                    }
                    let ci = cz as usize * wi + cx as usize;
                    if s.cell_stamp[ci] == self.id {
                        continue;
                    }
                    s.cell_stamp[ci] = self.id;
                    s.seg_cells[n] = ci as u32;
                    n += 1;
                }
                dz += 1.0;
            }
        }
        n
    }
    /// Over what of the line it lies in a box, and how deep the line starts in it (line_box's answer; a plain box is first refused by what it can be
    /// refused by: its height, how far off it stands in plan, and before or past the ends). None = it misses.
    #[inline(always)]
    fn span(&self, b: &[f64]) -> Option<(f64, f64, f64)> {
        if (b[R_KIND] as u32) & K_SHAPED != 0 {
            return if self.clear(b) { None } else { self.line_box(b) };
        }
        let (a, r, pad, e1) = (&self.a, &self.r, self.pad, self.e1);
        let (h0, h1, h2) = (b[R_H], b[R_H + 1], b[R_H + 2]);
        let hy = neg_up(h1);
        let m1 = b[R_C + 1] - a[1];
        if if e1 > 0.0 { m1 - hy > e1 + pad || m1 + hy < -pad } else { m1 - hy > pad || m1 + hy < e1 - pad } {
            return None;
        }
        let rr = neg_up(h0) + neg_up(h2) + pad;
        let (m0, m2) = (b[R_C] - a[0], b[R_C + 2] - a[2]);
        let cr = m0 * r[2] - m2 * r[0];
        if cr * cr > rr * rr * self.uu {
            return None;
        }
        let (sa, ea) = (m0 * r[0] + m2 * r[2], rr * self.su);
        if sa < -ea || sa > self.lu + ea {
            return None;
        }
        // line_box over scalars: its statements in its order
        let (co, sn) = (b[R_COS], b[R_SIN]);
        let (qx, qz) = (a[0] - b[R_C], a[2] - b[R_C + 2]);
        let (ox, oy, oz) = (co * qx - sn * qz, a[1] - b[R_C + 1], sn * qx + co * qz);
        let (ex, ez) = (co * r[0] - sn * r[2], sn * r[0] + co * r[2]);
        let (mut t0, mut t1) = (-INF, INF);
        if !(slab_span(ox, ex, h0, &mut t0, &mut t1) && slab_span(oy, r[1], h1, &mut t0, &mut t1) && slab_span(oz, ez, h2, &mut t0, &mut t1)) {
            return None;
        }
        Some((t0, t1, js_min(js_min(h0 - ox.abs(), h1 - oy.abs()), h2 - oz.abs())))
    }
}
/// a line's span cut down by the faces at ±h along one axis of a box: from `o` at `d` along it. false = nothing left
#[inline(always)]
fn slab_span(o: f64, d: f64, h: f64, t0: &mut f64, t1: &mut f64) -> bool {
    if d.abs() < 1e-12 {
        return !(o.abs() > h);
    }
    let (mut ta, mut tb) = ((-h - o) / d, (h - o) / d);
    if ta > tb {
        core::mem::swap(&mut ta, &mut tb);
    }
    if ta > *t0 {
        *t0 = ta;
    }
    if tb < *t1 {
        *t1 = tb;
    }
    !(*t0 > *t1)
}

// ---- a line asked from inside the module (the senses: sight.rs, light.rs, hear.rs, nav.rs), from its two ends
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
/// IS A MAN HIDDEN FROM AN EYE: the chest first, and the head only behind a blocked chest
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn man_hidden(w: &World, leaves: bool, through: u32, e0: f64, e1: f64, e2: f64, c0: f64, c1: f64, c2: f64, h0: f64, h1: f64, h2: f64, tune: &Knobs) -> Option<bool> {
    match sight_line(w, leaves, through, e0, e1, e2, c0, c1, c2, tune) {
        Some(true) => sight_line(w, leaves, through, e0, e1, e2, h0, h1, h2, tune),
        said => said,
    }
}
/// a line that PASSES glass (`through` carries the glass flag, so no pane is ever the host's to judge):
/// the length is the game's own, worked out here. None = not a line this module walks (a length that is no length of this world's).
#[inline]
pub fn sight_line(w: &World, leaves: bool, through: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, tune: &Knobs) -> Option<bool> {
    let len = match walked(a0, a1, a2, b0, b1, b2) {
        Ok(len) => len,
        Err(said) => return said,
    };
    Some(blocked(w, a0, a1, a2, b0, b1, b2, len, 0.02, through, leaves, 0, tune) == -1)
}
/// the same line with nothing passed. `ask` = ASK_GLASS where the game has a rule for an open pane. None = a line too
/// long to be a line of this world.
#[inline]
pub fn solid_line(w: &World, leaves: bool, ask: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, tune: &Knobs) -> Option<bool> {
    solid_over(w, &Over::with(leaves), ask, a0, a1, a2, b0, b1, b2, tune)
}
/// …of the world as the question takes it
#[allow(clippy::too_many_arguments)]
pub fn solid_over(w: &World, v: &Over, ask: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, tune: &Knobs) -> Option<bool> {
    let len = match walked(a0, a1, a2, b0, b1, b2) {
        Ok(len) => len,
        Err(said) => return said,
    };
    match blocked_over(w, v, a0, a1, a2, b0, b1, b2, len, 0.02, 0, ask, tune) {
        -1 => Some(true),
        0 => Some(false),
        _ => None,
    }
}
/// `leaves_blocked` from its two ends
#[inline]
pub fn leaf_line(w: &World, need: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Option<bool> {
    let len = match walked(a0, a1, a2, b0, b1, b2) {
        Ok(len) => len,
        Err(said) => return said,
    };
    Some(leaves_blocked(w, a0, a1, a2, b0, b1, b2, len, need))
}

// ---- THE RAY: the nearest thing along it, how far, and the face it is entered by

/// one slab of a ray against a box with the face it enters by: the interval cut down by the faces at ±h along axis `k`; the entry's axis and sign kept. false = nothing left
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
/// a face normal of the box's own frame, in the world's: by its rotation, or turned by its yaw (the cos and sin its row keeps beside it)
#[inline(always)]
fn out_of(b: &[f64], l0: f64, l1: f64, l2: f64) -> (f64, f64, f64) {
    if (b[R_KIND] as u32) & K_ROT != 0 {
        rotate(b, l0, l1, l2)
    } else {
        let (cs, sn) = (b[R_COS], b[R_SIN]);
        (cs * l0 + sn * l2, l1, -sn * l0 + cs * l2)
    }
}
/// the unit vector of axis `k`, times `sgn` (the other two plain zeros, never −0)
#[inline(always)]
fn face_of(axis: i32, sgn: f64) -> (f64, f64, f64) {
    (if axis == 0 { sgn } else { 0.0 }, if axis == 1 { sgn } else { 0.0 }, if axis == 2 { sgn } else { 0.0 })
}

/// a ray in a box's own frame: where it starts and its way
type Local = (f64, f64, f64, f64, f64, f64);
/// the ray cut down by a box's six faces: from where, to where, and the face it enters by (axis −1 = none, it starts inside); None = nothing left
#[inline(always)]
fn slabs_n(b: &[f64], (ox, oy, oz, dx, dy, dz): Local, tmax: f64) -> Option<(f64, f64, i32, f64)> {
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1i32, 0.0);
    let left = slab_n(b[R_H], ox, inv_or(dx), &mut t0, &mut t1, &mut axis, &mut sgn, 0)
        && slab_n(b[R_H + 1], oy, inv_or(dy), &mut t0, &mut t1, &mut axis, &mut sgn, 1)
        && slab_n(b[R_H + 2], oz, inv_or(dz), &mut t0, &mut t1, &mut axis, &mut sgn, 2);
    if left { Some((t0, t1, axis, sgn)) } else { None }
}
/// a round box: where, and the normal in the box's frame; face 0 = none entered (the origin is inside), 1 = an end, 2 = the side
fn round_n(b: &[f64], (ox, oy, oz, dx, dy, dz): Local, tmax: f64) -> Option<(f64, (f64, f64, f64))> {
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
    if face != 2 {
        return Some((t0, (0.0, sgn, 0.0)));
    }
    let (hx, hz) = (ox + dx * t0, oz + dz * t0);
    let len = sqrt(hx * hx + hz * hz);
    let il = 1.0 / (if len != 0.0 && len == len { len } else { 1.0 });
    Some((t0, (hx * il, 0.0, hz * il)))
}
/// a cut box, the same
fn cut_n(cut: &[f64], b: &[f64], l: Local, tmax: f64) -> Option<(f64, (f64, f64, f64))> {
    let (t0, t1, axis, sgn) = slabs_n(b, l, tmax)?;
    let (planes, n) = planes_of(cut, b);
    let (c0, _, k) = cut_clip(planes, n, l.0, l.1, l.2, l.3, l.4, l.5, t0, t1)?;
    if k >= 0 {
        let p = &planes[k as usize * 4..k as usize * 4 + 3];
        return Some((c0, (p[0], p[1], p[2])));
    }
    if axis < 0 { None } else { Some((c0, face_of(axis, sgn))) }
}
/// A RAY AGAINST A BOX (a round box, a cut one, then any other, rotated or yawed): where the ray enters the box within tmax and the normal of the
/// face it enters by. None = it does not, or it starts inside.
#[allow(clippy::too_many_arguments)]
pub(crate) fn ray_box_n(cut: &[f64], a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: &[f64], tmax: f64) -> Option<(f64, f64, f64, f64)> {
    let kind = b[R_KIND] as u32;
    let local = to_local(a0, a1, a2, r0, r1, r2, b);
    let (t, (l0, l1, l2)) = if kind & K_ROUND != 0 {
        round_n(b, local, tmax)?
    } else if kind & K_CUT != 0 {
        cut_n(cut, b, local, tmax)?
    } else {
        let (t0, _, axis, sgn) = slabs_n(b, local, tmax)?;
        if axis < 0 {
            return None;
        }
        (t0, face_of(axis, sgn))
    };
    let (n0, n1, n2) = out_of(b, l0, l1, l2);
    Some((t, n0, n1, n2))
}

/// the ray's answer when nothing is met, and when the host must answer it itself (a ray that is not a number)
pub const RAY_NONE: i32 = -1;
pub const RAY_HOST: i32 = -2;
/// what a leaf's index is offset by in the answer (index ≥ 1e6 = a leaf)
pub const RAY_LEAF: i32 = 1_000_000;
/// …and a box the question added
pub const RAY_EXTRA: i32 = 2_000_000;

/// The nearest thing the ray from a along the unit r meets within tmax — the leaves first (with `leaves`), then the statics of
/// the cells round the ray, walked in half-metre steps UNTIL THE RAY HAS LEFT THE LEVEL FOR GOOD (so no length is too long: a ray of a thousand
/// kilometres is walked as far as the level reaches and no farther; one straight up or down is one sample). → RAY_NONE · RAY_HOST (a ray that is no number: there is no answer to give) · the
/// static's index, or RAY_LEAF + the leaf's; and into `out` how far along, then the face's normal. `ask` = the game has a rule for an open pane: a pane
/// met where it is open is passed, and the ray flies on to what stands beyond.
#[allow(clippy::too_many_arguments)]
pub fn ray(w: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, leaves: bool, ask: bool, out: &mut [f64], tune: &Knobs) -> i32 {
    ray_over(w, &Over::with(leaves), a0, a1, a2, r0, r1, r2, tmax, ask, out, tune)
}
/// …with ONE LEAF OF THE LIST LEFT OUT, `but` (−1: none): the ray of a thing that is itself one of the leaves (a slab off its hinges is no wall to
/// itself). The leaves keep their own indices in the answer.
#[allow(clippy::too_many_arguments)]
pub fn ray_but(w: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, but: i32, ask: bool, out: &mut [f64], tune: &Knobs) -> i32 {
    let one = [but as u32];
    ray_over(w, &Over { but: if but >= 0 { &one } else { &[] }, ..Over::with(true) }, a0, a1, a2, r0, r1, r2, tmax, ask, out, tune)
}
/// …of the world as the question takes it. A box it adds is RAY_EXTRA + its place.
#[allow(clippy::too_many_arguments)]
pub fn ray_over(w: &World, v: &Over, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, ask: bool, out: &mut [f64], tune: &Knobs) -> i32 {
    let (mut best, mut bi, mut bn) = (tmax, -1i32, (0.0, 1.0, 0.0));
    let o = w;
    if v.leaves {
        for (i, row) in o.leaf.chunks_exact(REC).enumerate() {
            if !v.but.is_empty() && v.but.contains(&(i as u32)) {
                continue;
            }
            if let Some((t, n0, n1, n2)) = ray_box_n(&o.cut, a0, a1, a2, r0, r1, r2, row, best) {
                if t < best {
                    best = t;
                    bi = RAY_LEAF + i as i32;
                    bn = (n0, n1, n2);
                }
            }
        }
    }
    for (i, b) in v.extra.chunks_exact(REC).enumerate() {
        if let Some((t, n0, n1, n2)) = ray_box_n(v.extra_cut, a0, a1, a2, r0, r1, r2, b, best) {
            if t < best {
                best = t;
                bi = RAY_EXTRA + i as i32;
                bn = (n0, n1, n2);
            }
        }
    }
    let ray = Ray { o, v, tune, a: [a0, a1, a2], r: [r0, r1, r2], tmax, ask };
    let from = (best, bi, bn);
    let met = match ray.crossed(from) {
        Some(met) => Ok(met),
        None => ray.walked(from),
    };
    match met {
        Ok(met) => (best, bi, bn) = met,
        Err(said) => return said,
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

/// the best a ray has met so far: how far, what, its face's normal
type Met = (f64, i32, (f64, f64, f64));
/// A RAY AMONG THE STATICS.
struct Ray<'a> {
    o: &'a World,
    v: &'a Over<'a>,
    tune: &'a Knobs,
    a: [f64; 3],
    r: [f64; 3],
    tmax: f64,
    ask: bool,
}
impl Ray<'_> {
    /// static `b` asked, once: what is best now. `first`: of two met at one place, whether this one is the walk's
    #[inline(always)]
    fn asked(&self, b: usize, met: &mut Met, first: impl Fn(usize, usize) -> bool) {
        let (o, a, r) = (self.o, &self.a, &self.r);
        let fl = o.flags[b];
        if fl & F_NO_SHADOW != 0 || (!self.v.open.is_empty() && self.v.open.contains(&(b as u32))) {
            return;
        }
        if let Some((t, n0, n1, n2)) = ray_box_n(&o.cut, a[0], a[1], a[2], r[0], r[1], r[2], &o.rec[b * REC..b * REC + REC], met.0) {
            if t < met.0 || (t == met.0 && met.1 >= 0 && met.1 < RAY_LEAF && first(b, met.1 as usize)) {
                if self.ask && fl & F_GLASS != 0 && glass::open(o, b, a[0] + r[0] * t, a[1] + r[1] * t, a[2] + r[2] * t, self.tune) {
                    return;
                }
                *met = (t, b as i32, (n0, n1, n2));
            }
        }
    }
    /// how many half-metre samples the walk takes, and where each stands in plan
    #[inline(always)]
    fn steps(&self) -> f64 {
        js_ceil(self.tmax / 0.5) + 1.0
    }
    #[inline(always)]
    fn sample(&self, i: f64, steps: f64) -> (f64, f64, f64) {
        let t = (i / steps) * self.tmax;
        (t, floor((self.a[0] + self.r[0] * t) / self.o.cs), floor((self.a[2] + self.r[2] * t) / self.o.cs))
    }
    /// The statics of the cells it crosses, from its own on, and no others. None: what the walk would meet is the walk's to say (it starts outside the level, or passes a
    /// static that stands over the level's edge, or two that are tilted).
    fn crossed(&self, from: Met) -> Option<Met> {
        let (o, a, r) = (self.o, &self.a, &self.r);
        let (cs, gw, gh, wi) = (o.cs, o.w as f64, o.h as f64, o.w as usize);
        let (mut x, mut z) = (floor(a[0] / cs), floor(a[2] / cs));
        if !(x >= 0.0 && z >= 0.0 && x < gw && z < gh) || !(r[0] - r[0] == 0.0 && r[2] - r[2] == 0.0 && self.tmax - self.tmax == 0.0) {
            return None;
        }
        let q = asking();
        let id = q.ask();
        let mut met = from;
        let first = |b: usize, than: usize| self.before(b, than);
        let mut visit = |x: f64, z: f64, met: &mut Met| {
            let c = z as usize * wi + x as usize;
            // (as low and as high as it goes before the best so far)
            let end = a[1] + r[1] * met.0;
            let (foot, top) = if r[1] < 0.0 { (end - SEG_PAD, a[1] + SEG_PAD) } else { (a[1] - SEG_PAD, end + SEG_PAD) };
            for k in o.start[c] as usize..o.start[c + 1] as usize {
                let b = o.items[k] as usize;
                if o.tall[b][0] > top || o.tall[b][1] < foot || q.stamp[b] == id {
                    continue;
                }
                q.stamp[b] = id;
                self.asked(b, met, first);
            }
        };
        let next = |at: f64, c: f64, way: f64| if way > 0.0 { ((c + 1.0) * cs - at) / way } else if way < 0.0 { (c * cs - at) / way } else { f64::INFINITY };
        let (mut tx, mut tz) = (next(a[0], x, r[0]), next(a[2], z, r[2]));
        let (dx, dz) = (if r[0] != 0.0 { cs / r[0].abs() } else { f64::INFINITY }, if r[2] != 0.0 { cs / r[2].abs() } else { f64::INFINITY });
        let (sx, sz) = (if r[0] > 0.0 { 1.0 } else { -1.0 }, if r[2] > 0.0 { 1.0 } else { -1.0 });
        let on = |x: f64, z: f64| x >= 0.0 && z >= 0.0 && x < gw && z < gh;
        visit(x, z, &mut met);
        loop {
            let t = if tx < tz { tx } else { tz };
            if t > met.0 + 0.01 {
                break;
            }
            // (through a corner, or as good as: the cells at both sides of it)
            if (tx - tz).abs() < 1e-6 {
                if on(x + sx, z) {
                    visit(x + sx, z, &mut met);
                }
                if on(x, z + sz) {
                    visit(x, z + sz, &mut met);
                }
            }
            if tx < tz {
                x += sx;
                tx += dx;
            } else {
                z += sz;
                tz += dz;
            }
            if !on(x, z) {
                if o.outer.iter().any(|&b| self.passes(b as usize, met.0)) {
                    return None;
                }
                break;
            }
            visit(x, z, &mut met);
        }
        // a tilted static reaches past the cells it is filed in: it is met where the walk's samples come by those
        let mut tilted = None;
        for tl in o.tilted.chunks_exact(5) {
            let b = tl[0] as usize;
            if q.stamp[b] == id || !self.passes(b, met.0) {
                continue;
            }
            if tilted.is_some() {
                return None;
            }
            tilted = Some(tl);
        }
        if let Some(tl) = tilted {
            let steps = self.steps();
            let mut i = 0.0;
            while i <= steps {
                let (t, x, z) = self.sample(i, steps);
                if t > met.0 + 1.5 {
                    break;
                }
                if x >= (tl[1] - 1) as f64 && x <= (tl[2] + 1) as f64 && z >= (tl[3] - 1) as f64 && z <= (tl[4] + 1) as f64 {
                    self.asked(tl[0] as usize, &mut met, first);
                    break;
                }
                i += 1.0;
            }
        }
        Some(met)
    }
    /// whether it comes within the ball round a static before `far`
    fn passes(&self, b: usize, far: f64) -> bool {
        let b = &self.o.rec[b * REC..b * REC + REC];
        let (m0, m1, m2) = (b[R_C] - self.a[0], b[R_C + 1] - self.a[1], b[R_C + 2] - self.a[2]);
        let rr = neg_up(b[R_H]) + neg_up(b[R_H + 1]) + neg_up(b[R_H + 2]) + SEG_PAD;
        let s = m0 * self.r[0] + m1 * self.r[1] + m2 * self.r[2];
        s >= -rr && s <= far + rr && m0 * m0 + m1 * m1 + m2 * m2 - s * s <= rr * rr
    }
    /// which of two statics the walk comes on first
    #[cold]
    fn before(&self, b: usize, than: usize) -> bool {
        let o = self.o;
        let (steps, gw, gh) = (self.steps(), o.w as f64, o.h as f64);
        let mut i = 0.0;
        while i <= steps {
            let (_, x, z) = self.sample(i, steps);
            for dz in [-1.0, 0.0, 1.0] {
                for dx in [-1.0, 0.0, 1.0] {
                    let (cx, cz) = (x + dx, z + dz);
                    if !(cx >= 0.0 && cz >= 0.0 && cx < gw && cz < gh) {
                        continue;
                    }
                    let c = cz as usize * o.w as usize + cx as usize;
                    for &k in &o.items[o.start[c] as usize..o.start[c + 1] as usize] {
                        if k as usize == b || k as usize == than {
                            return k as usize == b;
                        }
                    }
                }
            }
            i += 1.0;
        }
        false
    }
    /// The walk: the statics of the cells round each half-metre sample.
    fn walked(&self, from: Met) -> Result<Met, i32> {
        let (o, tmax) = (self.o, self.tmax);
        let ([a0, a1, a2], [r0, r1, r2]) = (self.a, self.r);
        let mut met = from;
        let q = asking();
        let id = q.ask();
        let steps = js_ceil(tmax / 0.5) + 1.0;
        let (cs, gw, gh) = (o.cs, o.w as f64, o.h as f64);
        let wi = o.w as usize;
        let mut i = 0.0;
        // (the sample before this one: the cells round it have been asked)
        let (mut lx, mut lz) = (f64::NAN, f64::NAN);
        while i <= steps {
            let t = (i / steps) * tmax;
            if t > met.0 + 1.5 {
                break;
            }
            let (x, z) = (floor((a0 + r0 * t) / cs), floor((a2 + r2 * t) / cs));
            if x != x || z != z {
                return Err(RAY_HOST); // (a ray that is not a number — or of no finite length, whose samples are none: the caller's mistake, which the host says by name)
            }
            // (the cells round this sample lie off the grid on the side the ray is going, or it does not go that way at all: so does every later sample's,
            //  a sample's cell moving one way only along a ray. Nothing more can be met. Or it goes nowhere in plan: every later sample's cells are the
            //  first's, whose statics have each been asked the whole ray.)
            if (r0 >= 0.0 && x - 1.0 >= gw) || (r0 <= 0.0 && x + 1.0 < 0.0) || (r2 >= 0.0 && z - 1.0 >= gh) || (r2 <= 0.0 && z + 1.0 < 0.0) || (i > 0.0 && r0 == 0.0 && r2 == 0.0) {
                break;
            }
            i += 1.0;
            if x == lx && z == lz {
                continue;
            }
            let mut dz = -1.0;
            while dz <= 1.0 {
                let mut dx = -1.0;
                while dx <= 1.0 {
                    let (cx, cz) = (x + dx, z + dz);
                    dx += 1.0;
                    if !(cx >= 0.0 && cz >= 0.0 && cx < gw && cz < gh) || ((cx - lx).abs() <= 1.0 && (cz - lz).abs() <= 1.0) {
                        continue;
                    }
                    let c = cz as usize * wi + cx as usize;
                    let (k0, k1) = (o.start[c] as usize, o.start[c + 1] as usize);
                    // (as low and as high as the ray goes before the met.0 so far)
                    let end = a1 + r1 * met.0;
                    let (foot, top) = if r1 < 0.0 { (end - SEG_PAD, a1 + SEG_PAD) } else { (a1 - SEG_PAD, end + SEG_PAD) };
                    for k in k0..k1 {
                        let b = o.items[k] as usize;
                        if o.tall[b][0] > top || o.tall[b][1] < foot || q.stamp[b] == id {
                            continue;
                        }
                        q.stamp[b] = id;
                        self.asked(b, &mut met, |_, _| false);
                    }
                }
                dz += 1.0;
            }
            lx = x;
            lz = z;
        }
        Ok(met)
    }
}

/// A RAY ASKED BY A RULE (a slab off its hinges, a thrown thing). A RAY OF NO LENGTH
/// MEETS NOTHING; one that is no number has no answer (RAY_HOST: the rule writes nothing and says
/// so, and the host names the mistake). `but` and `ask` as ray_but takes them. → RAY_NONE · RAY_HOST · the static's index, or RAY_LEAF + the
/// leaf's; `out` as ray writes it.
#[allow(clippy::too_many_arguments)]
pub fn ray_rule(w: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, but: i32, ask: bool, out: &mut [f64], tune: &Knobs) -> i32 {
    if !(tmax > 0.0) {
        return RAY_NONE;
    }
    ray_but(w, a0, a1, a2, r0, r1, r2, tmax, but, ask, out, tune)
}

/// where ray_things says which part it met, after how far along and the normal
pub const RAY_PART: usize = 4;
/// The nearest PART of any thing of furniture the ray from a along the unit r meets within tmax — the things in the
/// furniture's own order (the world's `things`), each passed by if the upright cylinder that holds it is clear of the ray in plan or lies behind it
/// or beyond the best so far, its parts in their order. → −1, or which thing of the level's list; and into `out` how far
/// along, the face's normal, and which part of the list.
#[allow(clippy::too_many_arguments)]
pub fn ray_things(w: &World, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64, out: &mut [f64]) -> i32 {
    let (mut best, mut bp, mut bn, mut bb) = (tmax, -1i32, (0.0, 1.0, 0.0), 0usize);
    let fl = crate::maths::hypot2(r0, r2);
    for (r, t) in w.things.iter().enumerate() {
        let (id, start, count) = (t.id as usize, t.at as usize, t.parts as usize);
        let radius = w.thing[id * TH + TH_RADIUS];
        let (ox, oz) = (w.thing[id * TH + TH_X] - a0, w.thing[id * TH + TH_Z] - a2);
        let along = if fl > 1e-6 { (ox * r0 + oz * r2) / fl } else { 0.0 };
        if fl > 1e-6 && (along < -radius || along > best * fl + radius) {
            continue;
        }
        let perp2 = ox * ox + oz * oz - along * along;
        if perp2 > (radius + 0.05) * (radius + 0.05) {
            continue;
        }
        for i in start..start + count {
            if let Some((t, n0, n1, n2)) = ray_box_n(&w.cut, a0, a1, a2, r0, r1, r2, &w.part[i * REC..i * REC + REC], best) {
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
