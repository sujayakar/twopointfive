//! WHAT A PANE IS TO A ROUND: what a sheet takes from a round crossing it, how the round turns, where it
//! leaves, the flight over panes, and the through-holes rounds have left. One rule for a round and for a line-of-fire question: a man fires through a pane exactly
//! when his round would get through it. Statement for statement with the rule it replaced. What a round does next and what it files is the caller's.
//!
//! WHAT IS STATE. The holes (panes.rs `Hole`): a part of the panes' run in the game's block, a row a pane a round has been through.
use game_types::knobs::Knobs;
use crate::collide;
use crate::roles::*;
use crate::lines::{self, line_box, RAY_LEAF};
use crate::maths::{acos, cos, hypot3, sin, tan};
use crate::panes::{self, norm3, point_seed, type_of_pane, box_seed, Ctx, ANNEALED, LAMINATED, NO_TYPE, TEMPERED};
use crate::world::{asking, clamp, js_abs, js_max, js_min, world, F_GLASS, F_NO_SHADOW, REC, R_COS, R_H, R_SIN};
use prim::sqrt;

const PI: f64 = core::f64::consts::PI;
const THICK_GLASS_HALF: f64 = 0.05;
/// the next leg is born this far past the face the round left the glass by
const EXIT_EPS: f64 = 1e-3;
/// the most panes one flight is followed through (GLASSPEN.maxPanes is under this)
pub const MAX_CROSS: usize = 16;
pub const CROSS: usize = 25;

#[derive(Clone, Copy)]
pub struct Toll {
    pub ty: u32,
    pub n: [f64; 3],
    pub cos: f64,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}
#[derive(Clone, Copy)]
pub struct Crossing {
    pub t: Toll,
    pub bi: u32,
    pub at_t: f64,
    pub at: [f64; 3],
    pub exit: [f64; 3],
    pub dir_in: [f64; 3],
    pub dir_out: [f64; 3],
    pub e_in: f64,
    pub turn_deg: f64,
    pub thru: f64,
    pub into: i32,
}
#[inline]
fn sheet_box(c: &Ctx, bi: usize) -> Option<&'static [f64]> {
    let o = world(c.w);
    if bi >= o.boxes as usize || o.flags[bi] & F_GLASS == 0 {
        return None;
    }
    let b: &'static [f64] = &o.rec[bi * REC..bi * REC + REC];
    if js_min(b[R_H], b[R_H + 2]) <= THICK_GLASS_HALF {
        Some(b)
    } else {
        None
    }
}
fn near_hole(g: usize, bi: u32, p: [f64; 3], tune: &Knobs) -> bool {
    let ps = crate::panes::pn(g);
    let r = tune.glasspen.hole_r;
    let r2 = r * r;
    for h in 0..ps.nh as usize {
        let row = ps.hole(h);
        if row.bi == bi {
            for i in 0..row.n as usize {
                let (dx, dy, dz) = (row.p[3 * i] - p[0], row.p[3 * i + 1] - p[1], row.p[3 * i + 2] - p[2]);
                if dx * dx + dy * dy + dz * dz < r2 {
                    return true;
                }
            }
            return false;
        }
    }
    false
}
/// What sheet `bi` takes from a round of energy `e` meeting it at `at` flying `d` (unit), the `nth` pane of its flight; None: the round is spent in it
pub fn pane_toll(c: &Ctx, bi: u32, at: [f64; 3], d: [f64; 3], e: f64, nth: usize, tune: &Knobs) -> Option<Toll> {
    let b = sheet_box(c, bi as usize)?;
    if nth as f64 >= js_min(MAX_CROSS as f64, tune.glasspen.max_panes) {
        return None;
    }
    let (cs, sn) = (b[R_COS], b[R_SIN]);
    let mut n = if b[R_H] < b[R_H + 2] { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
    let mut co = n[0] * d[0] + n[1] * d[1] + n[2] * d[2];
    if co < 0.0 {
        n = [n[0] * -1.0, n[1] * -1.0, n[2] * -1.0];
        co = -co;
    }
    if co < tune.glasspen.graze_cos {
        return None;
    }
    let hole = near_hole(c.g, bi, at, tune);
    let ty = match type_of_pane(c, bi, tune) {
        NO_TYPE => ANNEALED,
        t => t,
    };
    let cost = match ty {
        TEMPERED => *tune.glasspen.cost.get(game_types::glasspen::PaneType::Tempered).unwrap(),
        LAMINATED => *tune.glasspen.cost.get(game_types::glasspen::PaneType::Laminated).unwrap(),
        _ => *tune.glasspen.cost.get(game_types::glasspen::PaneType::Annealed).unwrap(),
    };
    let take = cost * (2.0 * js_min(b[R_H], b[R_H + 2]) / tune.glasspen.ref_thick) * js_min(tune.glasspen.oblique_max, 1.0 / co) * (if hole { tune.glasspen.hole_k } else { 1.0 });
    let e_out = e - take;
    if e_out < tune.glasspen.min_energy {
        return None;
    }
    Some(Toll { ty, n, cos: co, take, e_out, hole })
}
/// What a round leaving the glass at `p` along `d` is born inside: −1 open air · −2 a static a round stops in · else the index of another sheet of glass
fn born_in(c: &Ctx, p: [f64; 3], d: [f64; 3], me: u32, tune: &Knobs) -> i32 {
    let n = collide::gather(c.w, p[0] - 0.02, p[2] - 0.02, p[0] + 0.02, p[2] + 0.02, p[1] - 0.02, p[1] + 0.02, 64);
    let mut sheet = -1;
    for i in 0..n {
        let o = world(c.w);
        let bi = asking(c.w).list[i];
        if bi == me {
            continue;
        }
        let fl = o.flags[bi as usize];
        if fl & F_NO_SHADOW != 0 {
            continue;
        }
        let b = &o.rec[bi as usize * REC..bi as usize * REC + REC];
        let s = match line_box(o, p[0], p[1], p[2], d[0], d[1], d[2], b) {
            Some(s) => s,
            None => continue,
        };
        if s.2 < 0.0 || s.1 <= 0.0 {
            continue;
        }
        if fl & F_GLASS != 0 && js_min(b[R_H], b[R_H + 2]) <= THICK_GLASS_HALF {
            if !crate::glass::open(o, bi as usize, p[0], p[1], p[2], tune) {
                sheet = bi as i32;
            }
        } else {
            return -2;
        }
    }
    sheet
}
/// The turn, toward the normal by the slant's share, plus the hashed scatter
fn deflect(b: &[f64], at: [f64; 3], d: [f64; 3], t: &Toll, e_in: f64, tune: &Knobs) -> [f64; 3] {
    let a = tune.glasspen.deflect_deg * PI / 180.0 * clamp(t.take / js_max(e_in, 1e-6), 0.0, 1.0);
    if a < 1e-6 {
        return d;
    }
    let mut s = point_seed(box_seed(b), at[0], at[1], at[2]);
    s = (s ^ (s >> 15)).wrapping_mul(0x2C1B_3C6D).wrapping_add(0x297A_2D39);
    let u1 = (s & 0xffff) as f64 / 65535.0;
    let u2 = (s >> 16) as f64 / 65535.0;
    let up = if js_abs(d[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let e1 = norm3(cross(up, d));
    let e2 = cross(d, e1);
    let r = tan(a * sqrt(u1));
    let phi = u2 * PI * 2.0;
    let mut out = [d[0] + (e1[0] * cos(phi) + e2[0] * sin(phi)) * r, d[1] + (e1[1] * cos(phi) + e2[1] * sin(phi)) * r, d[2] + (e1[2] * cos(phi) + e2[2] * sin(phi)) * r];
    let si = sqrt(js_max(0.0, 1.0 - t.cos * t.cos));
    if si > 1e-4 {
        let kk = tan(a * tune.glasspen.bend_k * si) / si;
        let (px, py, pz) = (t.n[0] - d[0] * t.cos, t.n[1] - d[1] * t.cos, t.n[2] - d[2] * t.cos);
        out = [out[0] + px * kk, out[1] + py * kk, out[2] + pz * kk];
    }
    norm3(out)
}
#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
#[inline]
fn mad(a: [f64; 3], b: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}
/// One pane on a round's flight: what it takes and where the round leaves it; None: spent in it
pub fn pane_crossing(c: &Ctx, bi: u32, o: [f64; 3], d: [f64; 3], t: f64, e: f64, nth: usize, tune: &Knobs) -> Option<Crossing> {
    let at = mad(o, d, t);
    let to = pane_toll(c, bi, at, d, e, nth, tune)?;
    let b = sheet_box(c, bi as usize)?;
    let te = match line_box(world(c.w), at[0], at[1], at[2], d[0], d[1], d[2], b) {
        Some(s) => s.1,
        None => -1.0,
    };
    let thru = (if te > 0.0 { te } else { 2.0 * js_min(b[R_H], b[R_H + 2]) / to.cos }) + EXIT_EPS;
    let exit = mad(at, d, thru);
    let into = born_in(c, exit, d, bi, tune);
    if into == -2 {
        return None;
    }
    let dir_out = if into >= 0 { d } else { deflect(b, at, d, &to, e, tune) };
    let turn_deg = acos(clamp(d[0] * dir_out[0] + d[1] * dir_out[1] + d[2] * dir_out[2], -1.0, 1.0)) * 180.0 / PI;
    Some(Crossing { t: to, bi, at_t: t, at, exit, dir_in: d, dir_out, e_in: e, turn_deg, thru, into })
}
/// a crossing as the host reads it
pub fn write_cross(x: &Crossing, o: &mut [f64]) {
    o[0] = x.t.ty as f64;
    o[1] = x.t.n[0];
    o[2] = x.t.n[1];
    o[3] = x.t.n[2];
    o[4] = x.t.cos;
    o[5] = x.t.take;
    o[6] = x.t.e_out;
    o[7] = x.t.hole as u32 as f64;
    o[8] = x.bi as f64;
    o[9] = x.at_t;
    for i in 0..3 {
        o[10 + i] = x.at[i];
        o[13 + i] = x.exit[i];
        o[16 + i] = x.dir_in[i];
        o[19 + i] = x.dir_out[i];
    }
    o[22] = x.e_in;
    o[23] = x.turn_deg;
    o[24] = x.thru;
    // (a crossing is 25 doubles and `into` the last of the path's own list: `path_into`)
}
#[inline]
fn ray(c: &Ctx, o: [f64; 3], d: [f64; 3], tmax: f64, out: &mut [f64; 4], tune: &Knobs) -> i32 {
    lines::ray_rule(c.w, o[0], o[1], o[2], d[0], d[1], d[2], tmax, c.set, -1, true, out, tune)
}
/// The arm's line met nothing: is that because every sheet of glass across it has fallen where the line crosses it (and there is one)?
fn arm_in_hole(c: &Ctx, from: [f64; 3], rd: [f64; 3], len: f64, tune: &Knobs) -> bool {
    let mut any = false;
    for s in 0..panes::sheet_count(c) as usize {
        let bi = panes::sheet_bi(c, s) as usize;
        let o = world(c.w);
        let b = &o.rec[bi * REC..bi * REC + REC];
        let s = match line_box(o, from[0], from[1], from[2], rd[0], rd[1], rd[2], b) {
            Some(s) => s,
            None => continue,
        };
        if s.1 < 0.0 || s.0 > len {
            continue;
        }
        let p = mad(from, rd, js_max(0.0, s.0));
        if !crate::glass::open(o, bi, p[0], p[1], p[2], tune) {
            return false;
        }
        any = true;
    }
    any
}
/// glasspen.ts armThroughGlass, the walk: a man's muzzle already past the glass. → −1: something a round stops in is across the arm (the old rule stands) · else how many panes
/// took (their crossings → `out`, in order, and the energy left → `left`; none and in a hole: `energy`)
pub fn arm_walk(c: &Ctx, from: [f64; 3], muzzle: [f64; 3], dir: [f64; 3], energy: f64, out: &mut [f64], left: &mut f64, tune: &Knobs) -> i32 {
    let dd = [muzzle[0] - from[0], muzzle[1] - from[1], muzzle[2] - from[2]];
    let len = hypot3(dd[0], dd[1], dd[2]);
    if len < 1e-3 {
        return -1;
    }
    let sc = 1.0 / len;
    let rd = [dd[0] * sc, dd[1] * sc, dd[2] * sc];
    let (mut o, mut t0, mut e) = (from, 0.0, energy);
    let mut n = 0usize;
    let mut k8 = 0;
    while k8 < 8 && t0 < len {
        k8 += 1;
        let mut h = [0.0; 4];
        let m = ray(c, o, rd, len - t0, &mut h, tune);
        if m < 0 {
            break;
        }
        if m >= RAY_LEAF {
            return -1;
        }
        let at = mad(o, rd, h[0]);
        let to = match pane_toll(c, m as u32, at, dir, e, n, tune) {
            Some(t) => t,
            None => return -1,
        };
        let b = match sheet_box(c, m as usize) {
            Some(b) => b,
            None => return -1,
        };
        let te = match line_box(world(c.w), at[0], at[1], at[2], rd[0], rd[1], rd[2], b) {
            Some(s) => s.1,
            None => 0.0,
        };
        let thru = te + EXIT_EPS;
        let x = Crossing { t: to, bi: m as u32, at_t: t0 + h[0], at, exit: mad(at, rd, thru), dir_in: dir, dir_out: dir, e_in: e, turn_deg: 0.0, thru, into: -1 };
        write_cross(&x, &mut out[n * CROSS..(n + 1) * CROSS]);
        n += 1;
        e = to.e_out;
        t0 += h[0] + thru;
        o = mad(at, rd, thru);
    }
    if n == 0 {
        if arm_in_hole(c, from, rd, len, tune) {
            *left = energy;
            return 0;
        }
        return -1;
    }
    *left = e;
    n as i32
}
/// Does a round of `energy` get through every one of the panes a line of fire meets (static, metres along it to where it enters each, nearest first)
pub fn shot_passes(c: &Ctx, a: [f64; 3], rd: [f64; 3], panes_list: &[f64], energy: f64, tune: &Knobs) -> bool {
    let mut e = energy;
    for i in 0..panes_list.len() / 2 {
        match pane_toll(c, panes_list[2 * i] as u32, mad(a, rd, panes_list[2 * i + 1]), rd, e, i, tune) {
            Some(t) => e = t.e_out,
            None => return false,
        }
    }
    true
}
/// A crossing remembered on the pane, the oldest forgotten past GLASSPEN.holes
pub fn add_hole(g: usize, bi: u32, p: [f64; 3], tune: &Knobs) {
    let ps = crate::panes::pn(g);
    ps.stand();
    let row = panes_hole_row(g, bi);
    let cap = js_min(panes::HOLE_PTS as f64, tune.glasspen.holes) as usize;
    let h = ps.hole(row);
    if h.n as usize >= cap {
        h.p.copy_within(3..3 * cap, 0);
        h.n = cap as u32 - 1;
    }
    let n = h.n as usize;
    h.p[3 * n] = p[0];
    h.p[3 * n + 1] = p[1];
    h.p[3 * n + 2] = p[2];
    h.n += 1;
}
/// pane `bi`'s row of holes, made if it has none
pub fn panes_hole_row(g: usize, bi: u32) -> usize {
    let ps = crate::panes::pn(g);
    for h in 0..ps.nh as usize {
        if ps.hole(h).bi == bi {
            return h;
        }
    }
    ps.add_hole_row(bi)
}
