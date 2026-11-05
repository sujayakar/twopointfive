//! The weather's dust in the rooms: a field over the plan, carried through openings, read by the men's eyes. (src/game/airDust.ts)
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

const AIR_CELL: f64 = crate::scene_air_grid::AIR_CELL;

/// Cells on the plan. (airDust.ts N)
fn cell_count() -> usize {
    n() as usize
}

/// A door leaf's `c`, `h`, `yaw` and whether it has `rot` (airDust.ts:146, 151, 152): BoxId has no table (ids.rs "no table here", properties.txt `g.none[id]`).
fn seam_box_pose(g: &mut Game, b: BoxId) -> (V3, V3, f64, bool) {
    todo!("needs: a box's c, h, yaw, rot by BoxId")
}

/// The men's eyes read the field this run. (airDust.ts AirDust.live)
pub fn air_dust_live(g: &mut crate::game::Game) -> bool {
    air_dust_ensure(g);
    g.air_dust.on
}

/// A new field, built against the game as it stands; the caller stores it at `g.air_dust`. (airDust.ts AirDust.constructor)
pub fn air_dust_new(g: &mut crate::game::Game) -> crate::air_dust::AirDust {
    let n = cell_count();
    let fresh = AirDust {
        s: vec![1.0; n],
        indoor: vec![0; n],
        stat: vec![0; n * 2],
        leaf: vec![0; n * 2],
        next: vec![0.0; n],
        gpu: vec![0.0; crate::scene_air_grid::air_rows() as usize * 4],
        ..AirDust::blank()
    };
    let was = core::mem::replace(&mut g.air_dust, fresh);
    air_dust_ensure(g);
    core::mem::replace(&mut g.air_dust, was)
}

/// The share at a spot of the plan; off the grid = the open air. (airDust.ts AirDust.shareAt)
pub fn air_dust_share_at(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    if g.air_dust.dirty {
        air_dust_ensure(g);
    }
    let min = crate::scene_world::world().min;
    let (w, h) = (crate::scene_air_grid::air_w(), crate::scene_air_grid::air_h());
    let ix = m::floor((x - min[0]) / AIR_CELL);
    let iz = m::floor((z - min[2]) / AIR_CELL);
    if ix < 0.0 || iz < 0.0 || ix >= w || iz >= h {
        return 1.0;
    }
    // (a NaN spot passes the test above; the TypeScript's read is then undefined, a NaN to every reader)
    if ix.is_nan() || iz.is_nan() {
        return m::NAN;
    }
    g.air_dust.s[(ix + iz * w) as usize]
}

/// The roofs or the weather changed: marks the field stale, the first reader builds it. (airDust.ts AirDust.reseed)
pub fn air_dust_reseed(g: &mut crate::game::Game) {
    g.air_dust.dirty = true;
}

/// Builds the field if it is stale. (airDust.ts AirDust.ensure)
pub fn air_dust_ensure(g: &mut crate::game::Game) {
    if g.air_dust.dirty {
        g.air_dust.dirty = false;
        air_dust_build(g);
    }
}

/// The field from nothing: every roofed cell at the leak, every opening asked, then run on until it stands still. (airDust.ts AirDust.build)
pub fn air_dust_build(g: &mut crate::game::Game) {
    let leak = g.weather.fog.as_ref().map(|f| f.leak);
    let has = g.weather.kind.is_some() && leak.is_some();
    {
        let d = &mut g.air_dust;
        d.on = has; // (AIRDUST.on is settled true on the page)
        d.acc = 0.0;
        d.drift = 0.0;
        d.cur = 0.0;
        d.leaf_sig = m::NAN;
        d.leaf.fill(0);
        d.stat.fill(0);
    }
    let leak = match (has, leak) {
        (true, Some(l)) => l,
        _ => {
            let d = &mut g.air_dust;
            if d.seeded {
                d.seeded = false;
                d.s.fill(1.0);
                d.indoor.fill(0);
                d.cells = Vec::new();
                d.edges = Vec::new();
            }
            return;
        }
    };
    g.air_dust.seeded = true;
    g.air_dust.leak = leak;

    let min = crate::scene_world::world().min;
    let w = crate::scene_air_grid::air_w() as usize;
    let h = crate::scene_air_grid::air_h() as usize;
    let mut cells: Vec<i32> = Vec::new();
    let mut edges: Vec<i32> = Vec::new();
    let mut i = 0usize;
    for iz in 0..h {
        for ix in 0..w {
            let r = crate::weather::weather_under_roof(g, min[0] + (ix as f64 + 0.5) * AIR_CELL, min[2] + (iz as f64 + 0.5) * AIR_CELL);
            g.air_dust.indoor[i] = if r { 1 } else { 0 };
            g.air_dust.s[i] = if r { leak } else { 1.0 };
            if r {
                cells.push(i as i32);
            }
            i += 1;
        }
    }
    let mut i = 0usize;
    for iz in 0..h {
        for ix in 0..w {
            let d = &g.air_dust;
            if ix + 1 < w && (d.indoor[i] != 0 || d.indoor[i + 1] != 0) {
                edges.push((i * 2) as i32);
            }
            if iz + 1 < h && (d.indoor[i] != 0 || d.indoor[i + w] != 0) {
                edges.push((i * 2 + 1) as i32);
            }
            i += 1;
        }
    }
    g.air_dust.stats.roofed = cells.len() as f64;
    g.air_dust.stats.edges = edges.len() as f64;
    g.air_dust.cells = cells;
    g.air_dust.edges = edges;

    if g.air_dust.on {
        air_dust_clear_of_leaves(g, &mut |g: &mut Game| {
            for q in 0..g.air_dust.edges.len() {
                let e = g.air_dust.edges[q];
                let open = air_dust_ask(g, e as f64);
                g.air_dust.stat[e as usize] = if open { 1 } else { 0 };
            }
        });
        air_dust_leaves(g);
        let mut k = 0;
        while k < 1500 {
            let h = step_secs(g);
            if !(air_dust_step_once(g, h) > 1e-4) {
                break;
            }
            k += 1;
        }
    }
    air_dust_send(g);
}

/// The level as authored again. (airDust.ts AirDust.reset)
pub fn air_dust_reset(g: &mut crate::game::Game) {
    air_dust_reseed(g);
}

/// Once a frame, after the doors and whatever breaks have moved: fixed steps off an accumulator. (airDust.ts AirDust.update)
pub fn air_dust_update(g: &mut crate::game::Game, dt: f64) {
    air_dust_ensure(g);
    if !g.air_dust.on {
        return;
    }
    let h = step_secs(g);
    g.air_dust.acc += dt;
    let mut n = 0;
    while g.air_dust.acc >= h && n < 4 {
        n += 1;
        g.air_dust.acc -= h;
        air_dust_recheck(g, h);
        air_dust_leaves(g);
        let moved = air_dust_step_once(g, h);
        g.air_dust.drift += moved;
    }
    if g.air_dust.acc > h {
        g.air_dust.acc = h; // (a long frame: the rest of it is dropped, not owed)
    }
    if g.air_dust.drift > SEND {
        air_dust_send(g);
    }
}

/// Runs `fn_` with the door leaves taken out of the collision's list. (airDust.ts AirDust.clearOfLeaves)
pub fn air_dust_clear_of_leaves(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game)) {
    let dyn_ = core::mem::take(&mut g.col.dynamic_boxes);
    fn_(g);
    g.col.dynamic_boxes = dyn_;
}

/// Is the way along edge `e` clear of every static (call inside clearOfLeaves). (airDust.ts AirDust.ask)
pub fn air_dust_ask(g: &mut crate::game::Game, e: f64) -> bool {
    let e = js::to_i32(e);
    let min = crate::scene_world::world().min;
    let w = crate::scene_air_grid::air_w() as i32;
    let i = e >> 1;
    let j = if e & 1 != 0 { i + w } else { i + 1 };
    let mut a: V3 = [0.0; 3];
    let mut b: V3 = [0.0; 3];
    a[0] = min[0] + ((i % w) as f64 + 0.5) * AIR_CELL;
    a[2] = min[2] + ((i / w) as f64 + 0.5) * AIR_CELL;
    b[0] = min[0] + ((j % w) as f64 + 0.5) * AIR_CELL;
    b[2] = min[2] + ((j / w) as f64 + 0.5) * AIR_CELL;
    for y in probe_y() {
        a[1] = y;
        b[1] = y;
        g.air_dust.stats.probes += 1.0;
        if !crate::scene_collision::solid_blocked(g, a, b, None) && !crate::scene_collision::solid_blocked(g, b, a, None) {
            return true;
        }
    }
    false
}

/// The shut edges asked again, a share of the list each step (a wall that broke, a pane that fell out). (airDust.ts AirDust.recheck)
pub fn air_dust_recheck(g: &mut crate::game::Game, h: f64) {
    let len = g.air_dust.edges.len();
    if len == 0 {
        return;
    }
    let l = len as f64;
    let per = m::min(l, m::max(1.0, m::ceil(l * h / m::max(0.25, js::or(g.k.airdust.recheck_secs, 0.0)))));
    let per = per as usize;
    air_dust_clear_of_leaves(g, &mut |g: &mut Game| {
        for _ in 0..per {
            let e = g.air_dust.edges[g.air_dust.cur as usize];
            g.air_dust.cur = js::rem(g.air_dust.cur + 1.0, l);
            if g.air_dust.stat[e as usize] == 0 && air_dust_ask(g, e as f64) {
                g.air_dust.stat[e as usize] = 1;
            }
        }
    });
}

/// Which edges the door leaves stand across: re-read only when a leaf has moved since the last look. (airDust.ts AirDust.leaves)
pub fn air_dust_leaves(g: &mut crate::game::Game) {
    let count = g.col.dynamic_boxes.len();
    // (a 32-bit hash of every leaf's pose to the millimetre / milliradian)
    let mut sig = count as i32;
    for q in 0..count {
        let b = g.col.dynamic_boxes[q];
        let (c, _h, yaw, rot) = seam_box_pose(g, b);
        sig = m::imul(sig ^ milli(c[0]), 16777619);
        sig = m::imul(sig ^ milli(c[2]), 16777619);
        sig = m::imul(sig ^ milli(yaw), 16777619);
        sig = m::imul(sig ^ milli(c[1]) ^ (if rot { 0x5bd1e995 } else { 0 }), 16777619);
    }
    if sig as f64 == g.air_dust.leaf_sig {
        return;
    }
    g.air_dust.leaf_sig = sig as f64;
    g.air_dust.leaf.fill(0);

    let min = crate::scene_world::world().min;
    let (x0, z0) = (min[0], min[2]);
    let w = crate::scene_air_grid::air_w();
    let h = crate::scene_air_grid::air_h();
    for q in 0..count {
        let bx = g.col.dynamic_boxes[q];
        let (c, bh, yaw, rot) = seam_box_pose(g, bx);
        if rot || c[1] - bh[1] > 1.0 || c[1] + bh[1] < 1.0 {
            continue; // (a leaf off its hinges lies where it fell: nothing to the air)
        }
        let cs = m::cos(yaw);
        let sn = m::sin(yaw);
        let hx = bh[0] + 0.02;
        let hz = bh[2] + 0.02;
        let r = m::hyp2(bh[0], bh[2]) + AIR_CELL;
        let ix0 = m::max(0.0, m::floor((c[0] - r - x0) / AIR_CELL - 0.5));
        let ix1 = m::min(w - 1.0, m::ceil((c[0] + r - x0) / AIR_CELL - 0.5));
        let iz0 = m::max(0.0, m::floor((c[2] - r - z0) / AIR_CELL - 0.5));
        let iz1 = m::min(h - 1.0, m::ceil((c[2] + r - z0) / AIR_CELL - 0.5));
        let leaf = &mut g.air_dust.leaf;
        // (doubles, as the TypeScript's: a NaN bound walks nothing)
        let mut iz = iz0;
        while iz <= iz1 {
            let mut ix = ix0;
            while ix <= ix1 {
                let px = x0 + (ix + 0.5) * AIR_CELL - c[0];
                let pz = z0 + (iz + 0.5) * AIR_CELL - c[2];
                let lx = cs * px - sn * pz;
                let lz = sn * px + cs * pz;
                let i = (ix + iz * w) as usize;
                // the edge to +x is the local segment (lx, lz) → (lx + cs·h, lz + sn·h); to +z: → (lx − sn·h, lz + cs·h)
                if ix + 1.0 < w && crosses(lx, lz, cs * AIR_CELL, sn * AIR_CELL, hx, hz) {
                    leaf[i * 2] = leaf[i * 2].wrapping_add(1);
                }
                if iz + 1.0 < h && crosses(lx, lz, -sn * AIR_CELL, cs * AIR_CELL, hx, hz) {
                    leaf[i * 2 + 1] = leaf[i * 2 + 1].wrapping_add(1);
                }
                ix += 1.0;
            }
            iz += 1.0;
        }
    }
}

/// `Math.round(x * 1000)` as an operand of `^`. (airDust.ts:146)
fn milli(x: f64) -> i32 {
    js::to_i32(m::round(x * 1000.0))
}

/// One step of the diffusion → the most any cell moved. (airDust.ts AirDust.stepOnce)
pub fn air_dust_step_once(g: &mut crate::game::Game, dt: f64) -> f64 {
    let k = m::min(0.2, m::max(0.0, js::or(g.k.airdust.diffuse, 0.0)) * dt / (AIR_CELL * AIR_CELL));
    let r = m::min(0.5, m::max(0.0, js::or(g.k.airdust.settle, 0.0)) * dt);
    let w = crate::scene_air_grid::air_w() as usize;
    let n = cell_count();
    let d = &mut g.air_dust;
    let leak = d.leak;
    let mut moved = 0.0;
    let mut open = 0.0;
    for q in 0..d.cells.len() {
        let i = d.cells[q] as usize;
        let v = d.s[i];
        let ix = i % w;
        let mut a = 0.0;
        if ix > 0 {
            let e = (i - 1) * 2;
            if d.stat[e] != 0 && d.leaf[e] == 0 {
                a += d.s[i - 1] - v;
                open += 1.0;
            }
        }
        if ix + 1 < w {
            let e = i * 2;
            if d.stat[e] != 0 && d.leaf[e] == 0 {
                a += d.s[i + 1] - v;
                open += 1.0;
            }
        }
        if i >= w {
            let e = (i - w) * 2 + 1;
            if d.stat[e] != 0 && d.leaf[e] == 0 {
                a += d.s[i - w] - v;
                open += 1.0;
            }
        }
        if i + w < n {
            let e = i * 2 + 1;
            if d.stat[e] != 0 && d.leaf[e] == 0 {
                a += d.s[i + w] - v;
                open += 1.0;
            }
        }
        let mut c = v + k * a - r * (v - leak);
        if m::abs(c - leak) < SNAP {
            c = leak;
        }
        let dv = m::abs(c - v);
        if dv > moved {
            moved = dv;
        }
        d.next[i] = c;
    }
    for q in 0..d.cells.len() {
        let i = d.cells[q] as usize;
        d.s[i] = d.next[i];
    }
    d.stats.steps += 1.0;
    d.stats.open = open;
    moved
}

/// The field as it stands, copied again for the renderer; no book of this module moves. The host calls this through `game_rehand` and takes `gpu` itself. (airDust.ts AirDust.resend)
pub fn air_dust_resend(g: &mut crate::game::Game) {
    if !g.air_dust.on {
        return;
    }
    copy_to_gpu(g);
}

/// The renderer's copy brought up to the field; the host takes `gpu` when `stats.sends` moves. (airDust.ts AirDust.send)
pub fn air_dust_send(g: &mut crate::game::Game) {
    copy_to_gpu(g);
    let d = &mut g.air_dust;
    d.stats.sends += 1.0;
    d.drift = 0.0;
}

/// `gpu` written from the field, a Float32Array store per cell. (airDust.ts:186, 188)
fn copy_to_gpu(g: &mut Game) {
    let n = cell_count();
    let d = &mut g.air_dust;
    for i in 0..n {
        d.gpu[i] = d.s[i] as f32;
    }
}

/// Not translated: a save is a clone of Game (conventions 6). No part calls it. (airDust.ts AirDust.snapshot)
pub fn air_dust_snapshot(g: &mut crate::game::Game) -> Option<crate::air_dust::AirDustSnap> {
    todo!("not translated: checkpoints are a clone of Game")
}

/// Not translated: a save is a clone of Game (conventions 6). No part calls it. (airDust.ts AirDust.restore)
pub fn air_dust_restore(g: &mut crate::game::Game, sn: Option<&crate::air_dust::AirDustSnap>) {
    todo!("not translated: checkpoints are a clone of Game")
}

/// One line for the debugger. (airDust.ts AirDust.summary)
pub fn air_dust_summary(g: &mut crate::game::Game) -> String {
    air_dust_ensure(g);
    if !g.air_dust.seeded {
        return String::from("no weather in the air");
    }
    let d = &g.air_dust;
    let mut mx = 0.0;
    let mut sum = 0.0;
    for &i in &d.cells {
        let v = d.s[i as usize];
        sum += v;
        if v > mx {
            mx = v;
        }
    }
    let len = d.cells.len();
    alloc::format!(
        "moving · {} roofed cells · mean share {:.3} (leak {:.3}, max {:.3}) · {} open ways",
        len,
        sum / m::max(1.0, len as f64),
        d.leak,
        mx,
        d.stats.open
    )
}

/// Does the plan segment from (x, z) along (dx, dz), t in [0, 1], a leaf's own frame, enter the rectangle |x| ≤ hx, |z| ≤ hz. (airDust.ts crosses)
pub fn crosses(x: f64, z: f64, dx: f64, dz: f64, hx: f64, hz: f64) -> bool {
    let mut t0 = 0.0;
    let mut t1 = 1.0;
    if m::abs(dx) < 1e-9 {
        if m::abs(x) > hx {
            return false;
        }
    } else {
        let mut a = (-hx - x) / dx;
        let mut b = (hx - x) / dx;
        if a > b {
            let t = a;
            a = b;
            b = t;
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
    if m::abs(dz) < 1e-9 {
        if m::abs(z) > hz {
            return false;
        }
    } else {
        let mut a = (-hz - z) / dz;
        let mut b = (hz - z) / dz;
        if a > b {
            let t = a;
            a = b;
            b = t;
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

/// The fixed step in use: AIRDUST.step held to what the explicit scheme and a frame can take. (airDust.ts stepSecs)
pub fn step_secs(g: &mut crate::game::Game) -> f64 {
    let h = g.k.airdust.step;
    if h >= 1.0 / 240.0 && h <= 0.1 {
        h
    } else {
        1.0 / 30.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The answers are the TypeScript function's own, run under bun.
    #[test]
    fn crosses_matches_the_typescript() {
        let cases: [(f64, f64, f64, f64, f64, f64, bool); 14] = [
            (0.0, 0.0, 0.5, 0.0, 0.45, 0.05, true),
            (-1.0, 0.0, 0.5, 0.0, 0.45, 0.05, false),
            (-0.6, 0.0, 0.5, 0.0, 0.45, 0.05, true),
            (0.6, 0.0, -0.5, 0.0, 0.45, 0.05, true),
            (1.0, 0.0, 0.5, 0.0, 0.45, 0.05, false),
            (0.3, 0.2, 0.0, 0.5, 0.45, 0.05, false),
            (0.3, -0.2, 0.0, 0.5, 0.45, 0.05, true),
            (0.3, -0.6, 0.0, 0.5, 0.45, 0.05, false),
            (0.5, 0.1, 0.0, 0.5, 0.45, 0.05, false),
            (-0.7, -0.3, 0.35, 0.35, 0.45, 0.05, true),
            (-0.7, 0.3, 0.35, 0.35, 0.45, 0.05, false),
            (0.45, 0.0, 0.5, 0.0, 0.45, 0.05, true),
            (-0.95, 0.0, 0.5, 0.0, 0.45, 0.05, true),
            (0.0, 0.05, 1e-10, 0.0, 0.45, 0.05, true),
        ];
        for (x, z, dx, dz, hx, hz, want) in cases {
            assert_eq!(crosses(x, z, dx, dz, hx, hz), want, "{x} {z} {dx} {dz}");
        }
    }
}
