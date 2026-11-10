//! The weather's dust in the rooms: the caller of rust/sim/src/air.rs, whose table in the block holds the field, and the picture's copy.

pub use ::game_types::air_dust::types;
pub use types::*;
use crate::game::Game;

const AIR_CELL: f64 = crate::scene_air_grid::AIR_CELL;

/// The dust's table in the game's block.
fn air(g: &Game) -> &'static mut ::sim::air::Air {
    &mut ::sim::weather::of(g.sim).air
}
/// The same table, to read.
fn air_read(g: &Game) -> &'static ::sim::air::Air {
    &::sim::weather::of_read(g.sim).air
}

/// The field stands: built now if it is owed.
pub(super) fn air_dust_ready(g: &mut crate::game::Game) {
    air_dust_ensure(g);
}

/// The module's switch.
fn air_dust_on(g: &crate::game::Game) -> bool {
    air_read(g).on != 0.0
}

/// A new one, against the game as it stands; the caller stores it at `g.air_dust`.
pub fn air_dust_new(g: &mut crate::game::Game) -> crate::air_dust::AirDust {
    let (w, h) = (crate::scene_air_grid::air_w(), crate::scene_air_grid::air_h());
    let (cap_w, cap_h) = (::sim::air::AIR_W as f64, ::sim::air::AIR_H as f64);
    let min = crate::scene_world::world().min;
    if cap_w != w || cap_h != h || AIR_CELL != 0.5 || min[0] != 0.0 || min[2] != 0.0 {
        panic!("the dust's plan is {} × {}, and rust/sim/src/air.rs holds {} × {}", w, h, cap_w, cap_h);
    }
    air(g).s.fill(1.0); // (a block's zeros are no air at all)
    let was = core::mem::replace(&mut g.air_dust, AirDust::blank());
    if g.weather.kind_now.is_some() {
        air_dust_ensure(g);
    }
    core::mem::replace(&mut g.air_dust, was)
}

/// The roofs or the weather changed: marks the field stale, the first reader builds it.
pub fn air_dust_reseed(g: &mut crate::game::Game) {
    g.air_dust.dirty = true;
}

/// Builds the field if it is stale.
fn air_dust_ensure(g: &mut crate::game::Game) {
    if g.air_dust.dirty {
        g.air_dust.dirty = false;
        air_dust_build(g);
    }
}

/// The field from nothing, in the module: every roofed cell at the leak, every opening asked, then run on until it stands still.
fn air_dust_build(g: &mut crate::game::Game) {
    let fog = g.weather.fog_spec;
    let has = g.weather.kind_now.is_some() && fog.is_some();
    let on = has; // (AIRDUST.on is settled true on the page)
    let stamp = ::sim::state::stamp(g.sim);
    ::sim::air_begin(g.sim, stamp, on, &g.weather.roof_rects);
    g.air_dust.drift = 0.0;
    let leak = match (has, fog) {
        (true, Some(f)) => f.leak,
        _ => {
            if g.air_dust.seeded {
                g.air_dust.seeded = false;
                let stamp = ::sim::state::stamp(g.sim);
                ::sim::air_clear(g.sim, stamp, &g.weather.roof_rects);
            }
            return;
        }
    };
    g.air_dust.seeded = true;
    crate::weather::weather_roofs_now(g);
    let stamp = ::sim::state::stamp(g.sim);
    ::sim::air_seed(g.sim, stamp, leak, &g.weather.roof_rects);
    g.air_dust.stats.roofed = air_read(g).n_cells;
    g.air_dust.stats.edges = air_read(g).n_edges;
    if on {
        ::sim::air_settle(g.sim, ::sim::state::stamp(g.sim), g.tab.boxes.world(), &g.weather.roof_rects, &g.k);
    }
    air_dust_send(g);
}

/// Once a frame, after the doors and whatever breaks have moved: through the openings as they stand, in the module.
pub fn air_dust_update(g: &mut crate::game::Game, dt: f64) {
    air_dust_ensure(g);
    if !air_dust_on(g) {
        return;
    }
    let t = ::sim::air_update(g.sim, ::sim::state::stamp(g.sim), g.tab.boxes.world(), dt, &g.weather.roof_rects, &g.k);
    g.air_dust.drift += t.moved;
    if g.air_dust.drift > SEND {
        air_dust_send(g);
    }
}

/// The renderer's copy brought up to the field; the host takes `gpu` when `stats.sends` moves.
fn air_dust_send(g: &mut crate::game::Game) {
    copy_to_gpu(g);
    let d = &mut g.air_dust;
    d.drift = 0.0;
}

/// `gpu` written from the field, a Float32Array store per cell.
fn copy_to_gpu(g: &mut Game) {
    let s = &air_read(g).s;
    let o = &mut g.air_dust.gpu;
    for i in 0..n() as usize {
        o[i] = s[i] as f32;
    }
}
