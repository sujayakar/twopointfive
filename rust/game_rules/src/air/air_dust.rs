//! The weather's dust in the rooms (`g.air_dust`, stepped by rust/sim air.rs): when it is built, and for what weather.

pub use ::game_types::air_dust::types;
pub use types::*;
use crate::game::Game;

/// The field stands: built now if the roofs or the weather have changed since.
pub(super) fn air_dust_ready(g: &mut Game) {
    if core::mem::take(&mut g.air_dust.dirty) {
        air_dust_build(g);
    }
}

/// What of the weather's dust stands at each half metre of the plan (row-major, x first; the open air's own density is 1). A reader's one question: the men's eyes read the same field.
pub fn air_grid(g: &Game) -> &[f64] {
    if g.air_dust.on { &g.air_dust.share } else { &[] }
}

/// A new one, against the game as it stands.
pub fn air_dust_new(g: &mut Game) {
    g.air_dust = AirDust::blank();
    if g.weather.kind_now.is_some() {
        air_dust_ready(g);
    }
}

/// The roofs or the weather changed: marks the field stale, the first reader builds it.
pub fn air_dust_reseed(g: &mut Game) {
    g.air_dust.dirty = true;
}

/// The field from nothing: every roofed cell at the leak, every opening asked, then run on until it stands still.
fn air_dust_build(g: &mut Game) {
    let leak = g.weather.kind_now.and(g.weather.fog_spec).map(|f| f.leak);
    ::sim::air::begin(&mut g.air_dust, leak.is_some());
    let Some(leak) = leak else { return ::sim::air::clear(&mut g.air_dust) };
    crate::weather::weather_roofs_now(g);
    ::sim::air::seed(&mut g.air_dust, leak, |x, z| ::sim::weather::under_any(&g.weather.roof_rects, x, z));
    ::sim::air::settle(&mut g.air_dust, g.tab.boxes.world(), &g.k);
}

/// Once a frame, after the doors and whatever breaks have moved: through the openings as they stand.
pub fn air_dust_update(g: &mut Game, dt: f64) {
    air_dust_ready(g);
    if g.air_dust.on {
        ::sim::air::update(&mut g.air_dust, g.tab.boxes.world(), dt, &g.k);
    }
}
