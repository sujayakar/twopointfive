#![allow(unused_variables)]
pub use ::game_types::out_events::types;
pub use types::*;
use crate::game::Game;
use crate::v3::{self, Quat, V3};

/// (Particles.emit: its body does something of its own, so a part calls this beside it)
pub fn push_particles_emit(g: &mut crate::game::Game, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts, ours: bool) { g.out.push(crate::out_events::OutEvent::ParticlesEmit(crate::out_events::OutParticlesEmit { kind: kind, pos: pos, dir: dir, count: count, o: o, ours: ours })); }
/// (SmokeSystemLike.clearAll: its body does something of its own, so a part calls this beside it)
pub fn push_smoke_system_like_clear_all(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeClearAll); }
/// the host's cans venting stop now
pub fn smoke_cans_shut(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::SmokeCansShut); }
/// (SmokeSystemLike.emit: its body does something of its own, so a part calls this beside it)
pub fn push_smoke_system_like_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeEmit(crate::out_events::OutSmokeSystemLikeEmit { e: crate::smoke_types::SmokeEmitter { held: Some(crate::sense::smoke_held(g, e)), ..e.clone() } })); }

pub(super) fn play_opts(o: &crate::audio_iface::PlayOpts) -> crate::out_events::OutPlayOpts { use crate::audio_iface::PlayOpts as P; let mut x = crate::out_events::OutPlayOpts::default(); match o { P::Rate { rate } => x.rate = Some(*rate), P::Thunder { lowpass, rate, pan, wet } => { x.lowpass = Some(*lowpass); x.rate = Some(*rate); x.pan = Some(*pan); x.wet = Some(*wet); } } x }
