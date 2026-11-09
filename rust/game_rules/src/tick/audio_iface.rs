#![allow(unused_variables)]
pub use ::game_types::audio_iface::types;
pub use types::*;
use crate::game::Game;
use crate::v3::{self, Quat, V3};
use alloc::string::String;

/// (src/game/audio_iface.ts:6 GameAudioLike.play)
pub fn game_audio_like_play(g: &mut crate::game::Game, name: &str, pos: Option<V3>, gain: Option<f64>, opts: Option<&crate::audio_iface::PlayOpts>) { g.out.push(crate::out_events::OutEvent::GameAudioLikePlay(crate::out_events::OutGameAudioLikePlay { name: String::from(name), pos: pos, gain: gain, opts: opts.map(crate::out_events::play_opts) })); }

/// (src/game/audio_iface.ts:9 GameAudioLike.footstep)
pub fn game_audio_like_footstep(g: &mut crate::game::Game, pos: V3, loudness: f64, is_player: bool, mat: crate::floor::FloorMat) { g.out.push(crate::out_events::OutEvent::GameAudioLikeFootstep(crate::out_events::OutGameAudioLikeFootstep { pos: pos, loudness: loudness, is_player: is_player, mat: mat })); }

/// (src/game/audio_iface.ts:11 GameAudioLike.rain)
pub fn game_audio_like_rain(g: &mut crate::game::Game, level: f64) { g.out.push(crate::out_events::OutEvent::GameAudioLikeRain(crate::out_events::OutGameAudioLikeRain { level: level })); }
