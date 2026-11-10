//! A LEAF OFF ITS HINGES, IN THE AIR: the loose slab a blast or a boot at a run sent out of its frame, for one tick. It flies, drags its foot, topples about it, is stopped dead
//! by the walls its rays find, is pushed out of what its toppling reaches, slams flat, slides out and comes to rest clear of every static.
//!
//! A slab is one record, on its door (`Door::slab`). The slabs still in the air are stepped one at a time, in the order of their doors, and each one's box is posed before the
//! next is stepped, so a slab's rays meet the ones before it where this tick left them. THE SLAB'S OWN BOX IS NOT A WALL TO IT: every ray leaves one leaf of the world's copy out.
use crate::doorblast::SlabMet;
use crate::doors::Slab;
use crate::knobs::{DoorblastK, Knobs};
use ::sim::collide;
use ::sim::lines::{ray_rule, RAY_HOST};
use ::sim::maths::{cos, exp, hypot2, sin};
use ::sim::world::{asking, clamp, js_abs, js_ceil, js_max, js_min, or_one, world, REC, R_C, R_COS, R_H, R_SIN};

const HALF_PI: f64 = core::f64::consts::PI / 2.0;
const PI: f64 = core::f64::consts::PI;
const LIFT: f64 = 0.005;
const G: f64 = 9.8;
/// statics one gather can list
const OV: usize = 512;

/// its door, and the world it flies in
pub struct Flight<'a> {
    pub w: usize,
    pub height: f64,
    pub thick: f64,
    pub hu: f64,
    pub closed: f64,
    pub fcx: f64,
    pub fcz: f64,
    /// which leaf of the world's copy is its own box (−1: none of them)
    pub but: i32,
    pub ask: bool,
    pub tune: &'a Knobs,
    pub k: &'a DoorblastK,
}

/// The slab's centre height with its foot (or its face) on the floor at this much tip
#[inline(always)]
fn support(d: &Flight, tip: f64) -> f64 {
    (d.height / 2.0) * cos(tip) + (d.thick / 2.0) * sin(tip) + LIFT
}
/// The slab's reach across its own line at this much tip
#[inline(always)]
fn across_of(d: &Flight, tip: f64) -> f64 {
    (d.height / 2.0) * sin(tip) + (d.thick / 2.0) * cos(tip)
}
/// Its half extent along the horizontal unit direction (ex, ez)
#[inline(always)]
fn reach_along(d: &Flight, s: &Slab, ex: f64, ez: f64) -> f64 {
    let (ux, uz) = (cos(s.a), sin(s.a));
    js_abs(ex * ux + ez * uz) * d.hu + js_abs(-ex * uz + ez * ux) * across_of(d, s.tip)
}
/// The nearest static, or other door's leaf, along a horizontal ray: how far, and the face's normal in plan
fn hit_at(d: &Flight, x: f64, y: f64, z: f64, ex: f64, ez: f64, tmax: f64) -> Option<(f64, f64, f64)> {
    let mut out = [0.0f64; 4];
    let k = ray_rule(d.w, x, y, z, ex, 0.0, ez, tmax, 0, d.but, d.ask, &mut out, d.tune);
    if k == RAY_HOST {
        panic!("{}", crate::scene_collision::no_number(None, None, None));
    }
    if k < 0 {
        return None;
    }
    Some((out[0], out[1], out[3]))
}
/// Does a floor rectangle (centre (x, z), heading a, half extents hu along it and hn across) cut into a static standing in the heights [y0, y1]?
#[allow(clippy::too_many_arguments)]
fn overlaps(d: &Flight, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64) -> bool {
    let (ux, uz, su, sn) = (cos(a), sin(a), hu - 0.012, hn - 0.012);
    let (ex, ez) = (js_abs(ux) * hu + js_abs(uz) * hn, js_abs(uz) * hu + js_abs(ux) * hn);
    let n = collide::gather(d.w, x - ex, z - ez, x + ex, z + ez, y0, y1, OV);
    let o = world(d.w);
    for i in 0..n {
        let b = asking(d.w).list[i] as usize * REC;
        let (bx, bz) = (o.rec[b + R_COS], -o.rec[b + R_SIN]);
        let (dx, dz) = (o.rec[b + R_C] - x, o.rec[b + R_C + 2] - z);
        let (h0, h2) = (o.rec[b + R_H], o.rec[b + R_H + 2]);
        let (c, s) = (js_abs(ux * bx + uz * bz), js_abs(ux * bz - uz * bx));
        if js_abs(dx * ux + dz * uz) > su + h0 * c + h2 * s {
            continue;
        }
        if js_abs(-dx * uz + dz * ux) > sn + h0 * s + h2 * c {
            continue;
        }
        if js_abs(dx * bx + dz * bz) > h0 + su * c + sn * s {
            continue;
        }
        if js_abs(-dx * bz + dz * bx) > h2 + su * s + sn * c {
            continue;
        }
        return true;
    }
    false
}
/// The nearest spot within max_r where that rectangle stands clear (rings of sixteen bearings, nearest first)
#[allow(clippy::too_many_arguments)]
fn free_spot(d: &Flight, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64, max_r: f64) -> Option<(f64, f64)> {
    let mut r = 0.03;
    while r <= max_r + 1e-9 {
        let mut i = 0.0;
        while i < 16.0 {
            let th = a + i * PI / 8.0;
            let (px, pz) = (x + cos(th) * r, z + sin(th) * r);
            if !overlaps(d, px, pz, a, hu, hn, y0, y1) {
                return Some((px, pz));
            }
            i += 1.0;
        }
        r += if r < 0.3 { 0.03 } else { 0.06 };
    }
    None
}
/// In the air, once out of its frame, a slab that has come to cut into a static is moved to the nearest clear spot and loses what speed it had into the thing
fn unstick(d: &Flight, s: &mut Slab) {
    let (hu, hn, y0, y1) = (d.hu, across_of(d, s.tip), 0.06, js_max(0.3, 2.0 * s.p[1] - 0.06));
    if s.t < s.wedged || !overlaps(d, s.p[0], s.p[2], s.a, hu, hn, y0, y1) {
        return;
    }
    let Some((t0, t1)) = free_spot(d, s.p[0], s.p[2], s.a, hu, hn, y0, y1, 0.45) else {
        s.wedged = s.t + 0.25;
        return;
    };
    let (sx, sz) = (t0 - s.p[0], t1 - s.p[2]);
    let l = or_one(hypot2(sx, sz));
    let vn = (s.v[0] * sx + s.v[2] * sz) / l;
    if vn < 0.0 {
        s.v[0] -= vn * sx / l;
        s.v[2] -= vn * sz / l;
    }
    s.p[0] = t0;
    s.p[2] = t1;
    s.spin *= 0.3;
}
/// At rest it lies clear of every static: where it stopped if that is clear; else the nearest clear spot, as it lies or turned a little; else laid through its own doorway
fn settle(d: &Flight, s: &mut Slab, met: &mut SlabMet) {
    let (hu, hn, y0, y1) = (d.hu, d.height / 2.0, 0.06, 0.3);
    if !overlaps(d, s.p[0], s.p[2], s.a, hu, hn, y0, y1) {
        return;
    }
    for da in [0.0, 0.2, -0.2, 0.45, -0.45, HALF_PI] {
        if let Some((t0, t1)) = free_spot(d, s.p[0], s.p[2], s.a + da, hu, hn, y0, y1, if da != 0.0 { 0.9 } else { 1.2 }) {
            met.nudge_max = js_max(met.nudge_max, hypot2(t0 - s.p[0], t1 - s.p[2]));
            s.p[0] = t0;
            s.p[2] = t1;
            s.a += da;
            return;
        }
    }
    let (a, fx, fz) = (d.closed, d.fcx, d.fcz);
    let to = if overlaps(d, fx, fz, a, hu, hn, y0, y1) { free_spot(d, fx, fz, a, hu, hn, y0, y1, 1.5) } else { Some((fx, fz)) };
    if let Some((t0, t1)) = to {
        s.p[0] = t0;
        s.p[2] = t1;
        s.a = a;
    }
}
/// Keep the slab's floor rectangle out of the statics as toppling (and its spin) lengthen it
fn push_out(d: &Flight, s: &mut Slab) {
    let (ux, uz) = (cos(s.a), sin(s.a));
    let (nx, nz, hu, hn) = (-uz, ux, d.hu, across_of(d, s.tip));
    let ym = clamp(s.p[1], 0.4, 1.3);
    let ways = [(nx, nz, hn, ux, uz, hu), (-nx, -nz, hn, ux, uz, hu), (ux, uz, hu, nx, nz, hn), (-ux, -uz, hu, nx, nz, hn)];
    for _pass in 0..2 {
        for (ex, ez, he, sx, sz, hs) in ways {
            let mut pen = 0.0;
            for f in [0.0, -0.8, 0.8] {
                let ys: &[f64] = if f == 0.0 { &[0.2, ym] } else { &[0.2] };
                for &y in ys {
                    if let Some((t, _, _)) = hit_at(d, s.p[0] + sx * hs * f, y, s.p[2] + sz * hs * f, ex, ez, he + 0.02) {
                        if he - t > pen {
                            pen = he - t;
                        }
                    }
                }
            }
            if pen > 0.0 {
                s.p[0] -= ex * (pen + 0.002);
                s.p[2] -= ez * (pen + 0.002);
                let vn = s.v[0] * ex + s.v[2] * ez;
                if vn > 0.0 {
                    s.v[0] -= vn * ex;
                    s.v[2] -= vn * ez;
                }
            }
        }
    }
}

/// One slab's tick, and what it met.
pub fn step(d: &Flight, s: &mut Slab, dt: f64) -> SlabMet {
    let mut met = SlabMet::default();
    let n = js_max(1.0, js_ceil(dt * 120.0));
    let h = dt / n;
    let mut i = 0.0;
    while i < n && !s.rest {
        i += 1.0;
        s.t += h;
        let flat = s.tip >= HALF_PI;
        if !flat && (s.tip_v > 0.0 || s.tip > 0.0) {
            s.tip_v += (1.5 * G / d.height) * sin(js_max(s.tip, 0.05)) * h;
            s.tip = js_min(HALF_PI, s.tip + s.tip_v * h);
        }
        // up and down: ballistic, never below what its tip stands it at
        let sup = support(d, s.tip);
        s.v[1] -= G * h;
        s.p[1] += s.v[1] * h;
        let down = s.p[1] <= sup;
        if down {
            s.p[1] = sup;
            if s.v[1] < 0.0 {
                s.v[1] = 0.0;
            }
        }
        // along the floor
        let mut sp = hypot2(s.v[0], s.v[2]);
        if sp > 1e-5 {
            let ns = if down { js_max(0.0, sp - (if flat { d.k.slide } else { d.k.scrape }) * h) } else { sp };
            s.v[0] *= ns / sp;
            s.v[2] *= ns / sp;
            sp = ns;
        }
        s.a += s.spin * h;
        if down {
            s.spin *= exp(-(if flat { 14.0 } else { 1.5 }) * h);
        }
        let mut hit_wall = false;
        if sp > 1e-5 {
            let (ex, ez, len) = (s.v[0] / sp, s.v[2] / sp, sp * h);
            let he = reach_along(d, s, ex, ez);
            let (mut adv, mut nn): (f64, Option<(f64, f64)>) = (len, None);
            for y in [0.2, clamp(s.p[1], 0.4, 1.3)] {
                if let Some((t, n0, n2)) = hit_at(d, s.p[0], y, s.p[2], ex, ez, len + he + 0.02) {
                    if t - he - 0.01 < adv {
                        adv = js_max(0.0, t - he - 0.01);
                        nn = Some((n0, n2));
                    }
                }
            }
            s.p[0] += ex * adv;
            s.p[2] += ez * adv;
            if let Some((n0, n2)) = nn {
                let nl = or_one(hypot2(n0, n2));
                let (wx, wz) = (n0 / nl, n2 / nl);
                let vn = s.v[0] * wx + s.v[2] * wz;
                if vn < 0.0 {
                    s.v[0] -= (1.0 + d.k.bounce) * vn * wx;
                    s.v[2] -= (1.0 + d.k.bounce) * vn * wz;
                }
                s.spin *= 0.3;
                hit_wall = true;
            }
        }
        if s.tip_v == 0.0 && s.tip == 0.0 && (hit_wall || sp < 0.4) {
            s.tip_v = d.k.tip; // (no room ahead: flat to the far wall first, and now it comes back down)
        }
        push_out(d, s);
        if s.t > 0.15 {
            unstick(d, s);
        }
        if (s.tip >= HALF_PI || s.t > 6.0) && !s.landed {
            // flat: the slam (the six-second cap lands it too)
            s.tip = HALF_PI;
            s.landed = true;
            s.p[1] = support(d, HALF_PI);
            settle(d, s, &mut met);
            s.v[0] *= d.k.slam;
            s.v[2] *= d.k.slam;
            if s.v[1] > 0.0 {
                s.v[1] = 0.0;
            }
            met.landed = true;
            met.lx = s.p[0];
            met.lz = s.p[2];
        }
        if s.landed && ((hypot2(s.v[0], s.v[2]) < 0.05 && js_abs(s.spin) < 0.08) || s.t > 6.0) {
            s.rest = true;
            s.v = [0.0, 0.0, 0.0];
            s.spin = 0.0;
            s.p[1] = support(d, HALF_PI);
            settle(d, s, &mut met);
        }
    }
    met.rested = s.rest;
    met
}
