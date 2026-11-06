//! THE BRING-UP'S OWN PART of render_particles: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:409 Particles.setImp)
pub fn particles_set_imp(g: &mut crate::game::Game, r: usize, pos: V3, radius: f64, strength: f64, up: f64, b: V3, e: V3, kinds: f64, flags: f64, seed: f64) {}


/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:421 Particles.setWind)
pub fn particles_set_wind(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource], knobs: [f64; 3]) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:423 Particles.clear)
pub fn particles_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/particles.ts:425 Particles.clearRain)
pub fn particles_clear_rain(g: &mut crate::game::Game) {}



/// THE POOL'S EMIT, the game's half: the record goes to the host's pool as an out event, as the generated boundary sends it (one with nothing to
/// throw is not sent: the TypeScript returns before it queues). Two things of the TypeScript's that the event does not carry are kept here:
///  · THE ANSWER. A rule reads how many were accepted (blastLook's `first`, the panel's sums): the count the pool takes while its queue has room
///    (floored, at most a frame's births; the rain's own cap for a rain record). Whether its queue was full is the host's to know: taken as not.
///  · `ours` (fogofwar.ts FOG.ours: "what is filed now is the team's own doing", up between the fog's `begin` and `done`; a rain record has none).
///    STAND-IN: the event has no such field, so the mark of each record of this step is kept, in the events' order, on this side's copy of the pool's
///    queue (`Particles.queue`, which holds nothing else here), for a host to read with the step's events and empty (`face_emits_ours`). Belongs in
///    the event (`OutParticlesEmit.ours`, asked of the generator).
/// (src/render/particles.ts:363 Particles.emit)
pub fn particles_emit(g: &mut crate::game::Game, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 {
    use crate::render_particles::{EmitRec, PKind, MAX_NEW_PER_FRAME, RAIN_MAX_BIRTHS};
    let n = m::max(0.0, m::floor(count));
    if n == 0.0 { return 0.0; }
    let rain = kind == PKind::Rain;
    let n = m::min(n, if rain { RAIN_MAX_BIRTHS } else { MAX_NEW_PER_FRAME });
    if let Some(p) = g.engine.particles.as_mut() {
        if p.queue.len() >= 8192 { p.queue.clear(); }   // (no host has read the marks for a long while: they are dropped, not kept for ever)
        let ours = !rain && p.ours;
        p.queue.push(EmitRec { kind, pos, dir, count: n, o: crate::render_particles::EmitOpts::default(), seed: 0.0, ours, waited: 0.0 });
    }
    g.out.push(crate::out_events::OutEvent::ParticlesEmit(crate::out_events::OutParticlesEmit { kind, pos, dir, count, o }));
    n
}
