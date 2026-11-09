//! The game's own smoke: what is let out, its steps, and the two questions asked of it.

pub use ::game_types::sense::types;
pub use types::*;
use crate::game::Game;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

/// THE GAME'S OWN SMOKE: what every eye, every lamp's light and the fog of war see through, and the picture's solver does not answer for. It is the coarse
/// grid of rust/sim's block (bricks.rs), fed by whatever the rules let out, stepped ten times a second, with the light that comes round (glow.rs) beside it.
/// All of it is the block's: nothing of it is kept here.

/// The block has a grid's part: an empty one, made as a game's first tick opens.
pub fn smoke_home(g: &mut crate::game::Game) {
    if sim::state::state_read(g.sim).smoke.live == 0 {
        sim::bricks::reset(g.sim, true);
    }
}

/// Is there smoke anywhere.
pub fn smoky(g: &crate::game::Game) -> bool {
    sim::bricks::smoky(g.sim)
}

/// HOW MUCH OF THE LIGHT ALONG a → b THE SMOKE LETS THROUGH (1 = clear air).
pub fn smoke_between(g: &crate::game::Game, a: V3, b: V3) -> f64 {
    if !smoky(g) {
        return 1.0;
    }
    sim::bricks::trans(g.sim, sim::state::stamp(g.sim), a, b, g.k.sense.smoke.sigma)
}

/// WHAT THE GAME HOLDS of a source: how wide it is and what it lets out a second. A source of the picture's alone (`look`) is nothing to the game; one too small to hide anybody is the picture's alone too, and takes no room of the budget.
fn held(g: &crate::game::Game, e: &crate::smoke_types::SmokeEmitter) -> Option<(f64, f64)> {
    let r = m::max(e.radius, 0.05);
    let rate = e.density * PI15 * r * r * r;
    if e.look != Some(true) && rate * e.ttl >= g.k.sense.smoke.least { Some((r, rate)) } else { None }
}

/// Does the game's own smoke hold what this source lets out? (The picture's is bent toward it, of such a source's alone.)
pub fn smoke_held(g: &crate::game::Game, e: &crate::smoke_types::SmokeEmitter) -> bool {
    held(g, e).is_some()
}

/// Something lets smoke out.
pub fn smoke_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) {
    let Some((r, rate)) = held(g, e) else { return };
    smoke_home(g);
    sim::bricks::emit(g.sim, e.pos[0], e.pos[1], e.pos[2], r, rate, e.ttl);
}

/// WHAT LETS SMOKE OUT NOW, each source as old as it is, for a picture of a game put back, which heard none of them begin. The game holds where one stands, how wide it
/// is, how much it lets out and for how long; how it comes out of it is a can's for all of them, as the game's own smoke rises alike from them all.
pub fn smoke_sources(g: &crate::game::Game) -> Vec<crate::smoke_types::SmokeEmitter> {
    let p = crate::combat::can_params(g);
    sim::bricks::letting(g.sim).map(|e| crate::smoke_types::SmokeEmitter {
        pos: e.at, dir: [0.0, 1.0, 0.0], speed: p.canister_speed, radius: e.radius, density: e.rate / (PI15 * e.radius * e.radius * e.radius), temperature: p.canister_temp, ttl: e.ttl, age: e.age,
        kind: crate::smoke_types::SmokeEmitterKind::Canister, held: Some(true), ..crate::smoke_types::SmokeEmitter::default()
    }).collect()
}

/// Every source lets out no more; what is in the air stays.
pub fn smoke_shut(g: &mut crate::game::Game) {
    if sim::state::state_read(g.sim).smoke.live != 0 { sim::bricks::shut(g.sim); }
}

/// How many more emitters there is room for, off the game's own count.
pub fn smoke_budget(g: &crate::game::Game) -> f64 {
    let k = g.k.sense.smoke;
    m::max(0.0, sim::bricks::MAX_EMIT as f64 - k.reserve - sim::bricks::emitting(g.sim) as f64)
}

/// `dt` seconds of the game: the steps come due are taken.
pub fn smoke_step(g: &mut crate::game::Game, dt: f64) {
    let n = sim::bricks::beat(g.sim, dt);
    if n == 0 {
        return;
    }
    let s = g.k.sense.smoke;
    let k = crate::scene_collision::GridStepK { decay: s.decay, spread: s.spread, rise: s.rise, rise_rate: s.rise_rate, gone: s.gone, in_wind: s.in_wind };
    let wind = if smoky(g) { crate::wind::wind_storm_drift(g) } else { None };
    let sweep = g.k.sense.glow.sweep;
    crate::scene_collision::grid_step(g, n as f64, k, sweep, wind.as_ref());
}

/// The smoke's ways are asked, of a game that was put back and is drawn before its next step. The game is as it was.
pub fn smoke_ways(g: &mut crate::game::Game) {
    if sim::state::state_read(g.sim).smoke.live != 0 { crate::scene_collision::grid_ways(g); }
}

/// No smoke anywhere; the beat, the plan and the light that comes round stay: they are the level's.
pub fn smoke_clear(g: &mut crate::game::Game) {
    if smoky(g) {
        sim::bricks::empty(g.sim);
    }
}

/// A new encounter: no smoke, on no plan, nothing swept. (A game no tick has opened has no smoke's part yet.)
pub fn smoke_renew(g: &mut crate::game::Game) {
    if sim::state::state_read(g.sim).smoke.live != 0 {
        sim::bricks::reset(g.sim, true);
    }
}

/// THE LIGHT UNDER A ROOF at a point that is no lamp's and not the sun's own beam (None: it is under the open
/// sky, or nothing was swept yet).
pub fn under_roof(g: &crate::game::Game, p: V3) -> Option<sim::glow::Field> {
    sim::bricks::under_roof(g.sim, g.sim, p)
}

/// THE LIGHT THAT STANDS ON A POINT, asked from outside a tick (src/view/worldask.ts `light`): a page's preview asks it a block of floor at a time. `owners`: whose carried
/// lamps are left out · `steady`: what the place lights, with nothing that passes · `leave`: lamps of the level's own left out, by what feeds them. The game is as it was
/// (`game_start/tests/senses.rs`), so no host takes its state again after it.
pub fn light_asked(g: &mut crate::game::Game, p: V3, owners: &[f64], steady: bool, leave: f64) -> f64 {
    if crate::ask::WorldAsk::try_of(g).is_err() { crate::scene_collision::stands(g); }
    crate::scene_box_mirror::box_mirror_things(g);
    crate::guards::light_at(g, p, Some(&mut |_: &mut Game, o: f64| owners.contains(&o)), Some(steady), Some(leave)).0
}

/// ASKING THE SENSES FROM OUTSIDE A TICK (src/game/sense.ts senseTool); no rule asks it. `what` 0: the light on points (`xs`: 1 for what the place lights with nothing that
/// passes, the level's lamps left out by what feeds them, how many owners follow, the owners whose carried lamps are left out, then x y z each) · 1: how much of the light along lines the smoke lets through (six numbers each) · 2: a canister's smoke let out
/// at x y z, to the game and to the picture alike · 3: the smoke cleared, the same.
pub fn sense_tool(g: &mut crate::game::Game, what: f64, xs: &[f64]) -> Vec<f64> {
    let at = |p: &[f64]| -> V3 { [p[0], p[1], p[2]] };
    match what as i32 {
        0 => {
            let [steady, leave, n, rest @ ..] = xs else { return Vec::new() };
            let (owners, points) = rest.split_at(*n as usize);
            points.chunks_exact(3).map(|p| crate::guards::light_at(g, at(p), Some(&mut |_: &mut Game, o: f64| owners.contains(&o)), Some(*steady != 0.0), Some(*leave)).0).collect()
        }
        1 => xs.chunks_exact(6).map(|l| smoke_between(g, at(l), at(&l[3..]))).collect(),
        2 if xs.len() == 3 => {
            let p = crate::combat::can_params(g);
            let e = crate::smoke_types::SmokeEmitter { pos: at(xs), dir: [0.0, 1.0, 0.0], speed: p.canister_speed, radius: p.canister_radius, density: p.canister_density, temperature: p.canister_temp, ttl: p.canister_duration, age: 0.0, kind: crate::smoke_types::SmokeEmitterKind::Canister, ..crate::smoke_types::SmokeEmitter::default() };
            crate::smoke_types::smoke_system_like_emit(g, &e);
            crate::out_events::push_smoke_system_like_emit(g, &e);
            Vec::new()
        }
        3 => {
            crate::smoke_types::smoke_system_like_clear_all(g);
            crate::out_events::push_smoke_system_like_clear_all(g);
            Vec::new()
        }
        _ => Vec::new(),
    }
}
