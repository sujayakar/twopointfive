//! WHAT A PANE IS TO A ROUND: what a sheet takes from a round crossing it, how the round turns, where it
//! leaves, the flight over panes, and the through-holes rounds have left. One rule for a round and for a line-of-fire question: a man fires through a pane exactly
//! when his round would get through it. What a round does next and what it files is the caller's.
//!
//! WHAT IS STATE. The holes (`Panes.holes`): a row a pane a round has been through.
use game_types::knobs::Knobs;
use alloc::vec::Vec;
use crate::collide;
use crate::lines::{self, line_box, RAY_LEAF};
use crate::maths::{acos, cos, hypot3, sin, tan};
use crate::panes::{self, Hole, Panes, point_seed, type_of_pane, box_seed, ANNEALED, LAMINATED, NO_TYPE, TEMPERED};
use crate::world::{asking, clamp, js_abs, js_max, js_min, World, F_GLASS, F_NO_SHADOW, REC, R_COS, R_H, R_SIN};
use prim::sqrt;

const PI: f64 = core::f64::consts::PI;
const THICK_GLASS_HALF: f64 = 0.05;
/// the next leg is born this far past the face the round left the glass by
const EXIT_EPS: f64 = 1e-3;
/// the most panes one flight is followed through (GLASSPEN.maxPanes is under this)
pub const MAX_CROSS: usize = 16;

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
fn sheet_box(o: &World, bi: usize) -> Option<&[f64]> {
    if bi >= o.boxes() || o.flags[bi] & F_GLASS == 0 {
        return None;
    }
    let b = o.row(bi);
    if js_min(b[R_H], b[R_H + 2]) <= THICK_GLASS_HALF {
        Some(b)
    } else {
        None
    }
}
fn near_hole(p: &Panes, bi: u32, at: [f64; 3], tune: &Knobs) -> bool {
    let r2 = tune.glasspen.hole_r * tune.glasspen.hole_r;
    let near = |h: &[f64; 3]| {
        let (dx, dy, dz) = (h[0] - at[0], h[1] - at[1], h[2] - at[2]);
        dx * dx + dy * dy + dz * dz < r2
    };
    p.holes.iter().find(|h| h.bi == bi).is_some_and(|h| h.at.iter().any(near))
}
/// What sheet `bi` takes from a round of energy `e` meeting it at `at` flying `d` (unit), the `nth` pane of its flight; None: the round is spent in it
pub fn pane_toll(w: &World, bi: u32, at: [f64; 3], d: [f64; 3], e: f64, nth: usize, tune: &Knobs) -> Option<Toll> {
    let b = sheet_box(w, bi as usize)?;
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
    let hole = near_hole(&w.panes, bi, at, tune);
    let ty = match type_of_pane(w, bi, tune) {
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
fn born_in(w: &World, p: [f64; 3], d: [f64; 3], me: u32, tune: &Knobs) -> i32 {
    let n = collide::gather(w, p[0] - 0.02, p[2] - 0.02, p[0] + 0.02, p[2] + 0.02, p[1] - 0.02, p[1] + 0.02, 64);
    let mut sheet = -1;
    for i in 0..n {
        let o = w;
        let bi = asking().list[i];
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
    let e1 = game_types::v3::normalize(cross(up, d));
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
    game_types::v3::normalize(out)
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
pub fn pane_crossing(w: &World, bi: u32, o: [f64; 3], d: [f64; 3], t: f64, e: f64, nth: usize, tune: &Knobs) -> Option<Crossing> {
    let at = mad(o, d, t);
    let to = pane_toll(w, bi, at, d, e, nth, tune)?;
    let b = sheet_box(w, bi as usize)?;
    let te = match line_box(w, at[0], at[1], at[2], d[0], d[1], d[2], b) {
        Some(s) => s.1,
        None => -1.0,
    };
    let thru = (if te > 0.0 { te } else { 2.0 * js_min(b[R_H], b[R_H + 2]) / to.cos }) + EXIT_EPS;
    let exit = mad(at, d, thru);
    let into = born_in(w, exit, d, bi, tune);
    if into == -2 {
        return None;
    }
    let dir_out = if into >= 0 { d } else { deflect(b, at, d, &to, e, tune) };
    let turn_deg = acos(clamp(d[0] * dir_out[0] + d[1] * dir_out[1] + d[2] * dir_out[2], -1.0, 1.0)) * 180.0 / PI;
    Some(Crossing { t: to, bi, at_t: t, at, exit, dir_in: d, dir_out, e_in: e, turn_deg, thru, into })
}
#[inline]
fn ray(w: &World, o: [f64; 3], d: [f64; 3], tmax: f64, out: &mut [f64; 4], tune: &Knobs) -> i32 {
    lines::ray_rule(w, o[0], o[1], o[2], d[0], d[1], d[2], tmax, -1, true, out, tune)
}
/// The arm's line met nothing: is that because every sheet of glass across it has fallen where the line crosses it (and there is one)?
fn arm_in_hole(w: &World, from: [f64; 3], rd: [f64; 3], len: f64, tune: &Knobs) -> bool {
    let mut any = false;
    let o = w;
    for s in &w.sheets {
        let bi = s.bi as usize;
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
/// The walk of a man's arm whose muzzle is already past the glass: the energy left and the panes it took, in order (none and in a hole: `energy`). None: something a
/// round stops in is across the arm.
pub fn arm_walk(w: &World, from: [f64; 3], muzzle: [f64; 3], dir: [f64; 3], energy: f64, tune: &Knobs) -> Option<(f64, Vec<Crossing>)> {
    let dd = [muzzle[0] - from[0], muzzle[1] - from[1], muzzle[2] - from[2]];
    let len = hypot3(dd[0], dd[1], dd[2]);
    if len < 1e-3 {
        return None;
    }
    let sc = 1.0 / len;
    let rd = [dd[0] * sc, dd[1] * sc, dd[2] * sc];
    let (mut o, mut t0, mut e) = (from, 0.0, energy);
    let mut out: Vec<Crossing> = Vec::new();
    let mut k8 = 0;
    while k8 < 8 && t0 < len {
        k8 += 1;
        let mut h = [0.0; 4];
        let m = ray(w, o, rd, len - t0, &mut h, tune);
        if m < 0 {
            break;
        }
        if m >= RAY_LEAF {
            return None;
        }
        let at = mad(o, rd, h[0]);
        let to = match pane_toll(w, m as u32, at, dir, e, out.len(), tune) {
            Some(t) => t,
            None => return None,
        };
        let b = match sheet_box(w, m as usize) {
            Some(b) => b,
            None => return None,
        };
        let te = match line_box(w, at[0], at[1], at[2], rd[0], rd[1], rd[2], b) {
            Some(s) => s.1,
            None => 0.0,
        };
        let thru = te + EXIT_EPS;
        let x = Crossing { t: to, bi: m as u32, at_t: t0 + h[0], at, exit: mad(at, rd, thru), dir_in: dir, dir_out: dir, e_in: e, turn_deg: 0.0, thru, into: -1 };
        out.push(x);
        e = to.e_out;
        t0 += h[0] + thru;
        o = mad(at, rd, thru);
    }
    if out.is_empty() {
        return if arm_in_hole(w, from, rd, len, tune) { Some((energy, out)) } else { None };
    }
    Some((e, out))
}
/// Does a round of `energy` get through every one of the panes a line of fire meets (static, metres along it to where it enters each, nearest first)
pub fn shot_passes(w: &World, a: [f64; 3], rd: [f64; 3], panes: &[(u32, f64)], energy: f64, tune: &Knobs) -> bool {
    let mut e = energy;
    for (i, &(bi, t)) in panes.iter().enumerate() {
        match pane_toll(w, bi, mad(a, rd, t), rd, e, i, tune) {
            Some(t) => e = t.e_out,
            None => return false,
        }
    }
    true
}
/// A crossing remembered on the pane, the oldest forgotten past GLASSPEN.holes
pub fn add_hole(p: &mut Panes, bi: u32, at: [f64; 3], tune: &Knobs) {
    let i = p.holes.iter().position(|h| h.bi == bi).unwrap_or_else(|| {
        p.holes.push(Hole { bi, at: Vec::new() });
        p.holes.len() - 1
    });
    let h = &mut p.holes[i].at;
    if h.len() >= js_min(panes::HOLE_PTS as f64, tune.glasspen.holes) as usize {
        h.remove(0);
    }
    h.push(at);
}
