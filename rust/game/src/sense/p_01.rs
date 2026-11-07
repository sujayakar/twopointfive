//! src/game/sense.ts lines 1 to 261
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The grid over a block.   (sense.ts SmokeGrid.constructor)
pub fn smoke_grid_new(g: &mut crate::game::Game, sim: crate::Opaque) -> crate::sense::SmokeGrid {
    crate::sense::SmokeGrid { clock: 0.0, pending: Vec::new(), stands: false, fed: false, sim }
}

/// Where the state row starts in the block, in doubles.   (sense.ts SmokeGrid.row0)
pub fn smoke_grid_row0(g: &mut crate::game::Game) -> f64 {
    (js::to_u32(crate::sim_state::sim_part(crate::sim_state::P_SMOKE)) >> 3) as f64 + 1.0
}

/// One double of the state row, by its place in it (GRID).   (sense.ts SmokeGrid.at)
pub fn smoke_grid_at(g: &mut crate::game::Game, i: f64) -> f64 {
    sim::state::state(g.sim).smoke.st[i as usize]
}

/// The state row as far as it is in use, for a read now: a copy, never kept.   (sense.ts SmokeGrid.row)
pub fn smoke_grid_row(g: &mut crate::game::Game) -> Vec<f64> {
    let p = &sim::state::state(g.sim).smoke;
    let n = if p.live > 8 { (p.live - 8) >> 3 } else { 0 };
    p.st[..n as usize].to_vec()
}

/// Is there smoke for a step to move: an emitter not yet taken, or a brick standing.   (sense.ts SmokeGrid.smoky)
pub fn smoke_grid_smoky(g: &mut crate::game::Game) -> bool {
    !g.cpu_smoke.grid.as_ref().unwrap().pending.is_empty() || sim::state::state(g.sim).smoke.live as f64 > 8.0 + grid().rho * 8.0
}

/// An emitter is handed over for the next step; a game no step has reached keeps the latest.   (sense.ts SmokeGrid.emit)
pub fn smoke_grid_emit(g: &mut crate::game::Game, x: f64, y: f64, z: f64, radius: f64, rate: f64, ttl: f64) {
    let most = 6.0 * g.k.sense.smoke.emitters;
    let gr = g.cpu_smoke.grid.as_mut().unwrap();
    if gr.pending.len() as f64 >= most {
        let cut = if gr.pending.len() < 6 { gr.pending.len() } else { 6 };
        gr.pending.drain(0..cut);
    }
    gr.pending.extend_from_slice(&[x, y, z, radius, rate, ttl]);
    gr.fed = true;
}

/// `dt` seconds of the game: the steps come due are taken, four at most.   (sense.ts SmokeGrid.step)
pub fn smoke_grid_step(g: &mut crate::game::Game, dt: f64, lamps: bool, wind: Option<&mut dyn FnMut(&mut crate::game::Game) -> Option<crate::scene_collision::GridWind>>) {
    let gr = g.cpu_smoke.grid.as_mut().unwrap();
    gr.clock += dt;
    if gr.clock < GRID_STEP {
        return;
    }
    let mut left = gr.clock;
    let mut n = 0.0;
    while left >= GRID_STEP && n < 4.0 {
        left -= GRID_STEP;
        n += 1.0;
    }
    let s = g.k.sense.smoke;
    let k = crate::scene_collision::GridStepK { decay: s.decay, spread: s.spread, rise: s.rise, rise_rate: s.rise_rate, gone: s.gone, in_wind: s.in_wind };
    // (`lamps` says only whether: grid_step itself calls guards::steady_lamps, the one lamps there is, and takes the rows' word from its answer.)
    let glow = if lamps {
        let gk = g.k.sense.glow;
        Some(crate::scene_collision::GridStepGlow { k: crate::scene_collision::GridStepGlowK { albedo: gk.albedo, share: gk.share, passes: gk.passes, sweep: gk.sweep } })
    } else {
        None
    };
    let wind_now = match wind {
        Some(w) => {
            if smoke_grid_smoky(g) {
                w(g)
            } else {
                None
            }
        }
        None => None,
    };
    let gr = g.cpu_smoke.grid.as_mut().unwrap();
    let pending = gr.pending.clone();
    let mut st = gr.sim;
    let stands = crate::scene_collision::grid_step(g, &mut st, &pending, n, k, glow, wind_now.as_ref());
    let gr = g.cpu_smoke.grid.as_mut().unwrap();
    gr.stands = stands;
    if stands {
        gr.clock = if left >= GRID_STEP { 0.0 } else { left };
        gr.pending.clear();
    } else if gr.clock > 4.0 * GRID_STEP {
        gr.clock = 4.0 * GRID_STEP;
    }
}

/// No smoke anywhere; the steps keep their beat, the plan and the light that comes round stay.   (sense.ts SmokeGrid.clear)
pub fn smoke_grid_clear(g: &mut crate::game::Game) {
    if !g.cpu_smoke.grid.as_ref().unwrap().fed {
        return;
    }
    sim::bricks::empty(g.sim);
    let gr = g.cpu_smoke.grid.as_mut().unwrap();
    gr.pending.clear();
    gr.fed = false;
}

/// A new grid where this one stood: no smoke, on no plan, nothing swept.   (sense.ts SmokeGrid.renew)
pub fn smoke_grid_renew(g: &mut crate::game::Game) {
    sim::bricks::reset(g.sim, true);
    let gr = g.cpu_smoke.grid.as_mut().unwrap();
    gr.clock = 0.0;
    gr.pending.clear();
    gr.stands = false;
    gr.fed = false;
}

/// How much of the light along a → b comes through; −1 = the blobs answer.   (sense.ts SmokeGrid.along)
pub fn smoke_grid_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    if !g.cpu_smoke.grid.as_ref().unwrap().stands {
        return -1.0;
    }
    let stamp = g.sim_stamp;   // (`st.stamp`, sense.ts:151: the stamp of the game's block, the one the lamps' shade and the dust hand to rust/sim)
    sim::bricks::trans(g.sim, stamp, a, b, g.k.sense.smoke.sigma)
}

/// The light that comes round on a point: the nearest square metre in clear sight, else 0.   (sense.ts SmokeGrid.glowAt)
pub fn smoke_grid_glow_at(g: &mut crate::game::Game, p: V3) -> f64 {
    let gc = grid();
    let (nx, nz) = {
        let d = &sim::state::state(g.sim).smoke.st;
        (d[gc.nbx as usize] * gc.side, d[gc.nbz as usize] * gc.side)
    };
    if !(nx * nz <= gc.glow_max) {
        return 0.0;
    }
    let (ix, iz) = (m::floor(p[0]), m::floor(p[2]));
    let (ax, az) = (p[0] - ix - 0.5, p[2] - iz - 0.5);
    let (sx, sz) = (if ax < 0.0 { -1.0 } else { 1.0 }, if az < 0.0 { -1.0 } else { 1.0 });
    let x_first = m::abs(ax) >= m::abs(az);
    for k in 0..4 {
        let dx = if k == 3 || k == (if x_first { 1 } else { 2 }) { sx } else { 0.0 };
        let dz = if k == 3 || k == (if x_first { 2 } else { 1 }) { sz } else { 0.0 };
        let (jx, jz) = (ix + dx, iz + dz);
        if !(jx >= 0.0 && jz >= 0.0 && jx < nx && jz < nz) {
            continue;
        }
        if !crate::scene_collision::segment_blocked(g, p, [jx + 0.5, gc.chest, jz + 0.5], None, None, None) {
            return sim::state::state(g.sim).smoke.st[(gc.glow_b + jz * nx + jx) as usize];
        }
    }
    0.0
}

/// As a tick opens the grid comes, empty, or goes, by SENSE's switches: this file's one tenant.   (sense.ts:172-175, by state.ts SimState.settle)
pub fn settle_grid(g: &mut crate::game::Game) {
    let want = g.k.sense.cpu && g.k.sense.grid && crate::sim_sim::sim_why_not().is_empty();
    let have = g.cpu_smoke.grid.is_some();
    if want == have {
        return;
    }
    if want {
        sim::bricks::reset(g.sim, true);
        let made = smoke_grid_new(g, crate::Opaque);
        g.cpu_smoke.grid = Some(made);
    } else {
        g.cpu_smoke.grid = None;
        sim::bricks::reset(g.sim, false);
    }
}

/// What a blob holds and has still to let out.   (sense.ts:207 worth)
fn worth(b: &crate::sense::Blob) -> f64 {
    b.mass + b.rate * m::max(0.0, b.ttl - b.age)
}

/// An emitter handed to the solver: what it will let out, as one blob where it stands.   (sense.ts CpuSmoke.emit)
pub fn cpu_smoke_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) {
    if e.look == Some(true) {
        return;
    }
    let k = g.k.sense.smoke;
    let r = m::max(e.radius, 0.05);
    let rate = e.density * PI15 * r * r * r;
    if e.ttl > 0.0 {
        g.cpu_smoke.lives.push(e.ttl);
        if g.cpu_smoke.lives.len() as f64 > 2.0 * k.emitters {
            g.cpu_smoke.lives.remove(0);
        }
    }
    if !(rate * e.ttl >= k.least) {
        return;
    }
    if g.cpu_smoke.grid.is_some() {
        smoke_grid_emit(g, e.pos[0], e.pos[1], e.pos[2], r, rate, e.ttl);
    }
    if g.cpu_smoke.blobs.len() as f64 >= m::min(k.cap, crate::scene_box_mirror::face().max_blobs) {
        let blobs = &mut g.cpu_smoke.blobs;
        let mut least = 0;
        for i in 1..blobs.len() {
            if worth(&blobs[i]) < worth(&blobs[least]) {
                least = i;
            }
        }
        if worth(&blobs[least]) >= rate * e.ttl {
            return;
        }
        blobs.remove(least);
    }
    g.cpu_smoke.blobs.push(crate::sense::Blob { x: e.pos[0], y0: e.pos[1], z: e.pos[2], r2: r * r, rate, ttl: e.ttl, age: 0.0, mass: 0.0 });
}

/// How many more emitters there is room for, off the game's own count.   (sense.ts CpuSmoke.budget)
pub fn cpu_smoke_budget(g: &mut crate::game::Game) -> f64 {
    let k = g.k.sense.smoke;
    m::max(0.0, k.emitters - k.reserve - g.cpu_smoke.lives.len() as f64)
}

/// Ages the emitters' lives and the blobs by `dt`.   (sense.ts CpuSmoke.step)
pub fn cpu_smoke_step(g: &mut crate::game::Game, dt: f64) {
    let k = g.k.sense.smoke;
    let own = &mut g.cpu_smoke;
    step_lists(&k, &mut own.lives, &mut own.blobs, dt);
}

/// `step`'s statements over the two lists.   (sense.ts:217-224)
fn step_lists(k: &crate::knobs::SenseKSmoke, lives: &mut Vec<f64>, blobs: &mut Vec<crate::sense::Blob>, dt: f64) {
    let keep = m::exp(-k.decay * dt);
    let mut w = 0;
    for i in 0..lives.len() {
        let t = lives[i] - dt;
        if t > 0.0 {
            lives[w] = t;
            w += 1;
        }
    }
    lives.truncate(w);
    let mut i = blobs.len();
    while i > 0 {
        i -= 1;
        let b = &mut blobs[i];
        b.mass *= keep;
        if b.age < b.ttl {
            b.mass += b.rate * m::min(dt, b.ttl - b.age);
        }
        b.age += dt;
        if b.age >= b.ttl && b.mass < k.gone {
            blobs.remove(i);
        }
    }
}

/// No smoke, here or on the grid.   (sense.ts CpuSmoke.clear)
pub fn cpu_smoke_clear(g: &mut crate::game::Game) {
    g.cpu_smoke.blobs.clear();
    g.cpu_smoke.lives.clear();
    if g.cpu_smoke.grid.is_some() {
        smoke_grid_clear(g);
    }
}

/// The smoke along a sight line in the game's own box world: the grid first, then the blobs.   (sense.ts CpuSmoke.along)
pub fn cpu_smoke_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    if g.cpu_smoke.grid.is_some() {
        let t = smoke_grid_along(g, a, b);
        if t != -1.0 {
            return t;
        }
    }
    cpu_smoke_blobs_along(g, a, b)
}

/// By the blobs, whatever a grid would say; asked between ticks, the whole question stands in one window over the copy.   (sense.ts CpuSmoke.blobsAlong)
pub fn cpu_smoke_blobs_along(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    if g.cpu_smoke.blobs.is_empty() {
        return 1.0;
    }
    if g.col.mirror.live {
        cpu_smoke_blobs_ask(g, a, b)
    } else {
        crate::scene_collision::window(g, &mut |g: &mut crate::game::Game| cpu_smoke_blobs_ask(g, a, b))
    }
}

/// The module's sum over these blobs, written there for the one question.   (sense.ts CpuSmoke.blobsAsk)
pub fn cpu_smoke_blobs_ask(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let n = g.cpu_smoke.blobs.len();
    let k = g.k.sense.smoke;
    crate::scene_collision::blobs_begin(g, n as f64);
    for i in 0..n {
        let bl = g.cpu_smoke.blobs[i];
        crate::scene_collision::blob(g, bl.x, bl.y0, bl.z, bl.r2, bl.age, bl.mass);
    }
    crate::scene_collision::blobs_along(g, a, b, k.sigma, k.spread, k.widest, k.rise, k.rise_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    /// SENSE.smoke as the TypeScript's table has it (the bits below were printed under it, EXACT on).
    fn k() -> crate::knobs::SenseKSmoke {
        crate::knobs::SenseKSmoke { sigma: 6.0, decay: 0.14, spread: 0.2, widest: 3.5, rise: 1.0, rise_rate: 0.3, in_wind: 0.15, least: 0.05, gone: 0.02, cap: 48.0, emitters: 64.0, reserve: 12.0 }
    }
    fn blob(x: f64, y0: f64, z: f64, r2: f64, rate: f64, ttl: f64) -> Blob {
        Blob { x, y0, z, r2, rate, ttl, age: 0.0, mass: 0.0 }
    }
    /// The same fixture as the TypeScript's: three blobs and three lives, two steps of a quarter second.
    fn stepped() -> (Vec<f64>, Vec<Blob>) {
        let mut blobs = vec![blob(2.5, 0.4, -1.25, 0.36, 3.3, 12.0), blob(7.0, 1.0, 2.0, 1.1, 0.7, 0.3), blob(-3.0, 0.2, 0.5, 0.09, 0.0001, 0.01)];
        let mut lives = vec![0.5, 0.01, 9.0];
        step_lists(&k(), &mut lives, &mut blobs, 0.25);
        step_lists(&k(), &mut lives, &mut blobs, 0.25);
        (lives, blobs)
    }

    #[test]
    fn step_matches_the_typescripts_bits() {
        let (lives, blobs) = stepped();
        assert_eq!(lives.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), vec![0x4021000000000000]);
        assert_eq!(blobs.len(), 2);
        assert_eq!(blobs[0].age.to_bits(), 0x3fe0000000000000);
        assert_eq!(blobs[0].mass.to_bits(), 0x3ff9f22c7fa399c2);
        assert_eq!(blobs[1].age.to_bits(), 0x3fe0000000000000);
        assert_eq!(blobs[1].mass.to_bits(), 0x3fca1c0c360036e2);
    }

    /// The TypeScript's own run: a look-only source, two too small to be blobs, seventy that overflow the cap, and one worth less than every blob.
    #[test]
    fn emit_matches_the_typescript() {
        let mut g = crate::game::Game::blank();
        g.k.sense.smoke = k();
        let e = |x: f64, radius: f64, density: f64, ttl: f64, look: Option<bool>| crate::smoke_types::SmokeEmitter { pos: [x, 0.5, -x], radius, density, ttl, look, ..crate::smoke_types::SmokeEmitter::zeroed() };
        cpu_smoke_emit(&mut g, &e(1.0, 0.3, 2.0, 5.0, Some(true)));
        cpu_smoke_emit(&mut g, &e(2.0, 0.01, 1.0, 0.001, None));
        cpu_smoke_emit(&mut g, &e(3.0, 0.02, 2.0, 1.0, None));
        for i in 0..70 {
            cpu_smoke_emit(&mut g, &e(10.0 + i as f64, 0.4 + i as f64 * 0.01, 1.0, 3.0, None));
        }
        cpu_smoke_emit(&mut g, &e(99.0, 0.4, 0.001, 3.0, None));
        assert_eq!((g.cpu_smoke.blobs.len(), g.cpu_smoke.lives.len()), (48, 73));
        assert_eq!(cpu_smoke_budget(&mut g), 0.0);
        for (i, b) in g.cpu_smoke.blobs.iter().enumerate() {
            assert_eq!(b.x, 32.0 + i as f64);
        }
        assert_eq!(g.cpu_smoke.blobs[0].r2.to_bits(), 0x3fd89a027525460b);
        assert_eq!(g.cpu_smoke.blobs[0].rate.to_bits(), 0x3ff53bc11fe49e6b);
        assert_eq!(g.cpu_smoke.blobs[47].rate.to_bits(), 0x401cd836b8216143);
    }

    /// The pending list keeps the latest, and a step not yet due takes nothing and asks nobody.
    #[test]
    fn the_grid_keeps_the_latest_and_waits_for_its_beat() {
        let mut g = crate::game::Game::blank();
        g.k.sense.smoke = k();
        g.cpu_smoke.grid = Some(smoke_grid_new(&mut g, crate::Opaque));
        for i in 0..65 {
            smoke_grid_emit(&mut g, i as f64, 0.0, 0.0, 1.0, 1.0, 1.0);
        }
        let gr = g.cpu_smoke.grid.as_ref().unwrap();
        assert_eq!((gr.pending.len(), gr.pending[0], gr.fed), (384, 1.0, true));
        smoke_grid_step(&mut g, 0.05, false, None);
        let gr = g.cpu_smoke.grid.as_ref().unwrap();
        assert_eq!((gr.clock, gr.pending.len(), gr.stands), (0.05, 384, false));
    }

    #[test]
    fn the_table_is_rust_sims_layout() {
        use sim::bricks as b;
        let c = grid();
        assert_eq!(c.step, b::SUB);
        assert_eq!(c.head as usize, b::HEAD);
        assert_eq!(c.emitters as usize, b::MAX_EMIT);
        assert_eq!(c.emit as usize, b::EMIT);
        assert_eq!(c.bricks as usize, b::MAX_BRICKS);
        assert_eq!(c.cells as usize, b::CELLS);
        assert_eq!(c.vol, b::CELL * b::CELL * b::CELL);
        assert_eq!((c.out as usize, c.given as usize, c.lost as usize), (b::H_OUT, b::H_GIVEN, b::H_LOST));
        assert_eq!(c.side, b::SIDE);
        assert_eq!(c.glow_max as usize, b::MAX_GLOW);
        assert_eq!((c.glow_d as usize, c.glow_b as usize, c.glow_x as usize, c.glow_z as usize), (b::O_GLOW_D, b::O_GLOW_B, b::O_GLOW_X, b::O_GLOW_Z));
        assert_eq!((c.glow_lamps as usize, c.rho as usize), (b::O_GLOW_LAMPS, b::O_RHO));
    }
}
