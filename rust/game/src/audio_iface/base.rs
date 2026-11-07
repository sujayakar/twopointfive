//! What the parts of `audio_iface` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/audio_iface.ts:6 GameAudioLike.play)
pub fn game_audio_like_play(g: &mut crate::game::Game, name: &str, pos: Option<V3>, gain: Option<f64>, opts: Option<&crate::audio_iface::PlayOpts>) { g.out.push(crate::out_events::OutEvent::GameAudioLikePlay(crate::out_events::OutGameAudioLikePlay { name: String::from(name), pos: pos, gain: gain, opts: opts.map(crate::out_events::play_opts) })); }

/// (src/game/audio_iface.ts:9 GameAudioLike.footstep)
pub fn game_audio_like_footstep(g: &mut crate::game::Game, pos: V3, loudness: f64, is_player: bool, mat: Option<crate::floor::FloorMat>) { g.out.push(crate::out_events::OutEvent::GameAudioLikeFootstep(crate::out_events::OutGameAudioLikeFootstep { pos: pos, loudness: loudness, is_player: is_player, mat: mat })); }

/// (src/game/audio_iface.ts:11 GameAudioLike.rain)
pub fn game_audio_like_rain(g: &mut crate::game::Game, level: f64) { g.out.push(crate::out_events::OutEvent::GameAudioLikeRain(crate::out_events::OutGameAudioLikeRain { level: level })); }



pub mod null_audio {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/audio_iface.ts:13 nullAudio.play)
    pub fn play() { todo!() }

    /// (src/game/audio_iface.ts:13 nullAudio.footstep)
    pub fn footstep() { todo!() }

}
