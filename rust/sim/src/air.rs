//! THE WEATHER'S DUST IN THE ROOMS: the share of the open air's density at each half metre of the plan, what the men's eyes read along a line (game_rules air/weather.rs
//! air_trans), carried through the openings between cells and settling back to the roof's leak. The field is a record of the game (game_types `AirDust`).
//!
//! THE ORDER. `begin`, then `clear` where there is no weather or `seed` where there is; `settle` once where the switch is on, so the level opens with the dust in its open
//! doorways; then `update` a tick.
//!
//! An opening between two cells is asked of the statics alone (three heights, both ways), and of the door leaves as rectangles, re-read whenever a leaf has moved.
//! Which cells are roofed and the lists walked over them are the roofs', and which edges a leaf stands across is the leaves' (`AirPlan`).
use alloc::{sync::Arc, vec, vec::Vec};
use game_types::air_dust::{AirDust, AirPlan, Roofed};
use game_types::knobs::Knobs;
use crate::lines;
use crate::maths::{cos, hypot2, sin};
use crate::world::{js_abs, js_ceil, js_max, js_min, K_ROT, REC, R_C, R_H, R_KIND, R_YAW, World};
use prim::floor;

/// the plan: half-metre cells over the 40 × 28 m world (the host holds its own numbers to these as it stands up: game_rules air/air_dust.rs)
pub const AIR_W: usize = 80;
pub const AIR_H: usize = 56;
pub const AIR_CELL: f64 = 0.5;
pub const N: usize = AIR_W * AIR_H;
/// the world's corner on the plan
pub const X0: f64 = 0.0;
pub const Z0: f64 = 0.0;
/// the heights a way between two cells is asked at
const PROBE_Y: [f64; 3] = [1.6, 0.4, 2.05];
/// a share this close to the leak IS the leak
const SNAP: f64 = 1e-6;

/// the cells `roofed` answers for by their centres, and the edges over them
fn under(roofed: impl Fn(f64, f64) -> bool) -> (Vec<bool>, Roofed) {
    let mut indoor = vec![false; N];
    let mut r = Roofed::default();
    for (i, is) in indoor.iter_mut().enumerate() {
        *is = roofed(X0 + ((i % AIR_W) as f64 + 0.5) * AIR_CELL, Z0 + ((i / AIR_W) as f64 + 0.5) * AIR_CELL);
        if *is {
            r.cells.push(i as u32);
        }
    }
    for i in 0..N {
        if i % AIR_W + 1 < AIR_W && (indoor[i] || indoor[i + 1]) {
            r.edges.push((i * 2) as u32);
        }
        if i / AIR_W + 1 < AIR_H && (indoor[i] || indoor[i + AIR_W]) {
            r.edges.push((i * 2 + 1) as u32);
        }
    }
    (indoor, r)
}

/// the fixed step in use: held to what the explicit scheme and a frame can take
#[inline]
fn step_secs(tune: &Knobs) -> f64 {
    let h = tune.airdust.step;
    if h >= 1.0 / 240.0 && h <= 0.1 {
        h
    } else {
        1.0 / 30.0
    }
}
/// `+x || 0`
#[inline(always)]
fn or_zero(x: f64) -> f64 {
    if x != x {
        0.0
    } else {
        x
    }
}


/// a build's head: its switch, nothing owed, no opening known yet
pub fn begin(d: &mut AirDust, on: bool) {
    d.on = on;
    d.acc = 0.0;
    d.cur = 0;
    d.plan = AirPlan { under: d.plan.under.clone(), leaf: vec![0; N * 2], read: None };
    d.open = vec![0; N * 2];
}
/// no weather in the air: every cell the open air's
pub fn clear(d: &mut AirDust) {
    d.share = Vec::new();
    d.plan.under = Arc::default();
}
/// Off the grid = the open air
#[inline]
pub fn share_at(d: &AirDust, x: f64, z: f64) -> f64 {
    let ix = floor((x - X0) / AIR_CELL);
    let iz = floor((z - Z0) / AIR_CELL);
    if ix < 0.0 || iz < 0.0 || ix >= AIR_W as f64 || iz >= AIR_H as f64 {
        1.0
    } else {
        d.share.get(ix as usize + iz as usize * AIR_W).copied().unwrap_or(1.0)
    }
}
/// Is the way along edge `e` clear of every static
fn ask(wo: &World, e: usize, tune: &Knobs) -> bool {
    let i = e >> 1;
    let j = if e & 1 != 0 { i + AIR_W } else { i + 1 };
    let ax = X0 + ((i % AIR_W) as f64 + 0.5) * AIR_CELL;
    let az = Z0 + ((i / AIR_W) as f64 + 0.5) * AIR_CELL;
    let bx = X0 + ((j % AIR_W) as f64 + 0.5) * AIR_CELL;
    let bz = Z0 + ((j / AIR_W) as f64 + 0.5) * AIR_CELL;
    for y in PROBE_Y.iter() {
        if lines::solid_line(wo, false, lines::ASK_GLASS, ax, *y, az, bx, *y, bz, tune) != Some(true) && lines::solid_line(wo, false, lines::ASK_GLASS, bx, *y, bz, ax, *y, az, tune) != Some(true) {
            return true;
        }
    }
    false
}
/// The shut edges asked again, a share of the list a step
fn recheck(d: &mut AirDust, wo: &World, h: f64, tune: &Knobs) {
    let under = d.plan.under.clone();
    let n = under.edges.len();
    if n == 0 {
        return;
    }
    let per = js_min(n as f64, js_max(1.0, js_ceil(n as f64 * h / js_max(0.25, or_zero(tune.airdust.recheck_secs))))) as usize;
    let mut cur = d.cur;
    for _ in 0..per {
        let e = under.edges[cur] as usize;
        cur = (cur + 1) % n;
        if d.open[e] == 0 && ask(wo, e, tune) {
            d.open[e] = 1;
        }
    }
    d.cur = cur;
}
/// Which edges the door leaves stand across, re-read only when a leaf has moved
fn leaves(p: &mut AirPlan, wo: &World) {
    let o = wo;
    let n = o.leaves();
    let rows = &o.leaf;
    let mut sig = n as i32;
    for i in 0..n {
        let q = i * REC;
        let rot = (rows[q + R_KIND] as u32) & K_ROT != 0;
        sig = (sig ^ js_round(rows[q + R_C] * 1000.0)).wrapping_mul(16777619);
        sig = (sig ^ js_round(rows[q + R_C + 2] * 1000.0)).wrapping_mul(16777619);
        sig = (sig ^ js_round(rows[q + R_YAW] * 1000.0)).wrapping_mul(16777619);
        sig = (sig ^ js_round(rows[q + R_C + 1] * 1000.0) ^ (if rot { 0x5bd1e995 } else { 0 })).wrapping_mul(16777619);
    }
    if p.read == Some(sig) {
        return;
    }
    p.read = Some(sig);
    p.leaf.fill(0);
    let (w_max, h_max) = ((AIR_W - 1) as f64, (AIR_H - 1) as f64);
    for i in 0..n {
        let q = i * REC;
        let (c0, c1, c2, h0, h1, h2) = (rows[q + R_C], rows[q + R_C + 1], rows[q + R_C + 2], rows[q + R_H], rows[q + R_H + 1], rows[q + R_H + 2]);
        let rot = (rows[q + R_KIND] as u32) & K_ROT != 0;
        if rot || c1 - h1 > 1.0 || c1 + h1 < 1.0 {
            continue;
        }
        let yaw = rows[q + R_YAW];
        let (cs, sn) = (cos(yaw), sin(yaw));
        let (hx, hz) = (h0 + 0.02, h2 + 0.02);
        let r = hypot2(h0, h2) + AIR_CELL;
        let ix0 = js_max(0.0, floor((c0 - r - X0) / AIR_CELL - 0.5)) as usize;
        let ix1 = js_min(w_max, js_ceil((c0 + r - X0) / AIR_CELL - 0.5));
        let iz0 = js_max(0.0, floor((c2 - r - Z0) / AIR_CELL - 0.5)) as usize;
        let iz1 = js_min(h_max, js_ceil((c2 + r - Z0) / AIR_CELL - 0.5));
        let mut iz = iz0;
        while iz as f64 <= iz1 {
            let mut ix = ix0;
            while ix as f64 <= ix1 {
                let px = X0 + (ix as f64 + 0.5) * AIR_CELL - c0;
                let pz = Z0 + (iz as f64 + 0.5) * AIR_CELL - c2;
                let lx = cs * px - sn * pz;
                let lz = sn * px + cs * pz;
                let c = ix + iz * AIR_W;
                if ix + 1 < AIR_W && crosses(lx, lz, cs * AIR_CELL, sn * AIR_CELL, hx, hz) {
                    p.leaf[c * 2] = p.leaf[c * 2].wrapping_add(1);
                }
                if iz + 1 < AIR_H && crosses(lx, lz, -sn * AIR_CELL, cs * AIR_CELL, hx, hz) {
                    p.leaf[c * 2 + 1] = p.leaf[c * 2 + 1].wrapping_add(1);
                }
                ix += 1;
            }
            iz += 1;
        }
    }
}
/// One step of the equation → the most any cell moved
fn step_once(d: &mut AirDust, dt: f64, tune: &Knobs) -> f64 {
    let AirDust { share, open, plan, leak, .. } = d;
    let leak = *leak;
    let ks = js_min(0.2, js_max(0.0, or_zero(tune.airdust.diffuse)) * dt / (AIR_CELL * AIR_CELL));
    let r = js_min(0.5, js_max(0.0, or_zero(tune.airdust.settle)) * dt);
    let mut moved = 0.0;
    let through = |e: usize| open[e] != 0 && plan.leaf[e] == 0;
    let mut next = Vec::with_capacity(plan.under.cells.len());
    for &i in &plan.under.cells {
        let i = i as usize;
        let v = share[i];
        let ix = i % AIR_W;
        let mut a = 0.0;
        if ix > 0 && through((i - 1) * 2) {
            a += share[i - 1] - v;
        }
        if ix + 1 < AIR_W && through(i * 2) {
            a += share[i + 1] - v;
        }
        if i >= AIR_W && through((i - AIR_W) * 2 + 1) {
            a += share[i - AIR_W] - v;
        }
        if i + AIR_W < N && through(i * 2 + 1) {
            a += share[i + AIR_W] - v;
        }
        let mut w = v + ks * a - r * (v - leak);
        if js_abs(w - leak) < SNAP {
            w = leak;
        }
        moved = js_max(moved, js_abs(w - v));
        next.push(w);
    }
    for (&i, w) in plan.under.cells.iter().zip(next) {
        share[i as usize] = w;
    }
    moved
}
/// once the roofs have been asked: the field from nothing. `roofed` answers for a cell's centre.
pub fn seed(d: &mut AirDust, leak: f64, roofed: impl Fn(f64, f64) -> bool) {
    d.leak = leak;
    let (indoor, lists) = under(roofed);
    d.plan.under = Arc::new(lists);
    d.share = indoor.into_iter().map(|is| if is { leak } else { 1.0 }).collect();
}
/// …and with the switch on: every opening asked, the leaves read, and run on until it stands still (the level opens with the dust already in its open doorways)
pub fn settle(d: &mut AirDust, wo: &World, tune: &Knobs) {
    let under = d.plan.under.clone();
    for &e in &under.edges {
        d.open[e as usize] = ask(wo, e as usize, tune) as u8;
    }
    leaves(&mut d.plan, wo);
    let h = step_secs(tune);
    for _ in 0..1500 {
        if !(step_once(d, h, tune) > 1e-4) {
            break;
        }
    }
}
/// Through the openings as they stand this tick
pub fn update(d: &mut AirDust, wo: &World, dt: f64, tune: &Knobs) {
    let h = step_secs(tune);
    d.acc += dt;
    let mut n = 0;
    while d.acc >= h && n < 4 {
        n += 1;
        d.acc -= h;
        recheck(d, wo, h, tune);
        leaves(&mut d.plan, wo);
        step_once(d, h, tune);
    }
    if d.acc > h {
        d.acc = h;
    }
}

/// a number a leaf's place makes, rounded half up, as a word (wrapping to 32 bits)
#[inline(always)]
fn js_round(x: f64) -> i32 {
    floor(x + 0.5) as i64 as i32
}
/// Does the plan segment from (x, z) along (dx, dz), t in [0, 1], enter the rectangle |x| ≤ hx, |z| ≤ hz
#[inline]
fn crosses(x: f64, z: f64, dx: f64, dz: f64, hx: f64, hz: f64) -> bool {
    let (mut t0, mut t1) = (0.0, 1.0);
    if js_abs(dx) < 1e-9 {
        if js_abs(x) > hx {
            return false;
        }
    } else {
        let (mut a, mut b) = ((-hx - x) / dx, (hx - x) / dx);
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        if a > t0 {
            t0 = a;
        }
        if b < t1 {
            t1 = b;
        }
        if t0 > t1 {
            return false;
        }
    }
    if js_abs(dz) < 1e-9 {
        if js_abs(z) > hz {
            return false;
        }
    } else {
        let (mut a, mut b) = ((-hz - z) / dz, (hz - z) / dz);
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        if a > t0 {
            t0 = a;
        }
        if b < t1 {
            t1 = b;
        }
        if t0 > t1 {
            return false;
        }
    }
    true
}
