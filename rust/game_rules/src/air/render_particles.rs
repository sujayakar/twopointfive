#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::render_particles::types;
pub use types::*;
use crate::game::Game;
use crate::m;
use crate::v3::{self, Quat, V3};

pub fn particles_impulse(g: &mut crate::game::Game, pos: V3, radius: f64, strength: f64, o: Option<&crate::render_particles::ImpulseOpts>) -> bool { g.out.push(crate::out_events::OutEvent::ParticlesImpulse(crate::out_events::OutParticlesImpulse { pos: pos, radius: radius, strength: strength, o: o.cloned() })); false }

pub(super) fn particles_set_heat(g: &mut crate::game::Game, srcs: &[crate::smoke_types::WindGpuSource]) { g.out.push(crate::out_events::OutEvent::ParticlesSetHeat(crate::out_events::OutParticlesSetHeat { srcs: srcs.to_vec() })); }

/// THE POOL'S EMIT, the game's half: the record goes to the host's pool as an out event (one with nothing to throw is not sent). What the event
/// says beside the record:
///  · THE ANSWER. A rule reads how many were accepted (blast_look's `first`, the panel's sums): the count the pool takes while its queue has room
///    (floored, at most a frame's births; the rain's own cap for a rain record). Whether its queue was full is the host's to know: taken as not.
///  · `ours` (the fog's `filing_ours`: what is filed now is the team's own doing, up between the fog's `begin` and `done`; a rain record has none): on the event.
pub fn particles_emit(g: &mut crate::game::Game, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 {
    use crate::render_particles::{PKind, MAX_NEW_PER_FRAME, RAIN_MAX_BIRTHS};
    let n = m::max(0.0, m::floor(count));
    if n == 0.0 { return 0.0; }
    let rain = kind == PKind::Rain;
    let n = m::min(n, if rain { RAIN_MAX_BIRTHS } else { MAX_NEW_PER_FRAME });
    let ours = !rain && g.fog_of_war.filing_ours;
    crate::out_events::push_particles_emit(g, kind, pos, dir, count, o, ours);
    n
}
