//! THE AIR AS ONE FIELD — src/game/wind.ts's arithmetic: a diffuser's jet, the storm outdoors between the roofs, and the noise off the clock that gusts them. Pure
//! functions of a point, statement for statement, and the numbers of the field's shape: one home for them, here (`WIND_FIELD` in wind.ts reads them through
//! `wind_shape`, and render/shaders/wind_field.wgsl carries the same ones: tools/qa/wind-probes.ts compares). Who blows when (the sources, the master, the machine's
//! own settings) is the frame's dressing step and stays TypeScript; only `Wind.stormDrift`'s share reaches a rule, through the smoke grid.
//!
//! exp and the lengths are the game's own, so these are the game's answers wherever EXACT.on, as every sense's are.
use crate::maths::{exp, hypot2, hypot3};
use crate::world::{js_max, js_min, or_one};

/// the field's shape (wind.ts WIND_FIELD): how a jet blown at a floor turns along it, its support across, the storm's ceiling, its lee off a roof, its share of
/// itself at the ground and what it gains by `STORM_FULL_AT` metres up, how a vent's record becomes a jet
pub const DEFLECT_H: f64 = 0.55;
pub const DEFLECT_LOSS: f64 = 0.35;
pub const SUPPORT_SIGMA: f64 = 2.6;
pub const STORM_TOP: f64 = 8.0;
pub const STORM_LEE: f64 = 1.5;
pub const STORM_LOW: f64 = 0.55;
pub const STORM_GAIN: f64 = 0.45;
pub const STORM_FULL_AT: f64 = 2.5;
pub const CUT_K: f64 = 2.5;
pub const CORE_K: f64 = 0.4;
/// `WIND_FIELD` by place: the order wind.ts names them in
pub const SHAPE: [f64; 10] = [DEFLECT_H, DEFLECT_LOSS, SUPPORT_SIGMA, STORM_TOP, STORM_LEE, STORM_LOW, STORM_GAIN, STORM_FULL_AT, CUT_K, CORE_K];

/// wind.ts smoothstep
#[inline]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = js_max(0.0, js_min(1.0, (x - a) / (b - a)));
    t * t * (3.0 - 2.0 * t)
}

/// wind.ts jetVel: the jet's air at `p`
#[allow(clippy::too_many_arguments)]
pub fn jet_vel(pos: [f64; 3], dir: [f64; 3], speed: f64, reach: f64, tan_spread: f64, r0: f64, d_max: f64, floor_y: f64, p: [f64; 3]) -> [f64; 3] {
    let none = [0.0, 0.0, 0.0];
    if !(speed > 0.0) {
        return none;
    }
    let (qx, qy, qz) = (p[0] - pos[0], p[1] - pos[1], p[2] - pos[2]);
    let d = qx * dir[0] + qy * dir[1] + qz * dir[2];
    if d <= 0.0 || d >= d_max {
        return none;
    }
    let (rx, ry, rz) = (qx - dir[0] * d, qy - dir[1] * d, qz - dir[2] * d);
    let r = hypot3(rx, ry, rz);
    let sigma = r0 + d * tan_spread;
    if r > SUPPORT_SIGMA * sigma {
        return none;
    }
    let x = d / js_max(reach, 1e-3);
    let rs = r / sigma;
    let mut s = speed * exp(-rs * rs) / (1.0 + x * x) * (1.0 - smoothstep(0.7 * d_max, d_max, d));
    let ap = d + r0 / js_max(tan_spread, 0.02);
    let (mut ux, mut uy, mut uz) = (dir[0] * ap + rx, dir[1] * ap + ry, dir[2] * ap + rz);
    {
        let l = or_one(hypot3(ux, uy, uz));
        ux /= l;
        uy /= l;
        uz /= l;
    }
    if floor_y > -100.0 && dir[1] < -0.5 {
        let h = p[1] - floor_y;
        if h < 0.0 {
            return none;
        }
        if h < DEFLECT_H {
            let t = 1.0 - h / DEFLECT_H;
            let (mut hx, mut hz) = (rx, rz);
            let hl = hypot2(hx, hz);
            if hl > 1e-3 {
                hx /= hl;
                hz /= hl;
            } else {
                hx = dir[2] + 0.37;
                hz = 0.29 - dir[0];
                let l = hypot2(hx, hz);
                hx /= l;
                hz /= l;
            }
            let k = t * t;
            let (mx, my, mz) = (ux + (hx - ux) * k, uy + (0.0 - uy) * k, uz + (hz - uz) * k);
            let ml = or_one(hypot3(mx, my, mz));
            ux = mx / ml;
            uy = my / ml;
            uz = mz / ml;
            s *= 1.0 - DEFLECT_LOSS * t;
        }
    }
    [ux * s, uy * s, uz * s]
}

#[inline(always)]
fn in_rect(x: f64, z: f64, r: &[f64]) -> bool {
    x > r[0] && z > r[1] && x < r[2] && z < r[3]
}
/// wind.ts stormVel: `wind` outdoors, nothing inside the two footprints or over the storm's ceiling
pub fn storm_vel(wind: [f64; 3], interior: &[f64], annex: &[f64], p: [f64; 3]) -> [f64; 3] {
    if p[1] > STORM_TOP || p[1] < 0.0 || in_rect(p[0], p[2], interior) || in_rect(p[0], p[2], annex) {
        return [0.0, 0.0, 0.0];
    }
    let d_out = js_max(js_max(interior[0] - p[0], p[0] - interior[2]), js_max(interior[1] - p[2], p[2] - interior[3]));
    let k = smoothstep(0.0, STORM_LEE, d_out) * (STORM_LOW + STORM_GAIN * js_max(0.0, js_min(1.0, p[1] / STORM_FULL_AT)));
    [wind[0] * k, wind[1] * k, wind[2] * k]
}
/// wind.ts roofOut: how far (x, z) stands outside a roof along its nearer axis; a footprint with no width is no roof
#[inline]
fn roof_out(x: f64, z: f64, r: &[f64]) -> f64 {
    if !(r[2] > r[0]) || !(r[3] > r[1]) {
        1e6
    } else {
        js_max(js_max(r[0] - x, x - r[2]), js_max(r[1] - z, z - r[3]))
    }
}
/// wind.ts stormVelRoofs: `wind` wherever none of the `n` roofs (four doubles each, at most four) is overhead, easing in off the nearest
pub fn storm_vel_roofs(wind: [f64; 3], roofs: &[f64], n: usize, p: [f64; 3]) -> [f64; 3] {
    if p[1] > STORM_TOP || p[1] < 0.0 {
        return [0.0, 0.0, 0.0];
    }
    let mut d_out = 1e6;
    let mut i = 0;
    while i < n && i < 4 {
        d_out = js_min(d_out, roof_out(p[0], p[2], &roofs[i * 4..i * 4 + 4]));
        i += 1;
    }
    if d_out <= 0.0 {
        return [0.0, 0.0, 0.0];
    }
    let k = smoothstep(0.0, STORM_LEE, d_out) * (STORM_LOW + STORM_GAIN * js_max(0.0, js_min(1.0, p[1] / STORM_FULL_AT)));
    [wind[0] * k, wind[1] * k, wind[2] * k]
}

/// wind.ts hashU
#[inline]
fn hash_u(x: u32) -> u32 {
    let mut x = (x ^ 0x9E3779B9).wrapping_mul(0x85EBCA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2AE35);
    x ^= x >> 16;
    x
}
#[inline]
fn hash01(x: u32) -> f64 {
    (hash_u(x) >> 8) as f64 / 16777216.0
}
/// wind.ts vnoise: smooth value noise in [−1, 1] at x, one lattice value per integer, per seed
pub fn vnoise(seed: u32, x: f64) -> f64 {
    let i = prim::floor(x);
    let f = x - i;
    let lat = i as i64 as u32;   // (Math.imul takes its arguments modulo 2³²)
    let a = hash01(seed ^ lat.wrapping_mul(2654435761)) * 2.0 - 1.0;
    let b = hash01(seed ^ lat.wrapping_add(1).wrapping_mul(2654435761)) * 2.0 - 1.0;
    let t = f * f * (3.0 - 2.0 * f);
    a + (b - a) * t
}
