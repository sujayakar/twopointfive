#![allow(unused_variables)]
pub use ::game_types::audio_iface::types;
pub use types::*;
use crate::game::Game;
use crate::v3::{self, Quat, V3};
use alloc::string::String;

/// a sound at a place, as loud as this
pub fn play_at(g: &mut crate::game::Game, name: Sfx, at: V3, gain: f64) { game_audio_like_play(g, name, Some(at), Some(gain), None); }
/// …pitched by `rate`
pub fn play_at_rate(g: &mut crate::game::Game, name: Sfx, at: V3, gain: f64, rate: f64) { game_audio_like_play(g, name, Some(at), Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate })); }
pub fn game_audio_like_play(g: &mut crate::game::Game, name: Sfx, pos: Option<V3>, gain: Option<f64>, opts: Option<&crate::audio_iface::PlayOpts>) { g.out.push(crate::out_events::OutEvent::GameAudioLikePlay(crate::out_events::OutGameAudioLikePlay { name, pos: pos, gain: gain, opts: opts.map(crate::out_events::play_opts) })); }

pub fn game_audio_like_footstep(g: &mut crate::game::Game, pos: V3, loudness: f64, is_player: bool, mat: crate::floor::FloorMat) { g.out.push(crate::out_events::OutEvent::GameAudioLikeFootstep(crate::out_events::OutGameAudioLikeFootstep { pos: pos, loudness: loudness, is_player: is_player, mat: mat })); }

pub fn game_audio_like_rain(g: &mut crate::game::Game, level: f64) { g.out.push(crate::out_events::OutEvent::GameAudioLikeRain(crate::out_events::OutGameAudioLikeRain { level: level })); }
