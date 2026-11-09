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
/// The same table, to read.
fn air_read(g: &Game) -> &'static ::sim::air::Air {
    &::sim::weather::of_read(g.sim).air
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
pub fn air_dust_on(g: &crate::game::Game) -> bool {
    air_read(g).on != 0.0
}

/// A new one, against the game as it stands; the caller stores it at `g.air_dust`. (airDust.ts AirDust.constructor)
pub fn air_dust_new(g: &mut crate::game::Game) -> crate::air_dust::AirDust {
    let (w, h) = (crate::scene_air_grid::air_w(), crate::scene_air_grid::air_h());
    let (cap_w, cap_h) = (::sim::weather_cap(2) as f64, ::sim::weather_cap(3) as f64);
    let min = crate::scene_world::world().min;
    if cap_w != w || cap_h != h || AIR_CELL != 0.5 || min[0] != 0.0 || min[2] != 0.0 {
        panic!("the dust's plan is {} × {}, and rust/sim/src/air.rs holds {} × {}", w, h, cap_w, cap_h);
    }
    air(g).s.fill(1.0); // (a block's zeros are no air at all)
    let was = core::mem::replace(&mut g.air_dust, AirDust::blank());
    if crate::weather::weather_kind(g).is_some() {
        air_dust_ensure(g);
    }
    core::mem::replace(&mut g.air_dust, was)
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
    let stamp = ::sim::state::stamp(g.sim);
    ::sim::air_begin(g.sim, stamp, if on { 1 } else { 0 });
    g.air_dust.drift = 0.0;
    let leak = match (has, fog) {
        (true, Some(f)) => f.leak,
        _ => {
            if g.air_dust.seeded {
                g.air_dust.seeded = false;
                let stamp = ::sim::state::stamp(g.sim);
                ::sim::air_clear(g.sim, stamp);
            }
            return;
        }
    };
    g.air_dust.seeded = true;
    crate::weather::weather_roofs_now(g);
    let stamp = ::sim::state::stamp(g.sim);
    ::sim::air_seed(g.sim, stamp, leak);
    g.air_dust.stats.roofed = air_read(g).n_cells;
    g.air_dust.stats.edges = air_read(g).n_edges;
    if on {
        knobs_here(g);
        crate::scene_collision::rule(g, &mut |g: &mut Game| {
            crate::scene_collision::rule_open(g);
            let (w, set, ask) = rule_args(g);
            let stamp = ::sim::state::stamp(g.sim);
            ::sim::air_settle(g.sim, stamp, w, set, ask);
            let t = said();
            air_dust_tally(g, &t);
        });
    }
    air_dust_send(g);
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
        let stamp = ::sim::state::stamp(g.sim);
        ::sim::air_update(g.sim, stamp, w, set, ask, dt);
        let t = said();
        g.air_dust.drift += t[0];
        air_dust_tally(g, &t);
    });
    if g.air_dust.drift > SEND {
        air_dust_send(g);
    }
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
    let s = &air_read(g).s;
    let o = &mut g.air_dust.gpu;
    for i in 0..n() as usize {
        o[i] = s[i] as f32;
    }
}

