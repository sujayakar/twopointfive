//! The weather's dust in the rooms: the caller of rust/sim/src/air.rs, whose table in the block holds the field, and the picture's copy. (src/game/airDust.ts)
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

const AIR_CELL: f64 = crate::scene_air_grid::AIR_CELL;

/// The dust's table in the game's block.
fn air(g: &Game) -> &'static mut ::sim::air::Air {
    &mut ::sim::weather::of(g.sim).air
}

/// The slot's tables, made if they are owed. (rust/sim/src/lib.rs air_of)
fn plan(g: &mut Game) -> &'static mut ::sim::air::Plan {
    let stamp = g.sim_stamp;
    let wx = ::sim::weather::of(g.sim);
    ::sim::air::plan(g.sim, stamp, &wx.air, &wx.roofs, wx.roofs_n as usize)
}

/// The knobs current for a question asked of the module now. WX_KNOBS.here is set by rustknobs.ts:59 and reads the knobs; the skeleton's `here()` has no g (notes, ## Skeleton).
fn knobs_here(g: &mut Game) {
    if !g.m.rustknobs.rust_knobs.tick {
        crate::rustknobs::knobs_to_rust(g);
    }
}

/// What `ruleOpen` left on the copy: its slot, which list of leaves, which pane rule. Call after `rule_open`.
fn rule_args(g: &Game) -> (usize, i32, u32) {
    let c = &g.col.mirror;
    (c.slot as usize, c.rule_set as i32, c.rule_panes as u32)
}

/// A step's tale, from the module's scratch: how far the field moved, the steps taken, the lines asked, the open ways the last step found. (airDust.ts said)
pub fn said() -> Vec<f64> {
    crate::sim_weatherhome::wx_out(4.0)
}

/// The field stands: built now if it is owed. (airDust.ts AirDust.ready)
pub fn air_dust_ready(g: &mut crate::game::Game) {
    air_dust_ensure(g);
}

/// The module's switch. (airDust.ts AirDust.on)
pub fn air_dust_on(g: &mut crate::game::Game) -> bool {
    air(g).on != 0.0
}

/// The module's leak. (airDust.ts AirDust.leak)
pub fn air_dust_leak(g: &mut crate::game::Game) -> f64 {
    air(g).leak
}

/// The roofed cells, a copy for the asker. (airDust.ts AirDust.cells)
pub fn air_dust_cells(g: &mut crate::game::Game) -> Vec<u32> {
    let n = air(g).n_cells as usize;
    plan(g).cells[..n].to_vec()
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
    air(g).s[(ix + iz * w) as usize]
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

/// A tale's count into the book. (airDust.ts AirDust.tally)
pub fn air_dust_tally(g: &mut crate::game::Game, t: &[f64]) {
    let st = &mut g.air_dust.stats;
    st.steps += t[1];
    st.probes += t[2];
    if t[1] > 0.0 {
        st.open = t[3]; // (the open ways the last step found: a tick with no step leaves them)
    }
}

/// The field from nothing, in the module: every roofed cell at the leak, every opening asked, then run on until it stands still. (airDust.ts AirDust.build)
pub fn air_dust_build(g: &mut crate::game::Game) {
    let fog = crate::weather::weather_fog(g);
    let has = crate::weather::weather_kind(g).is_some() && fog.is_some();
    let on = has; // (AIRDUST.on is settled true on the page)
    let stamp = g.sim_stamp;
    ::sim::air_begin(g.sim, stamp, if on { 1 } else { 0 });
    g.air_dust.drift = 0.0;
    let leak = match (has, fog) {
        (true, Some(f)) => f.leak,
        _ => {
            if g.air_dust.seeded {
                g.air_dust.seeded = false;
                let stamp = g.sim_stamp;
                ::sim::air_clear(g.sim, stamp);
            }
            return;
        }
    };
    g.air_dust.seeded = true;
    crate::weather::weather_roofs_now(g);
    let stamp = g.sim_stamp;
    ::sim::air_seed(g.sim, stamp, leak);
    g.air_dust.stats.roofed = air(g).n_cells;
    g.air_dust.stats.edges = air(g).n_edges;
    if on {
        knobs_here(g);
        crate::scene_collision::rule(g, &mut |g: &mut Game| {
            crate::scene_collision::rule_open(g);
            let (w, set, ask) = rule_args(g);
            let stamp = g.sim_stamp;
            ::sim::air_settle(g.sim, stamp, w, set, ask);
            let t = said();
            air_dust_tally(g, &t);
        });
    }
    air_dust_send(g);
}

/// The level as authored again. (airDust.ts AirDust.reset)
pub fn air_dust_reset(g: &mut crate::game::Game) {
    air_dust_reseed(g);
}

/// Once a frame, after the doors and whatever breaks have moved: through the openings as they stand, in the module. (airDust.ts AirDust.update)
pub fn air_dust_update(g: &mut crate::game::Game, dt: f64) {
    air_dust_ensure(g);
    if !air_dust_on(g) {
        return;
    }
    knobs_here(g);
    crate::scene_collision::rule(g, &mut |g: &mut Game| {
        crate::scene_collision::rule_open(g);
        let (w, set, ask) = rule_args(g);
        let stamp = g.sim_stamp;
        ::sim::air_update(g.sim, stamp, w, set, ask, dt);
        let t = said();
        g.air_dust.drift += t[0];
        air_dust_tally(g, &t);
    });
    if g.air_dust.drift > SEND {
        air_dust_send(g);
    }
}

/// The field as it stands, copied again for the renderer; no book of this module moves. The host calls this through `game_rehand` and takes `gpu` itself. (airDust.ts AirDust.resend)
pub fn air_dust_resend(g: &mut crate::game::Game) {
    if !air_dust_on(g) {
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

/// `gpu` written from the field, a Float32Array store per cell. (airDust.ts:137, 139)
fn copy_to_gpu(g: &mut Game) {
    let s = &air(g).s;
    let o = &mut g.air_dust.gpu;
    for i in 0..n() as usize {
        o[i] = s[i] as f32;
    }
}



/// One line for the debugger. (airDust.ts AirDust.summary)
pub fn air_dust_summary(g: &mut crate::game::Game) -> String {
    air_dust_ensure(g);
    if !g.air_dust.seeded {
        return String::from("no weather in the air");
    }
    let c = air_dust_cells(g);
    let s = &air(g).s;
    let mut mx = 0.0;
    let mut sum = 0.0;
    for &i in &c {
        let v = s[i as usize];
        sum += v;
        if v > mx {
            mx = v;
        }
    }
    let len = c.len();
    alloc::format!(
        "moving · {} roofed cells · mean share {} (leak {}, max {}) · {} open ways",
        len,
        js::to_fixed(sum / m::max(1.0, len as f64), 3),
        js::to_fixed(air_dust_leak(g), 3),
        js::to_fixed(mx, 3),
        js::num_str(g.air_dust.stats.open)
    )
}

